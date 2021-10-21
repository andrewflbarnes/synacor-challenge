mod vmlib;
pub mod vm {
    pub use super::vmlib::{
        Stack,
        MemBank,
        Registers,
        VirtualMachine,
    };
}

mod disassemble;
pub mod dis {
    pub use super::disassemble::{
        Disassembler,
    };
}