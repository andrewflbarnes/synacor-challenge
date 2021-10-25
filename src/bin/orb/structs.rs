use std::fmt;

pub struct Path {
    pub location: usize,
    pub steps: u8,
    pub val: u64,
    pub route: String,
}

#[derive(Eq, PartialEq)]
pub enum Op {
    Add,
    Mul,
    Sub,
}

impl Op {
    pub fn apply(&self, operand1: u64, operand2: u64) -> u64 {
        match self {
            Op::Add => operand1 + operand2,
            Op::Sub => operand1 - operand2,
            Op::Mul => operand1 * operand2,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
        };
        write!(f, "{}", symbol)
    }
}

pub struct Tile<'a> {
    pub value: u64,
    pub neighbours: &'a [(usize, Op)],
}

impl<'a> Tile<'a> {
    pub const fn new(value: u64, neighbours: &'a [(usize, Op)]) -> Self {
        Tile { value, neighbours }
    }
}
