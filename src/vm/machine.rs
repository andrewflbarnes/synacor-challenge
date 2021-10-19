use super:: {
    memory::MemBank,
    registers::Registers,
};

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

    pub fn load(&self, program: Vec<u16>) {
        self.memory.loadAt(0, program)
    }

    pub fn run(&self) {
        
    }
}