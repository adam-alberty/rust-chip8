/////////////////////////////////////////////
///               DISPLAY                 ///
/////////////////////////////////////////////

pub struct Display([bool; 64 * 32]);
impl Display {
    pub fn clear(&mut self) {
        self.0.fill(false);
    }
}
