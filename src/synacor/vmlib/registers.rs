pub struct Registers {
    registers: Vec<u16>,
}

pub fn is_reg(reg: u16) -> bool {
    reg > 0x7FFF && reg < 0x8008
}

fn check(reg: u16) {
    if !is_reg(reg) {
        panic!("Invalid register 0x{:x}", reg)
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: vec![0; 8],
        }
    }

    pub fn set(&mut self, reg: u16, val: u16) {
        check(reg);
        self.registers[(reg & 0x7) as usize] = val;
    }

    pub fn get(&self, reg: u16) -> u16 {
        check(reg);
        *self.registers.get((reg & 0x7) as usize).unwrap()
    }
}
