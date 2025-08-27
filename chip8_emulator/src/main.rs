use lib_chip8::Chip8;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

struct App<'a> {
    window: Option<Window>,
    pixels: Option<Pixels<'a>>,
    chip8: Chip8,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (width, height) = self.chip8.display.get_resolution();

        // Create a window
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("CHIP-8 Emulator")
                        .with_inner_size(LogicalSize::new(
                            width as f64 * 10.0,
                            height as f64 * 10.0,
                        )),
                )
                .unwrap(),
        );

        let pixels = Pixels::new(
            width as u32,
            height as u32,
            SurfaceTexture::new(width as u32, height as u32, self.window.as_ref().unwrap()),
        )
        .unwrap();

        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels) {
                    let (width, height) = self.chip8.display.get_resolution();

                    // execute chip8 instruction
                    self.chip8.tick();

                    // Create the pixel buffer
                    let frame = pixels.frame_mut();

                    let display = self.chip8.display.dump();

                    for (i, pixel_on) in display.iter().enumerate() {
                        let base = i * 4;
                        if *pixel_on {
                            frame[base] = 0xff;
                            frame[base + 1] = 0xff;
                            frame[base + 2] = 0xff;
                            frame[base + 3] = 0xff;
                        } else {
                            frame[base] = 0x00;
                            frame[base + 1] = 0x00;
                            frame[base + 2] = 0x00;
                            frame[base + 3] = 0x00;
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
fn main() -> Result<(), EventLoopError> {
    // Try to load ROM
    let rom_path = std::env::args().nth(1).expect("Provide ROM file path");
    let rom_bytes: Vec<u8> = std::fs::read(rom_path).expect("Failed to read ROM");

    // Construct the chip
    let mut chip = Chip8::new();
    chip.load_rom(&rom_bytes).unwrap_or_else(|e| {
        panic!("Failed to load rom: {}", e);
    });

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        window: None,
        pixels: None,
        chip8: chip,
    };

    event_loop.run_app(&mut app)
}
