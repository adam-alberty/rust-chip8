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
        Timers { delay: 0, sound: 0 }
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

    pub fn decrement(&mut self, t: Timer) {
        match t {
            Timer::Delay => {
                if self.delay > 0 {
                    self.delay -= 1;
                }
            }
            Timer::Sound => {
                if self.sound > 0 {
                    self.sound -= 1;
                }
            }
        };
    }
}
