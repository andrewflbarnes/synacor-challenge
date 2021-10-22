mod vmlib;
pub mod vm {
    pub use super::vmlib::{MemBank, Registers, Stack, VirtualMachine};
}

mod disassemble;
pub mod dis {
    pub use super::disassemble::Disassembler;
}
