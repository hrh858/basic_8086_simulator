pub fn decode_s(byte: u8, at: u8) -> S {
    if (byte >> at) == 0b1 {
        S::Sign
    } else {
        S::NoSign
    }
}

pub fn decode_w(byte: u8, at: u8) -> W {
    if (byte >> at) & 0b1 == 0b1 {
        W::Word
    } else {
        W::Byte
    }
}

pub fn decode_d(byte: u8, at: u8) -> D {
    if (byte >> at) & 0b1 == 0b1 {
        D::RegIsDst
    } else {
        D::RegIsSrc
    }
}

pub fn decode_mod(byte: u8, from: u8) -> Mode {
    match (byte >> (from - 1)) & 0b11 {
        0b00 => Mode::MemoryNoDispWithExeption,
        0b01 => Mode::Memory8BitsDisp,
        0b10 => Mode::Memory16BitsDisp,
        0b11 => Mode::RegisterNoDisp,
        _ => panic!("invalid value mode value")
    }
}

pub fn decode_reg(byte: u8, from: u8, size: W) -> Reg {
    let val = (byte >> (from - 2)) & 0b111;
    match val {
        0b000 => match size {
            W::Byte => Reg::AL,
            W::Word => Reg::AX,
        },
        0b001 => match size {
            W::Byte => Reg::CL,
            W::Word => Reg::CX,
        },
        0b010 => match size {
            W::Byte => Reg::DL,
            W::Word => Reg::DX,
        },
        0b011 => match size {
            W::Byte => Reg::BL,
            W::Word => Reg::BX,
        },
        0b100 => match size {
            W::Byte => Reg::AH,
            W::Word => Reg::SP,
        },
        0b101 => match size {
            W::Byte => Reg::CH,
            W::Word => Reg::BP,
        },
        0b110 => match size {
            W::Byte => Reg::DH,
            W::Word => Reg::SI,
        },
        0b111 => match size {
            W::Byte => Reg::BH,
            W::Word => Reg::DI,
        },
        _ => panic!("Invalid reg value")
    }
}

pub fn decode_rm(byte: u8, from: u8) -> u8 {
    (byte >> (from - 2)) & 0b111
}

pub enum S {
    NoSign,
    Sign,
}

#[derive(Copy, Clone)]
pub enum W {
    Byte,
    Word,
}


#[derive(Copy, Clone)]
pub enum D {
    RegIsSrc,
    RegIsDst,
}


#[derive(Copy, Clone)]
pub enum Mode {
    MemoryNoDispWithExeption,
    Memory8BitsDisp,
    Memory16BitsDisp,
    RegisterNoDisp,
}

#[derive(Debug)]
pub enum Reg {
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
