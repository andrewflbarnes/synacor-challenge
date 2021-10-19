use super::utils;

const MODULO: u16 = 0x8000;

pub fn add(operand1: u16, operand2: u16) -> u16 {
    let op1 = utils::little_to_big(operand1);
    let op2 = utils::little_to_big(operand2);

    let result = (op1 + op2) % MODULO;

    utils::little_to_big(result as u16)
}
