pub struct Stack {
    stack: Vec<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: u16) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Option<u16> {
        self.stack.pop()
    }
}
