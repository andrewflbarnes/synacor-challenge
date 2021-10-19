#[derive(PartialEq, Debug)]
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
    NOOP,
}

impl OpCode {
    pub fn of(val: &u16) -> Self {
        // Little endian
        match val {
            0x0000 => OpCode::HALT,
            0x0100 => OpCode::SET,
            0x0200 => OpCode::PUSH,
            0x0300 => OpCode::POP,
            0x0400 => OpCode::EQ,
            0x0500 => OpCode::GT,
            0x0600 => OpCode::JMP,
            0x0700 => OpCode::JT,
            0x0800 => OpCode::JF,
            0x0900 => OpCode::ADD,
            0x0A00 => OpCode::MULT,
            0x0B00 => OpCode::MOD,
            0x0C00 => OpCode::AND,
            0x0D00 => OpCode::OR,
            0x0E00 => OpCode::NOT,
            0x0F00 => OpCode::RMEM,
            0x1000 => OpCode::WMEM,
            0x1100 => OpCode::CALL,
            0x1200 => OpCode::RET,
            0x1300 => OpCode::OUT,
            0x1400 => OpCode::IN,
            0x1500 => OpCode::NOOP,
            _ => panic!("Invalid opcode {}", val),
        }
    }
}
