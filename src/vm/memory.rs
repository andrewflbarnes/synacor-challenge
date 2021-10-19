pub struct MemBank {
    memory: Vec<u16>,
}

impl MemBank {
    pub fn new() -> Self {
        MemBank {
            // 15 bit address space
            memory: vec![0; 0x8F]
        }
    }

    pub fn loadAt(&self, location: u16, data: Vec<u16>) {
        // TODO
    }

    pub fn run(&self) {

    }
}