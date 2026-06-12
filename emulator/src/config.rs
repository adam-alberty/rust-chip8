use serde::{Deserialize, Serialize};
use std::{fs, path::Path, time::Duration};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

#[derive(Serialize, Deserialize)]
pub struct DisplayConfig {
    pub on_color: Color,
    pub off_color: Color,
    pub scale: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AudioConfig {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TimingConfig {
    pub cpu_hz: u64,
    pub timer_hz: u64,
}

impl TimingConfig {
    pub const fn cpu_tick_duration(&self) -> Duration {
        Duration::from_nanos(1_000_000_000 / self.cpu_hz)
    }

    pub const fn timer_tick_duration(&self) -> Duration {
        Duration::from_nanos(1_000_000_000 / self.timer_hz)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub timing: TimingConfig,
    pub audio: AudioConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                on_color: Color(255, 255, 255, 255),
                off_color: Color(0, 0, 0, 255),
                scale: 25,
            },
            timing: TimingConfig {
                cpu_hz: 700,
                timer_hz: 60,
            },
            audio: AudioConfig { enabled: true },
        }
    }
}

/// Loads config from toml file.
pub fn load_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;

    Ok(config)
}
