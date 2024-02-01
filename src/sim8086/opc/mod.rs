pub mod arith;
pub mod cond_jump;
pub mod mov;

use arith::{ArithmeticFamily, ArithmeticVariant};
use cond_jump::ConditionalJumpVariant;

use self::mov::MoveVariant;

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

pub fn parse_opcode(first_byte: u8, second_byte: u8) -> Opcode {
    if let Some(variant) = try_decode_move(first_byte) {
        Opcode::Move { variant }
    } else if let Some((family, variant)) = try_decode_arithmetic(first_byte, second_byte) {
        Opcode::Arithmetic { family, variant }
    } else if let Some(variant) = try_decode_jump(first_byte) {
        Opcode::ConditionalJump { variant }
    } else {
        Opcode::NotImplemented
    }
}

fn try_decode_move(byte: u8) -> Option<MoveVariant> {
    if byte >> 2 == 0b100010 {
        return Some(MoveVariant::RegMemToFromReg);
    }
    if byte >> 1 == 0b1100011 {
        return Some(MoveVariant::ImmToRegMem);
    }
    if byte >> 4 == 0b1011 {
        return Some(MoveVariant::ImmToReg);
    }
    if byte >> 1 == 0b1010000 {
        return Some(MoveVariant::MemToAcc);
    }
    if byte >> 1 == 0b1010001 {
        return Some(MoveVariant::AccToMem);
    }
    // if byte == 0b10001110 {
    //     return Some(MoveVariant::RegMemToSeg);
    // }
    // if byte == 10001100 {
    //     return Some(MoveVariant::SegToRegMem);
    // }
    None
}

fn try_decode_arithmetic(
    first_byte: u8,
    second_byte: u8,
) -> Option<(ArithmeticFamily, ArithmeticVariant)> {
    if first_byte >> 2 == 0b000000 {
        return Some((ArithmeticFamily::Add, ArithmeticVariant::RegMemAndRegEither));
    }
    if first_byte >> 2 == 0b100000 {
        return Some((
            match (second_byte >> 3) & 0b111 {
                0b000 => ArithmeticFamily::Add, // add
                0b101 => ArithmeticFamily::Sub, // sub
                0b111 => ArithmeticFamily::Cmp, // cmp
                _ => todo!(),
            },
            ArithmeticVariant::ImmRegMem,
        ));
    }
    if first_byte >> 1 == 0b0000010 {
        return Some((ArithmeticFamily::Add, ArithmeticVariant::ImmAcc));
    }
    if first_byte >> 2 == 0b001010 {
        return Some((ArithmeticFamily::Sub, ArithmeticVariant::RegMemAndRegEither));
    }
    if first_byte >> 2 == 0b100000 {
        return Some((ArithmeticFamily::Sub, ArithmeticVariant::ImmRegMem));
    }
    if first_byte >> 1 == 0b0010110 {
        return Some((ArithmeticFamily::Sub, ArithmeticVariant::ImmAcc));
    }
    if first_byte >> 2 == 0b001110 {
        return Some((ArithmeticFamily::Cmp, ArithmeticVariant::RegMemAndRegEither));
    }
    if first_byte >> 2 == 0b100000 {
        return Some((ArithmeticFamily::Cmp, ArithmeticVariant::ImmRegMem));
    }
    if first_byte >> 1 == 0b0011110 {
        return Some((ArithmeticFamily::Cmp, ArithmeticVariant::ImmAcc));
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
    if byte == 0b01110001 {
        return Some(ConditionalJumpVariant::Jno);
    }
    if byte == 0b01111001 {
        return Some(ConditionalJumpVariant::Jns);
    }
    if byte == 0b11100010 {
        return Some(ConditionalJumpVariant::Loop);
    }
    if byte == 0b11100001 {
        return Some(ConditionalJumpVariant::LoopzLoope);
    }
    if byte == 0b11100000 {
        return Some(ConditionalJumpVariant::LoopnzLoopne);
    }
    if byte == 0b11100011 {
        return Some(ConditionalJumpVariant::Jcxz);
    }
    None
}
