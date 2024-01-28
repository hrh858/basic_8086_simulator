use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Sim8086Register {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

impl Sim8086Register {
    pub fn from_u8(reg_value: u8, w_value: bool) -> Sim8086Register {
        if !w_value {
            match reg_value {
                0b000 => Self::AL,
                0b001 => Self::CL,
                0b010 => Self::DL,
                0b011 => Self::BL,
                0b100 => Self::AH,
                0b101 => Self::CH,
                0b110 => Self::DH,
                0b111 => Self::BH,
                _ => panic!("Invalid value for register {:b}", reg_value),
            }
        } else {
            match reg_value {
                0b000 => Self::AX,
                0b001 => Self::CX,
                0b010 => Self::DX,
                0b011 => Self::BX,
                0b100 => Self::SP,
                0b101 => Self::BP,
                0b110 => Self::SI,
                0b111 => Self::DI,
                _ => panic!("Invalid value for register {:b}", reg_value),
            }
        }
    }
}

impl Display for Sim8086Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AL => write!(f, "al"),
            Self::CL => write!(f, "cl"),
            Self::DL => write!(f, "dl"),
            Self::BL => write!(f, "bl"),
            Self::AH => write!(f, "ah"),
            Self::CH => write!(f, "ch"),
            Self::DH => write!(f, "dh"),
            Self::BH => write!(f, "bh"),
            Self::AX => write!(f, "ax"),
            Self::CX => write!(f, "cx"),
            Self::DX => write!(f, "dx"),
            Self::BX => write!(f, "bx"),
            Self::SP => write!(f, "sp"),
            Self::BP => write!(f, "bp"),
            Self::SI => write!(f, "si"),
            Self::DI => write!(f, "di"),
        }
    }
}