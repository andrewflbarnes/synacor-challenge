use crate::vmlib::registers;
use crate::vmlib::OpCode;
pub enum Repr {
    Word,
    Byte,
}

impl Repr {
    fn from_word(&self, word: usize) -> usize {
        match self {
            Repr::Word => word,
            Repr::Byte => word * 2,
        }
    }
}

pub struct Disassembler {
    repr: Repr,
}

impl Disassembler {
    pub fn of(repr: Repr) -> Self {
        Disassembler { repr }
    }

    pub fn new() -> Self {
        Self::of(Repr::Word)
    }

    fn as_repr(&self, val: usize) -> usize {
        self.repr.from_word(val)
    }

    pub fn disassemble(&self, program: Vec<u16>) {
        let mut pointer = 0;

        while pointer < program.len() {
            let raw_opcode = program[pointer];
            print!("{:#06x}: {:#06x} ", self.as_repr(pointer), raw_opcode);

            pointer += 1;

            if !OpCode::valid(raw_opcode) {
                println!("INVALID");
                continue;
            }

            let opcode = OpCode::of(raw_opcode);
            print!("{:<6} ", format!("({:?})", opcode));

            for _ in 0..opcode.args() {
                print!("{} ", self.reg_or_mem(program[pointer]));
                pointer += 1;
            }

            println!("");
        }
    }

    fn reg_or_mem(&self, addr: u16) -> String {
        return if registers::is_reg(addr) {
            format!("r{:<5} ", addr & 0x7)
        } else {
            format!("{:#06x} ", self.as_repr(addr as usize))
        };
    }
}
