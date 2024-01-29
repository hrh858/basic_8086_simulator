// use crate::v2_sim8086::{
//     dec::{decode_d, decode_mod, decode_reg, decode_rm, decode_w, Mode, Reg, D, W},
//     dis::EffectiveAddress,
// };

#[derive(Debug)]
pub enum MoveVariant {
    RegMemToFromReg,
    ImmToRegMem,
    ImmToReg,
    MemToAcc,
    AccToMem,
}

#[derive(Debug)]
pub enum Operand {
    Address(u16),
    EffectiveAddress(EffectiveAddress),
    ImmediateValue(ImmediateValue),
    Register(Register),
}

#[derive(Debug)]
pub enum ImmediateValue {
    EightBits(i8),
    SixteenBits(i16),
}

#[derive(Debug)]
pub enum EffectiveAddress {
    JustRegister(Register),
    RegisterAndOffset(Register, Register),
    RegisterAndDisplacement(Register, Displacement),
    RegisterOffsetAndDisplacement(Register, Register, Displacement),
}

#[derive(Debug)]
pub enum Displacement {
    EightBits(i8),
    SixteenBits(i16),
}

#[derive(Debug)]
pub enum Register {
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
}

#[derive(Debug)]
pub struct GetOperandsParams {
    pub mode: u8,
    pub rm: u8,
    pub w: u8,
}

