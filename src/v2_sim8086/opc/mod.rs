pub mod cond_jump;
pub mod mov;
pub mod arith;

use mov::{MoveVariant, RegMemToFromRegStruct, ImmToRegStruct, ImmToRegMemStruct, MemToAccStruct, AccToMemStruct};
use arith::{ArithmeticVariant, ArithmeticFamily};
use cond_jump::ConditionalJumpVariant;

use self::arith::{RegMemAndRegEitherStruct, ImmRegMemStruct, ImmAccStruct};

#[derive(Debug)]
pub enum Opcode {
    Move {
        variant: MoveVariant,
    },
    Arithmetic {
        variant: ArithmeticVariant,
        family: ArithmeticFamily,
    },
    ConditionalJump {
        variant: ConditionalJumpVariant,
    },
    NotImplemented,
}

pub fn parse_opcode(byte: u8) -> Opcode {
    if let Some(variant) = try_decode_move(byte) {
        Opcode::Move { variant }
    } else if let Some((family, variant)) = try_decode_arithmetic(byte) {
        Opcode::Arithmetic { family, variant }
    } else if let Some(variant) = try_decode_jump(byte) {
        Opcode::ConditionalJump { variant }
    } else {
        Opcode::NotImplemented
    }
}

fn try_decode_move(byte: u8) -> Option<MoveVariant> {
    if byte >> 2 == 0b100010 {
        return Some(MoveVariant::RegMemToFromReg(RegMemToFromRegStruct{}));
    }
    if byte >> 1 == 0b1100011 {
        return Some(MoveVariant::ImmToRegMem(ImmToRegMemStruct{}));
    }
    if byte >> 4 == 0b1011 {
        return Some(MoveVariant::ImmToReg(ImmToRegStruct{}));
    }
    if byte >> 1 == 0b1010000 {
        return Some(MoveVariant::MemToAcc(MemToAccStruct{}));
    }
    if byte >> 1 == 0b1010001 {
        return Some(MoveVariant::AccToMem(AccToMemStruct{}));
    }
    // if byte == 0b10001110 {
    //     return Some(MoveVariant::RegMemToSeg);
    // }
    // if byte == 10001100 {
    //     return Some(MoveVariant::SegToRegMem);
    // }
    None
}

fn try_decode_arithmetic(byte: u8) -> Option<(ArithmeticFamily, ArithmeticVariant)> {
    if byte >> 2 == 0b000000 {
        return Some((ArithmeticFamily::Add, ArithmeticVariant::RegMemAndRegEither(RegMemAndRegEitherStruct{})));
    }
    if byte >> 2 == 0b100000 {
        return Some((ArithmeticFamily::Add, ArithmeticVariant::ImmRegMem(ImmRegMemStruct{})));
    }
    if byte >> 1 == 0b0000010 {
        return Some((ArithmeticFamily::Add, ArithmeticVariant::ImmAcc(ImmAccStruct{})));
    }
    if byte >> 2 == 0b001010 {
        return Some((ArithmeticFamily::Sub, ArithmeticVariant::RegMemAndRegEither(RegMemAndRegEitherStruct{})));
    }
    if byte >> 2 == 0b100000 {
        return Some((ArithmeticFamily::Sub, ArithmeticVariant::ImmRegMem(ImmRegMemStruct{})));
    }
    if byte >> 1 == 0b0010110 {
        return Some((ArithmeticFamily::Sub, ArithmeticVariant::ImmAcc(ImmAccStruct{})));
    }
    if byte >> 2 == 0b001110 {
        return Some((ArithmeticFamily::Cmp, ArithmeticVariant::RegMemAndRegEither(RegMemAndRegEitherStruct{})));
    }
    if byte >> 2 == 0b100000 {
        return Some((ArithmeticFamily::Cmp, ArithmeticVariant::ImmRegMem(ImmRegMemStruct{})));
    }
    if byte >> 1 == 0b0011110 {
        return Some((ArithmeticFamily::Cmp, ArithmeticVariant::ImmAcc(ImmAccStruct{})));
    }
    None
}

fn try_decode_jump(byte: u8) -> Option<ConditionalJumpVariant> {
    if byte == 0b01110100 {
        return Some(ConditionalJumpVariant::JeJz);
    }
    if byte == 0b01111100 {
        return Some(ConditionalJumpVariant::JlJnge);
    }
    if byte == 0b01111110 {
        return Some(ConditionalJumpVariant::JleJng);
    }
    if byte == 0b01110010 {
        return Some(ConditionalJumpVariant::JbJnae);
    }
    if byte == 0b01110110 {
        return Some(ConditionalJumpVariant::JbeJna);
    }
    if byte == 0b01111010 {
        return Some(ConditionalJumpVariant::JpJpe);
    }
    if byte == 0b01110000 {
        return Some(ConditionalJumpVariant::Jo);
    }
    if byte == 0b01111000 {
        return Some(ConditionalJumpVariant::Js);
    }
    if byte == 0b01110101 {
        return Some(ConditionalJumpVariant::JneJnz);
    }
    if byte == 0b01111101 {
        return Some(ConditionalJumpVariant::JnlJge);
    }
    if byte == 0b01111111 {
        return Some(ConditionalJumpVariant::JnleJg);
    }
    if byte == 0b01110011 {
        return Some(ConditionalJumpVariant::JnbJae);
    }
    if byte == 0b01110111 {
        return Some(ConditionalJumpVariant::JnbeJa);
    }
    if byte == 0b01111011 {
        return Some(ConditionalJumpVariant::JnpJpo);
    }
    if byte == 0b011100 {
        return Some(ConditionalJumpVariant::Jno);
    }
    None
}