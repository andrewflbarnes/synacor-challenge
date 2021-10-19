use super::{memory::MemBank, opcode::OpCode, registers::Registers};

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
            let op = OpCode::of(&code);
            if op == OpCode::HALT {
                return;
            }

            self.exec_opcode(op);
        }
    }

    fn exec_opcode(&mut self, opcode: OpCode) {
        match opcode {
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
            OpCode::NOOP => {}
            _ => panic!("Unimplemented opcode: {:?}", opcode),
        }
    }

    fn next_literal_or_register(&mut self) -> u16 {
        let raw = self.memory.next();
        self.literal_or_register(raw)
    }

    fn literal_or_register(&self, addr_or_literal: u16) -> u16 {
        if 0x0080 & addr_or_literal == 0x0080 {
            let reg = (0x0700 & addr_or_literal) >> 8;
            return self.registers.get(reg);
        } else {
            return addr_or_literal >> 8 | (addr_or_literal << 8);
        }
    }

    fn console_write(ch: u16) {
        let resolved_ch = (ch >> 8) as u8 as char;
        print!("{}", resolved_ch)
    }
}
