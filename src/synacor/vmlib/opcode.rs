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
    pub fn valid(opcode: u16) -> bool {
        opcode <= 0x15
    }

    pub fn of(val: u16) -> Self {
        // Little endian
        match val {
            0x0000 => OpCode::HALT,
            0x0001 => OpCode::SET,
            0x0002 => OpCode::PUSH,
            0x0003 => OpCode::POP,
            0x0004 => OpCode::EQ,
            0x0005 => OpCode::GT,
            0x0006 => OpCode::JMP,
            0x0007 => OpCode::JT,
            0x0008 => OpCode::JF,
            0x0009 => OpCode::ADD,
            0x000A => OpCode::MULT,
            0x000B => OpCode::MOD,
            0x000C => OpCode::AND,
            0x000D => OpCode::OR,
            0x000E => OpCode::NOT,
            0x000F => OpCode::RMEM,
            0x0010 => OpCode::WMEM,
            0x0011 => OpCode::CALL,
            0x0012 => OpCode::RET,
            0x0013 => OpCode::OUT,
            0x0014 => OpCode::IN,
            0x0015 => OpCode::NOOP,
            _ => panic!("Invalid opcode 0x{:x}", val),
        }
    }

    pub fn args(&self) -> u8 {
        match self {
            OpCode::HALT => 0,
            OpCode::SET => 2,
            OpCode::PUSH => 1,
            OpCode::POP => 1,
            OpCode::EQ => 3,
            OpCode::GT => 3,
            OpCode::JMP => 1,
            OpCode::JT => 2,
            OpCode::JF => 2,
            OpCode::ADD => 3,
            OpCode::MULT => 3,
            OpCode::MOD => 3,
            OpCode::AND => 3,
            OpCode::OR => 3,
            OpCode::NOT => 2,
            OpCode::RMEM => 2,
            OpCode::WMEM => 2,
            OpCode::CALL => 1,
            OpCode::RET => 0,
            OpCode::OUT => 1,
            OpCode::IN => 1,
            OpCode::NOOP => 0,
        }
    }
}
