use super::opc::mov::Register;

pub fn decode_w(byte: u8, at: u8) -> u8 {
    (byte >> at) & 0b1
}

pub fn decode_d(byte: u8, at: u8) -> u8 {
    (byte >> at) & 0b1
}

pub fn decode_reg(byte: u8, from: u8) -> u8 {
    (byte >> (from - 2)) & 0b111
}

pub fn decode_rm(byte: u8, from: u8) -> u8 {
    (byte >> (from - 2)) & 0b111
}

pub fn decode_mode(byte: u8, from: u8) -> u8 {
    (byte >> (from - 1)) & 0b11
}

pub enum RegisterSize {
    EightBits,
    SixteenBits,
}
pub fn decode_register(byte: u8, from: u8, size: RegisterSize) -> Register {
    let value = (byte >> (from - 2)) & 0b111;
    match size {
        RegisterSize::EightBits => match value {
            0b000 => Register::AL,
            0b001 => Register::CL,
            0b010 => Register::DL,
            0b011 => Register::BL,
            0b100 => Register::AH,
            0b101 => Register::CH,
            0b110 => Register::DH,
            0b111 => Register::BH,
            _ => panic!(),
        },
        RegisterSize::SixteenBits => match value {
            0b000 => Register::AX,
            0b001 => Register::CX,
            0b010 => Register::DX,
            0b011 => Register::BX,
            0b100 => Register::SP,
            0b101 => Register::BP,
            0b110 => Register::SI,
            0b111 => Register::DI,
            _ => panic!(),
        },
    }
}
