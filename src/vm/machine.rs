use super::{maths, memory::MemBank, opcode::OpCode, registers::Registers, stack::Stack, utils};

pub struct VirtualMachine {
    memory: MemBank,
    registers: Registers,
    stack: Stack,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            memory: MemBank::new(),
            registers: Registers::new(),
            stack: Stack::new(),
        }
    }

    pub fn load(&mut self, program: Vec<u16>) {
        self.memory.load_at(0, program)
    }

    pub fn run(&mut self) {
        loop {
            let code = self.memory.next();
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
            OpCode::PUSH => {
                let addr  = self.memory.next();
                let val = self.literal_or_register(addr);
                self.stack.push(val);
            }
            OpCode::POP => {
                let reg = self.memory.next();
                let val = self.stack.pop();
                self.registers.set(reg, val);

            }
            OpCode::EQ => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers.set(dest, if operand1 == operand2 { 0x0100 } else { 0x0000 });
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
            OpCode::ADD => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers.set(dest, maths::add(operand1, operand2))
            }
            OpCode::OUT => {
                let ch = self.next_literal_or_register();
                Self::console_write(ch);
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
