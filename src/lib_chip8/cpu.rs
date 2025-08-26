const REGISTER_COUNT: usize = 16;

pub struct Cpu {
    pub v: GeneralRegisters, // general purpose registers
    pub i: IRegister,
    pub pc: ProgramCounter,
}
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            v: GeneralRegisters([0; REGISTER_COUNT]),
            i: IRegister(0),
            pc: ProgramCounter(0x200),
        }
    }
}

pub struct GeneralRegisters([u8; REGISTER_COUNT]);
impl GeneralRegisters {
    fn bounds_check(&self, index: u8) -> usize {
        if index as usize >= self.0.len() {
            panic!(
                "out of range register access: {} (0..{})",
                index,
                self.0.len() - 1
            );
        }
        index as usize
    }

    pub fn set(&mut self, index: u8, value: u8) {
        self.0[self.bounds_check(index)] = value;
    }

    pub fn get(&self, index: u8) -> u8 {
        self.0[self.bounds_check(index)]
    }
}

pub struct IRegister(u16);
impl IRegister {
    pub fn set(&mut self, value: u16) {
        self.0 = value;
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}

pub struct ProgramCounter(u16);
impl ProgramCounter {
    pub fn set(&mut self, address: u16) {
        self.0 = address;
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn advance(&mut self) {
        self.0 = self.0.checked_add(2).expect("Program counter overflow");
    }
}
