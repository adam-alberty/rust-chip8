use crate::config;
use crate::{config::Config, keyboard, rom, sound};
use libchip8::config as libconfig;
use libchip8::{Chip8, timers::Timer};
use pixels::{Pixels, SurfaceTexture};
use rodio::Player;
use std::path::Path;
use std::{sync::Arc, time::Instant};
use winit::keyboard::PhysicalKey;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct App<'win> {
    pub config: Config,
    pub chip8: Chip8,
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'win>>,
    pub sink: Player,
    pub last_cpu_tick: Instant,
    pub last_timer_tick: Instant,
}

impl<'win> App<'win> {
    /// Renders display.
    fn render(&mut self) {
        // Display pixels on screen
        if let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels) {
            let display = self.chip8.display.dump();
            let frame = pixels.frame_mut();

            for (i, pixel_on) in display.iter().enumerate() {
                let color = if *pixel_on {
                    [
                        self.config.display.on_color.0,
                        self.config.display.on_color.1,
                        self.config.display.on_color.2,
                        self.config.display.on_color.3,
                    ]
                } else {
                    [
                        self.config.display.off_color.0,
                        self.config.display.off_color.1,
                        self.config.display.off_color.2,
                        self.config.display.off_color.3,
                    ]
                };
                frame[i * 4..(i + 1) * 4].copy_from_slice(&color);
            }
            pixels.render().unwrap();
            window.request_redraw();
        }
    }

    /// Advances CPU.
    fn advance(&mut self) {
        let now = Instant::now();

        while now.duration_since(self.last_cpu_tick) >= self.config.timing.cpu_tick_duration() {
            self.chip8.tick().unwrap();
            self.last_cpu_tick += self.config.timing.cpu_tick_duration();
        }

        while now.duration_since(self.last_timer_tick) >= self.config.timing.timer_tick_duration() {
            self.chip8.timers.tick();
            self.last_timer_tick += self.config.timing.timer_tick_duration();
        }

        // play sound if the sound timer is not 0
        if self.chip8.timers.get(Timer::Sound) > 0 {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }
}

impl<'win> ApplicationHandler for App<'win> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("CHIP-8 Emulator")
                        .with_inner_size(LogicalSize::new(
                            libconfig::DISPLAY_WIDTH as f64 * self.config.display.scale as f64,
                            libconfig::DISPLAY_HEIGHT as f64 * self.config.display.scale as f64,
                        )),
                )
                .unwrap(),
        );

        let pixels = Pixels::new(
            libconfig::DISPLAY_WIDTH as u32,
            libconfig::DISPLAY_HEIGHT as u32,
            SurfaceTexture::new(
                (libconfig::DISPLAY_WIDTH * self.config.display.scale as usize) as u32,
                (libconfig::DISPLAY_HEIGHT * self.config.display.scale as usize) as u32,
                window.clone(),
            ),
        )
        .unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.advance();
                self.render();
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
                if let Some(chip8_key) = keyboard::map_to_chip8(physical_key) {
                    self.chip8
                        .keyboard
                        .set_key(chip8_key, state == ElementState::Pressed);
                }
            }
            _ => (),
        }
    }
}

/// Sets up the event loop.
pub fn set_up_event_loop() -> EventLoop<()> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
}
