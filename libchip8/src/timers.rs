pub enum Timer {
    Delay,
    Sound,
}

pub struct Timers {
    delay: u8,
    sound: u8,
}

impl Timers {
    pub fn new() -> Self {
        Self { delay: 0, sound: 0 }
    }

    pub fn get(&self, t: Timer) -> u8 {
        match t {
            Timer::Delay => self.delay,
            Timer::Sound => self.sound,
        }
    }

    pub fn set(&mut self, t: Timer, value: u8) {
        match t {
            Timer::Delay => self.delay = value,
            Timer::Sound => self.sound = value,
        };
    }

    pub fn tick(&mut self) {
        if self.sound > 0 {
            self.sound -= 1;
        }

        if self.delay > 0 {
            self.delay -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_setting() {
        let mut t = Timers::new();
        t.set(Timer::Sound, 10);
        t.set(Timer::Delay, 20);
        assert_eq!(t.get(Timer::Sound), 10);
        assert_eq!(t.get(Timer::Delay), 20);
    }
    #[test]
    fn test_timer_ticking() {
        let mut t = Timers::new();

        t.set(Timer::Sound, 10);
        t.set(Timer::Delay, 20);

        t.tick();
        assert_eq!(t.get(Timer::Sound), 9);
        assert_eq!(t.get(Timer::Delay), 19);
    }
    #[test]
    fn test_timer_undeflowing() {
        let mut t = Timers::new();
        t.tick();
        assert_eq!(t.get(Timer::Sound), 0);
        assert_eq!(t.get(Timer::Delay), 0);
    }
}
