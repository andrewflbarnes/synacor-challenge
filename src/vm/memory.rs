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
        let val = self.read_at(self.pointer);
        self.pointer += 1;
        val
    }

    pub fn read_at(&self, location: u16) -> u16 {
        if let Some(val) = self.memory.get(location as usize) {
            return *val;
        }
        panic!("End of memory when reading at {}", location)
    }

    pub fn set_pointer(&mut self, position: u16) {
        self.pointer = position
    }
}
