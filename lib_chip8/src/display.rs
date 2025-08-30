/////////////////////////////////////////////
///               DISPLAY                 ///
/////////////////////////////////////////////
use crate::config;

pub struct Display([bool; config::DISPLAY_WIDTH * config::DISPLAY_HEIGHT]);
impl Display {
    pub fn new() -> Self {
        Display([false; config::DISPLAY_WIDTH * config::DISPLAY_HEIGHT])
    }

    pub fn dump(&self) -> &[bool] {
        &self.0
    }

    pub fn clear(&mut self) {
        self.0.fill(false);
    }

    pub fn display_sprite(&mut self, pos_x: usize, pos_y: usize, data: &[u8]) -> bool {
        let mut collision = false;

        for row in 0..data.len() {
            for col in 0..8 {
                let screen_x = pos_x + col;
                let screen_y = pos_y + row;

                let new_pixel = ((data[row] >> (7 - col)) & 1) == 1;
                let current_pixel = self.get_pixel(screen_x, screen_y);

                if current_pixel && new_pixel {
                    collision = true;
                }
                self.set_pixel(screen_x, screen_y, current_pixel ^ new_pixel);
            }
        }
        collision
    }

    fn get_pixel(&self, mut x: usize, mut y: usize) -> bool {
        x %= config::DISPLAY_WIDTH;
        y %= config::DISPLAY_HEIGHT;
        self.0[y * config::DISPLAY_WIDTH + x]
    }

    fn set_pixel(&mut self, mut x: usize, mut y: usize, on: bool) {
        x %= config::DISPLAY_WIDTH;
        y %= config::DISPLAY_HEIGHT;
        self.0[y * config::DISPLAY_WIDTH + x] = on
    }
}
