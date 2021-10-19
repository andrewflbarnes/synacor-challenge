pub enum OpCode {
    HALT,
    SET,
    PUSH,
    POP,
    EQ,
    GT,
    JMP,
    JT,
    JF,
    ADD,
    MULT,
    MOD,
    AND,
    OR,
    NOT,
    RMEM,
    WMEM,
    CALL,
    RET,
    OUT,
    IN,
    NOOP
}

impl OpCode {
    pub fn of(val: u16) -> Self {
        // Little endian :(
        match val {
            0x00 => OpCode::HALT,
            0x80 => OpCode::SET,
            0x40 => OpCode::PUSH,
            0xC0 => OpCode::POP,
            0x20 => OpCode::EQ,
            0xA0 => OpCode::GT,
            0x60 => OpCode::JMP,
            0xE0 => OpCode::JT,
            0x10 => OpCode::JF,
            0x90 => OpCode::ADD,
            0x50 => OpCode::MULT,
            0xD0 => OpCode::MOD,
            0x30 => OpCode::AND,
            0xB0 => OpCode::OR,
            0x70 => OpCode::NOT,
            0xF0 => OpCode::RMEM,
            0x01 => OpCode::WMEM,
            0xB1 => OpCode::CALL,
            0x41 => OpCode::RET,
            0xC1 => OpCode::OUT,
            0x21 => OpCode::IN,
            0xA1 => OpCode::NOOP,
            _ => panic!("Invalid opcode {}", val)
        }
    }
}