pub fn little_to_big(val: u16) -> u16 {
    (val >> 8) | (val << 8)
}

pub fn is_reg(reg: u16) -> bool {
    let big_endian = little_to_big(reg);
    big_endian > 0x7FFF && big_endian < 0x8008
}
