use crate::{
    app,
    config::{self, Config},
    rom, sound,
};
use libchip8::Chip8;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

/// Runs the ROM.
pub fn run_rom(rom_path: &Path, config_path: Option<PathBuf>) -> anyhow::Result<()> {
    let config = if let Some(config_path) = config_path {
        config::load_config(config_path)?
    } else {
        config::Config::default()
    };

    let rom_bytes = rom::read_rom_bytes(rom_path)?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom_bytes)?;

    // set up audio sink
    let (sink, _out) = sound::create_beep_sink();

    let mut app = app::App {
        config,
        chip8,
        pixels: None,
        window: None,
        last_cpu_tick: Instant::now(),
        last_timer_tick: Instant::now(),
        sink,
    };
    app::set_up_event_loop().run_app(&mut app).unwrap();

    Ok(())
}

/// Generates the default emulator configuration.
pub fn generate_default_config(path: &Path) -> anyhow::Result<()> {
    let mut file = File::create(path)?;

    let default_config = Config::default();
    let serialized_config = toml::to_string_pretty(&default_config)?;

    file.write_all(&serialized_config.into_bytes())?;

    Ok(())
}
