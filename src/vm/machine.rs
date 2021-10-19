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

    pub fn exec_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::OUT => Self::console_write(self.memory.next()),
            OpCode::JMP => {
                let raw = self.memory.next();
                let addr = raw >> 8 | (raw << 8);
                self.memory.set_pointer(addr);
            },
            OpCode::NOOP => {},
            _ => panic!("Unimplemented opcode: {:?}", opcode),
        }
    }

    fn console_write(ch: u16) {
        let resolved_ch = (ch >> 8) as u8 as char;
        print!("{}", resolved_ch)
    }
}
