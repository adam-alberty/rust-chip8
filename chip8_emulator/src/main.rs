use lib_chip8::{Chip8, timers::Timer};
use pixels::{Pixels, SurfaceTexture};
use rodio::{
    Sink,
    source::{SineWave, Source},
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

const SCALE: usize = 20;
const CPU_HZ: u64 = 700;
const TIMER_HZ: u64 = 60;
const CPU_TICK_DURATION: Duration = Duration::from_nanos(1_000_000_000 / CPU_HZ);
const TIMER_TICK_DURATION: Duration = Duration::from_nanos(1_000_000_000 / TIMER_HZ);

struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    chip8: Chip8,
    audio_sink: Sink,
    last_timer_tick: Instant,
    last_cpu_tick: Instant,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (width, height) = self.chip8.display.get_resolution();

        // Create a window
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("CHIP-8 Emulator")
                        .with_inner_size(LogicalSize::new(
                            width as f64 * SCALE as f64,
                            height as f64 * SCALE as f64,
                        )),
                )
                .unwrap(),
        );

        let pixels = Pixels::new(
            (width * SCALE) as u32,
            (height * SCALE) as u32,
            SurfaceTexture::new(
                (width * SCALE) as u32,
                (height * SCALE) as u32,
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
                println!("Window close requested");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels) {
                    let now = Instant::now();

                    while now.duration_since(self.last_cpu_tick) >= CPU_TICK_DURATION {
                        self.chip8.tick();
                        self.last_cpu_tick += CPU_TICK_DURATION;
                    }

                    while now.duration_since(self.last_timer_tick) >= TIMER_TICK_DURATION {
                        self.chip8.timers.tick();
                        self.last_timer_tick += TIMER_TICK_DURATION;
                    }

                    // Play sound if the sound timer is not 0
                    if self.chip8.timers.get(Timer::Sound) > 0 {
                        play_beep(&self.audio_sink);
                    } else {
                        self.audio_sink.stop();
                    }

                    // Display pixels on screen
                    let display = self.chip8.display.dump();
                    let frame = pixels.frame_mut();
                    let (width, height) = self.chip8.display.get_resolution();

                    for y in 0..height {
                        for x in 0..width {
                            let pixel_on = display[y * width + x];
                            let color = if pixel_on {
                                [0xff, 0xff, 0xff, 0xff]
                            } else {
                                [0x00, 0x00, 0x00, 0x00]
                            };

                            for dy in 0..SCALE {
                                for dx in 0..SCALE {
                                    let px = x * SCALE + dx;
                                    let py = y * SCALE + dy;
                                    let idx = (py * width * SCALE + px) * 4;
                                    frame[idx..idx + 4].copy_from_slice(&color);
                                }
                            }
                        }
                    }
                    pixels.render().unwrap();
                    window.request_redraw();
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
    event_loop.set_control_flow(ControlFlow::Poll);

    // Set up audio
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    let now = Instant::now();
    let mut app = App {
        window: None,
        pixels: None,
        chip8: chip,
        last_timer_tick: now,
        last_cpu_tick: now,
        audio_sink: sink,
    };

    event_loop.run_app(&mut app)
}
