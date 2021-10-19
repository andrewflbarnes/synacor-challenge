use super::{memory::MemBank, opcode::OpCode, registers::Registers, utils};

pub struct VirtualMachine {
    memory: MemBank,
    registers: Registers,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            memory: MemBank::new(),
            registers: Registers::new(),
        }
    }

    pub fn load(&mut self, program: Vec<u16>) {
        self.memory.load_at(0, program)
    }

    pub fn run(&mut self) {
        loop {
            let code = self.memory.next();
            // println!("\n0x{:x} => 0x{:x}", self.memory.get_pointer() - 1, code);
            let op = OpCode::of(code);
            if op == OpCode::HALT {
                return;
            }

            self.exec_opcode(op);
        }
    }

    fn exec_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::SET => {
                let reg = self.memory.next();
                let val = self.next_literal_or_register();
                self.registers.set(reg, val);
            }
            OpCode::OUT => {
                let ch = self.next_literal_or_register();
                Self::console_write(ch);
            }
            OpCode::JMP => {
                let addr = self.next_literal_or_register();
                self.memory.set_pointer(addr);
            }
            OpCode::JT => {
                let check = self.next_literal_or_register();
                let addr = self.next_literal_or_register();
                if check != 0 {
                    self.memory.set_pointer(addr);
                }
            }
            OpCode::JF => {
                let check = self.next_literal_or_register();
                let addr = self.next_literal_or_register();
                if check == 0 {
                    self.memory.set_pointer(addr);
                }
            }
            OpCode::NOOP => {}
            _ => panic!("Unimplemented opcode: {:?}", opcode),
        }
    }

    fn next_literal_or_register(&mut self) -> u16 {
        let raw = self.memory.next();
        self.literal_or_register(raw)
    }

    fn literal_or_register(&self, addr_or_literal: u16) -> u16 {
        if utils::is_reg(addr_or_literal) {
            return self.registers.get(addr_or_literal);
        } else {
            return addr_or_literal;
        }
    }

    fn console_write(ch: u16) {
        print!("{}", utils::little_to_big(ch) as u8 as char)
    }
}
