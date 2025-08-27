/////////////////////////////////////////////
///               DISPLAY                 ///
/////////////////////////////////////////////

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display([bool; DISPLAY_WIDTH * DISPLAY_HEIGHT]);
impl Display {
    pub fn new() -> Self {
        Display([false; DISPLAY_WIDTH * DISPLAY_HEIGHT])
    }

    pub fn dump(&self) -> &[bool] {
        &self.0
    }

    pub fn clear(&mut self) {
        self.0.fill(false);
    }

    pub fn get_resolution(&self) -> (usize, usize) {
        (DISPLAY_WIDTH, DISPLAY_HEIGHT)
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

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        let x = x % DISPLAY_WIDTH;
        let y = y % DISPLAY_HEIGHT;
        self.0[y * DISPLAY_WIDTH + x]
    }

    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        let x = x % DISPLAY_WIDTH;
        let y = y % DISPLAY_HEIGHT;
        self.0[y * DISPLAY_WIDTH + x] = on
    }
}
