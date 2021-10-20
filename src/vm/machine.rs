use super::{maths, memory::MemBank, opcode::OpCode, registers::Registers, stack::Stack, utils};
use std::io;
use std::io::Read;

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
            if OpCode::HALT == self.exec_opcode(op) {
                return;
            }
        }
    }

    fn exec_opcode(&mut self, opcode: OpCode) -> OpCode {
        match opcode {
            OpCode::HALT => {}
            OpCode::SET => {
                let reg = self.memory.next();
                let val = self.next_literal_or_register();
                self.registers.set(reg, val);
            }
            OpCode::PUSH => {
                let addr = self.memory.next();
                let val = self.literal_or_register(addr);
                self.stack.push(val);
            }
            OpCode::POP => {
                let reg = self.memory.next();
                let val = self.stack.pop().expect("Empty stack");
                self.registers.set(reg, val);
            }
            OpCode::EQ => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers
                    .set(dest, if operand1 == operand2 { 0x0100 } else { 0x0000 });
            }
            OpCode::GT => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers
                    .set(dest, if operand1 > operand2 { 0x0100 } else { 0x0000 });
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
            OpCode::MULT => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers.set(dest, maths::mult(operand1, operand2))
            }
            OpCode::MOD => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers.set(dest, maths::modulo(operand1, operand2))
            }
            OpCode::AND => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers.set(dest, operand1 & operand2)
            }
            OpCode::OR => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers.set(dest, operand1 | operand2)
            }
            OpCode::NOT => {
                let dest = self.memory.next();
                let operand = self.next_literal_or_register();

                self.registers.set(dest, operand ^ 0xFF7F)
            }
            OpCode::RMEM => {
                let dest = self.memory.next();
                let mem_addr = self.next_literal_or_register();

                let mem_val = self.memory.read_at_addr(mem_addr);

                self.registers.set(dest, mem_val);
            }
            OpCode::WMEM => {
                let dest = self.next_literal_or_register();
                let val = self.next_literal_or_register();

                self.memory.write_at_addr(dest, val);
            }
            OpCode::CALL => {
                let addr = self.next_literal_or_register();

                let next_instr = self.memory.get_pointer();
                self.stack.push(next_instr);

                self.memory.set_pointer(addr)
            }
            OpCode::RET => {
                if let Some(addr) = self.stack.pop() {
                    self.memory.set_pointer(addr);
                } else {
                    return OpCode::HALT
                }
            }
            OpCode::OUT => {
                let ch = self.next_literal_or_register();
                Self::console_write(ch);
            }
            OpCode::IN => {
                let dest = self.memory.next();

                let mut buf = vec![0; 1];
                io::stdin().read_exact(&mut buf).expect("Expected input");

                self.registers.set(dest, (buf[0] as u16) << 8)
            }
            OpCode::NOOP => {}
        }

        return opcode;
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
