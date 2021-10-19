pub struct Registers {
    registers: Vec<u16>,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: vec![0; 8],
        }
    }

    pub fn get(&self, reg: u16) -> u16 {
        if reg > 7 {
            panic!("Invalid register {}!", reg)
        }

        *self.registers.get(reg as usize).unwrap()
    }
}
