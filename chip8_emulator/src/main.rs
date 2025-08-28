use lib_chip8::{Chip8, timers::Timer};
use pixels::{Pixels, SurfaceTexture};
use rodio::{
    Sink,
    source::{SineWave, Source},
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    error::EventLoopError,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const SCALE: usize = 20;
const CPU_HZ: u64 = 700;
const TIMER_HZ: u64 = 60;
const CPU_TICK_DURATION: Duration = Duration::from_nanos(1_000_000_000 / CPU_HZ);
const TIMER_TICK_DURATION: Duration = Duration::from_nanos(1_000_000_000 / TIMER_HZ);

struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    chip8: Arc<Mutex<Chip8>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("CHIP-8 Emulator")
                        .with_inner_size(LogicalSize::new(
                            DISPLAY_WIDTH as f64 * SCALE as f64,
                            DISPLAY_HEIGHT as f64 * SCALE as f64,
                        )),
                )
                .unwrap(),
        );

        let pixels = Pixels::new(
            DISPLAY_WIDTH as u32,
            DISPLAY_HEIGHT as u32,
            SurfaceTexture::new(
                (DISPLAY_WIDTH * SCALE) as u32,
                (DISPLAY_HEIGHT * SCALE) as u32,
                Arc::clone(&window),
            ),
        )
        .unwrap();

        self.pixels = Some(pixels);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key,
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(chip8_key) = map_keycode_to_chip8(physical_key) {
                    let mut chip8 = self.chip8.lock().unwrap();
                    chip8
                        .keyboard
                        .set_key(chip8_key, state == ElementState::Pressed);
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    // Display pixels on screen
                    let chip8 = self.chip8.lock().unwrap();
                    let display = chip8.display.dump();
                    let frame = pixels.frame_mut();

                    for (i, pixel_on) in display.iter().enumerate() {
                        let color = if *pixel_on {
                            [0xff, 0xff, 0xff, 0xff] // white
                        } else {
                            [0x00, 0x00, 0x00, 0xff] // black
                        };
                        frame[i * 4..(i + 1) * 4].copy_from_slice(&color);
                    }
                    drop(chip8);
                    pixels.render().unwrap();

                    // Request the next frame ~16ms later
                    let next_frame = Instant::now() + Duration::from_secs_f64(1.0 / 60.0);
                    event_loop.set_control_flow(ControlFlow::WaitUntil(next_frame));
                }
            }
            _ => {}
        }
    }
}

fn play_beep(sink: &Sink) {
    if sink.empty() {
        let source = SineWave::new(440.0).amplify(0.20).repeat_infinite();
        sink.append(source);
    } else {
        sink.play();
    }
}

fn map_keycode_to_chip8(keycode: PhysicalKey) -> Option<u8> {
    match keycode {
        PhysicalKey::Code(KeyCode::Digit1) => Some(0x1),
        PhysicalKey::Code(KeyCode::Digit2) => Some(0x2),
        PhysicalKey::Code(KeyCode::Digit3) => Some(0x3),
        PhysicalKey::Code(KeyCode::Digit4) => Some(0xc),
        PhysicalKey::Code(KeyCode::KeyQ) => Some(0x4),
        PhysicalKey::Code(KeyCode::KeyW) => Some(0x5),
        PhysicalKey::Code(KeyCode::KeyE) => Some(0x6),
        PhysicalKey::Code(KeyCode::KeyR) => Some(0xd),
        PhysicalKey::Code(KeyCode::KeyA) => Some(0x7),
        PhysicalKey::Code(KeyCode::KeyS) => Some(0x8),
        PhysicalKey::Code(KeyCode::KeyD) => Some(0x9),
        PhysicalKey::Code(KeyCode::KeyF) => Some(0xe),
        PhysicalKey::Code(KeyCode::KeyZ) => Some(0xa),
        PhysicalKey::Code(KeyCode::KeyX) => Some(0x0),
        PhysicalKey::Code(KeyCode::KeyC) => Some(0xb),
        PhysicalKey::Code(KeyCode::KeyV) => Some(0xf),
        _ => None,
    }
}

fn main() -> Result<(), EventLoopError> {
    // Try to load ROM
    let rom_path = std::env::args().nth(1).expect("Provide ROM file path");
    let rom_bytes: Vec<u8> = std::fs::read(&rom_path)
        .unwrap_or_else(|e| panic!("Failed to read ROM from path: '{}': {}", rom_path, e));

    // Construct the chip
    let mut chip = Chip8::new();
    chip.load_rom(&rom_bytes).unwrap_or_else(|e| {
        panic!("Failed to load rom: {}", e);
    });

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now()));

    let chip8 = Arc::new(Mutex::new(chip));
    let chip8_thread = Arc::clone(&chip8);

    let mut app = App {
        window: None,
        pixels: None,
        chip8: chip8,
    };

    std::thread::spawn(move || {
        let now = Instant::now();
        let mut last_cpu_tick = now;
        let mut last_timer_tick = now;

        // Set up audio
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        loop {
            let mut chip8 = chip8_thread.lock().unwrap();
            let now = Instant::now();

            if now.duration_since(last_cpu_tick) >= CPU_TICK_DURATION {
                chip8.tick();
                last_cpu_tick += CPU_TICK_DURATION;
            }

            if now.duration_since(last_timer_tick) >= TIMER_TICK_DURATION {
                chip8.timers.tick();
                last_timer_tick += TIMER_TICK_DURATION;
            }

            // Play sound if the sound timer is not 0
            if chip8.timers.get(Timer::Sound) > 0 {
                play_beep(&sink);
            } else {
                sink.stop();
            }

            std::thread::sleep(Duration::from_micros(100));
        }
    });

    event_loop.run_app(&mut app)
}
