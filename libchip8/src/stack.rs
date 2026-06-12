use crate::config;

pub struct Stack {
    data: [u16; config::STACK_SIZE],
    sp: usize, // stack pointer
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: [0; config::STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.sp >= config::STACK_SIZE {
            panic!(
                "Stack overflow: tried to push {:04X} with SP at {}",
                value, self.sp
            );
        }
        self.data[self.sp] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.sp == 0 {
            panic!("Stack underflow: attempted pop with SP at 0");
        }
        self.sp -= 1;
        self.data[self.sp]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push() {
        let mut s = Stack::new();
        s.push(5);
        assert_eq!(1, s.sp);
        assert_eq!(5, s.data[0]);
    }
    #[test]
    fn test_stack_pop() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        let value = s.pop();
        assert_eq!(1, s.sp);
        assert_eq!(2, value);
    }
    #[test]
    #[should_panic]
    fn test_stack_push_overflow() {
        let mut s = Stack::new();
        s.sp = config::STACK_SIZE;
        s.push(1);
    }
    #[test]
    #[should_panic]
    fn test_stack_pop_underflow() {
        let mut s = Stack::new();
        s.sp = 0;
        s.pop();
    }
}