pub fn get_operand(
    mode: u8,
    rm: u8,
    w: u8,
    first_displacement_byte: Option<u8>,
    second_displacement_byte: Option<u8>,
) -> (Operand, u8) {
    match rm {
        0b000 => match mode {
            0b00 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndOffset(
                    Register::BX,
                    Register::SI,
                )),
                2,
            ),
            0b01 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterOffsetAndDisplacement(
                    Register::BX,
                    Register::SI,
                    Displacement::EightBits(first_displacement_byte.unwrap() as i8),
                )),
                3,
            ),
            0b10 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterOffsetAndDisplacement(
                    Register::BX,
                    Register::SI,
                    Displacement::SixteenBits(
                        ((second_displacement_byte.unwrap() as i16) << 8)
                            | first_displacement_byte.unwrap() as i16,
                    ),
                )),
                4,
            ),
            0b11 => match w {
                0b0 => (Operand::Register(Register::AL), 2),
                0b1 => (Operand::Register(Register::AX), 2),
                _ => panic!(),
            },
            _ => panic!(),
        },
        0b001 => match mode {
            0b00 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndOffset(
                    Register::BX,
                    Register::DI,
                )),
                2,
            ),
            0b01 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterOffsetAndDisplacement(
                    Register::BX,
                    Register::DI,
                    Displacement::EightBits(first_displacement_byte.unwrap() as i8),
                )),
                3,
            ),
            0b10 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterOffsetAndDisplacement(
                    Register::BX,
                    Register::DI,
                    Displacement::SixteenBits(
                        ((second_displacement_byte.unwrap() as i16) << 8)
                            | first_displacement_byte.unwrap() as i16,
                    ),
                )),
                4,
            ),
            0b11 => match w {
                0b0 => (Operand::Register(Register::CL), 2),
                0b1 => (Operand::Register(Register::CX), 2),
                _ => panic!(),
            },
            _ => panic!(),
        },
        0b010 => match mode {
            0b00 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndOffset(
                    Register::BP,
                    Register::SI,
                )),
                2,
            ),
            0b01 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterOffsetAndDisplacement(
                    Register::BP,
                    Register::SI,
                    Displacement::EightBits(first_displacement_byte.unwrap() as i8),
                )),
                3,
            ),
            0b10 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterOffsetAndDisplacement(
                    Register::BP,
                    Register::SI,
                    Displacement::SixteenBits(
                        ((second_displacement_byte.unwrap() as i16) << 8)
                            | first_displacement_byte.unwrap() as i16,
                    ),
                )),
                4,
            ),
            0b11 => match w {
                0b0 => (Operand::Register(Register::DL), 2),
                0b1 => (Operand::Register(Register::DX), 2),
                _ => panic!(),
            },
            _ => panic!(),
        },
        0b011 => match mode {
            0b00 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndOffset(
                    Register::BP,
                    Register::DI,
                )),
                2,
            ),
            0b01 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterOffsetAndDisplacement(
                    Register::BP,
                    Register::DI,
                    Displacement::EightBits(first_displacement_byte.unwrap() as i8),
                )),
                3,
            ),
            0b10 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterOffsetAndDisplacement(
                    Register::BP,
                    Register::DI,
                    Displacement::SixteenBits(
                        ((second_displacement_byte.unwrap() as i16) << 8)
                            | first_displacement_byte.unwrap() as i16,
                    ),
                )),
                4,
            ),
            0b11 => match w {
                0b0 => (Operand::Register(Register::BL), 2),
                0b1 => (Operand::Register(Register::BX), 2),
                _ => panic!(),
            },
            _ => panic!(),
        },
        0b100 => match mode {
            0b00 => (Operand::Register(Register::SI), 2),
            0b01 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndDisplacement(
                    Register::SI,
                    Displacement::EightBits(first_displacement_byte.unwrap() as i8),
                )),
                3,
            ),
            0b10 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndDisplacement(
                    Register::SI,
                    Displacement::SixteenBits(
                        ((second_displacement_byte.unwrap() as i16) << 8)
                            | first_displacement_byte.unwrap() as i16,
                    ),
                )),
                4,
            ),
            0b11 => match w {
                0b0 => (Operand::Register(Register::AH), 2),
                0b1 => (Operand::Register(Register::SP), 2),
                _ => panic!(),
            },
            _ => panic!(),
        },
        0b101 => match mode {
            0b00 => (Operand::Register(Register::DI), 2),
            0b01 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndDisplacement(
                    Register::DI,
                    Displacement::EightBits(first_displacement_byte.unwrap() as i8),
                )),
                3,
            ),
            0b10 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndDisplacement(
                    Register::DI,
                    Displacement::SixteenBits(
                        ((second_displacement_byte.unwrap() as i16) << 8)
                            | first_displacement_byte.unwrap() as i16,
                    ),
                )),
                4,
            ),
            0b11 => match w {
                0b0 => (Operand::Register(Register::CH), 2),
                0b1 => (Operand::Register(Register::BP), 2),
                _ => panic!(),
            },
            _ => panic!(),
        },
        0b110 => match mode {
            0b00 => (Operand::Address(todo!()), 4),
            0b01 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndDisplacement(
                    Register::BP,
                    Displacement::EightBits(first_displacement_byte.unwrap() as i8),
                )),
                3,
            ),
            0b10 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndDisplacement(
                    Register::BP,
                    Displacement::SixteenBits(
                        ((second_displacement_byte.unwrap() as i16) << 8)
                            | first_displacement_byte.unwrap() as i16,
                    ),
                )),
                4,
            ),
            0b11 => match w {
                0b0 => (Operand::Register(Register::DH), 2),
                0b1 => (Operand::Register(Register::SI), 2),
                _ => panic!(),
            },
            _ => panic!(),
        },
        0b111 => match mode {
            0b00 => (Operand::Register(Register::BX), 2),
            0b01 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndDisplacement(
                    Register::BX,
                    Displacement::EightBits(first_displacement_byte.unwrap() as i8),
                )),
                3,
            ),
            0b10 => (
                Operand::EffectiveAddress(EffectiveAddress::RegisterAndDisplacement(
                    Register::BX,
                    Displacement::SixteenBits(
                        ((second_displacement_byte.unwrap() as i16) << 8)
                            | first_displacement_byte.unwrap() as i16,
                    ),
                )),
                4,
            ),
            0b11 => match w {
                0b0 => (Operand::Register(Register::BH), 2),
                0b1 => (Operand::Register(Register::DI), 2),
                _ => panic!(),
            },
            _ => panic!(),
        },
        _ => panic!(),
    }
}
