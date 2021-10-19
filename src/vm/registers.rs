use std::mem;

pub struct Registers {
    registers: Vec<u16>,
}

use super::utils;

fn check(reg: u16) {
    if !utils::is_reg(reg) {
        panic!("Invalid register 0x{:x}", reg)
    }
}

fn as_index(reg: u16) -> usize {
    check(reg);

    (reg >> 8) as usize
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: vec![0; 8],
        }
    }

    pub fn set(&mut self, reg: u16, val: u16) {
        check(reg);
        let index = as_index(reg);
        mem::replace(&mut self.registers[index], val);
    }

    pub fn get(&self, reg: u16) -> u16 {
        check(reg);
        let index = as_index(reg);
        *self.registers.get(index).unwrap()
    }
}
