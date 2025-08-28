const KEY_COUNT: u8 = 16;

pub struct Keyboard([bool; KEY_COUNT as usize]);
impl Keyboard {
    pub fn new() -> Self {
        Keyboard([false; KEY_COUNT as usize])
    }

    pub fn is_pressed(&mut self, key: u8) -> bool {
        if key as usize >= self.0.len() {
            return false;
        }
        self.0[key as usize]
    }

    pub fn get_pressed_key(&mut self) -> Option<u8> {
        for idx in 0..KEY_COUNT {
            if self.0[idx as usize] {
                return Some(idx);
            }
        }

        None
    }

    pub fn set_key(&mut self, key: u8, is_pressed: bool) {
        if let Some(slot) = self.0.get_mut(key as usize) {
            *slot = is_pressed;
        }
    }
}
