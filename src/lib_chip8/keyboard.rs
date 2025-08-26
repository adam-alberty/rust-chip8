const KEY_COUNT: usize = 16;

pub struct Keyboard([bool; KEY_COUNT]);
impl Keyboard {
    pub fn new() -> Self {
        Keyboard([false; KEY_COUNT])
    }

    pub fn is_pressed(&mut self, key: u8) -> bool {
        if key as usize >= self.0.len() {
            return false;
        }
        self.0[key as usize]
    }
}
