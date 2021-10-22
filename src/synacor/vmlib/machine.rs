use super::{memory::MemBank, opcode::OpCode, registers, registers::Registers, stack::Stack};
use std::io;
use std::io::Read;

const MODULO: u16 = 0x7FFF;

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

    pub fn init_register(&mut self, reg: u16, val: u16) {
        self.registers.set(reg, val)
    }

    pub fn run(&mut self) {
        loop {
            let code = self.memory.next();
            let op = OpCode::of(code);

            if !self.exec_opcode(op) {
                return;
            }
        }
    }

    fn exec_opcode(&mut self, opcode: OpCode) -> bool {
        match opcode {
            OpCode::HALT => {
                return false;
            }
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
                    .set(dest, if operand1 == operand2 { 0x0001 } else { 0x0000 });
            }
            OpCode::GT => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers
                    .set(dest, if operand1 > operand2 { 0x0001 } else { 0x0000 });
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

                self.registers.set(dest, (operand1 + operand2) & MODULO)
            }
            OpCode::MULT => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers.set(
                    dest,
                    ((operand1 as u32 * operand2 as u32) & MODULO as u32) as u16,
                )
            }
            OpCode::MOD => {
                let dest = self.memory.next();
                let operand1 = self.next_literal_or_register();
                let operand2 = self.next_literal_or_register();

                self.registers.set(dest, operand1 % operand2)
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

                self.registers.set(dest, operand ^ 0x7FFF)
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
                    return false;
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

                self.registers.set(dest, buf[0] as u16)
            }
            OpCode::NOOP => {}
        }

        true
    }

    fn next_literal_or_register(&mut self) -> u16 {
        let raw = self.memory.next();
        self.literal_or_register(raw)
    }

    fn literal_or_register(&self, addr_or_literal: u16) -> u16 {
        if registers::is_reg(addr_or_literal) {
            return self.registers.get(addr_or_literal);
        } else {
            return addr_or_literal;
        }
    }

    fn console_write(ch: u16) {
        print!("{0}", ch as u8 as char)
    }
}
