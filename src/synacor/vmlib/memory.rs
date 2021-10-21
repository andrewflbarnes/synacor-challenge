pub struct MemBank {
    memory: Vec<u16>,
    pointer: u16,
}

impl MemBank {
    pub fn new() -> Self {
        MemBank {
            // 15 bit address space
            memory: vec![0; 0x8FFF],
            pointer: 0,
        }
    }

    pub fn load_at(&mut self, location: u16, data: Vec<u16>) {
        let memloc = location as usize;
        println!("Program size {:?}", data.len());
        self.memory.splice(memloc..data.len(), data);
    }

    pub fn next(&mut self) -> u16 {
        let val = self.read_at_index(self.pointer);
        self.pointer += 1;
        val
    }

    pub fn read_at_addr(&self, addr: u16) -> u16 {
        self.read_at_index(addr)
    }

    fn read_at_index(&self, index: u16) -> u16 {
        if let Some(val) = self.memory.get(index as usize) {
            return *val;
        }
        panic!(
            "End of memory when reading at 0x{:x} (Size 0x{:x})",
            index,
            self.memory.len()
        );
    }

    pub fn write_at_addr(&mut self, addr: u16, val: u16) {
        self.write_at_index(addr, val)
    }

    fn write_at_index(&mut self, index: u16, val: u16) {
        if index as usize > self.memory.len() {
            panic!(
                "End of memory when writing at 0x{:x} (Size 0x{:x})",
                index,
                self.memory.len()
            );
        }
        self.memory[index as usize] = val;
    }

    pub fn set_pointer(&mut self, position: u16) {
        self.pointer = position
    }

    pub fn get_pointer(&self) -> u16 {
        self.pointer
    }
}
