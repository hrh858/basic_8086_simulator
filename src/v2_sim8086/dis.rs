use crate::v2_sim8086::dec::decode_d;

use super::{
    dec::{decode_mode, decode_register, decode_rm, decode_w, RegisterSize},
    opc::{
        // arith::ArithmeticVariant,
        mov::{get_operand, ImmediateValue, MoveVariant, Operand},
        parse_opcode,
        Opcode,
    },
};

pub struct Dissassembler<'a> {
    program: &'a [u8],
    cursor: usize,
}

impl<'a> Dissassembler<'a> {
    pub fn new(program: &'a [u8]) -> Self {
        Self { program, cursor: 0 }
    }
}

impl<'a> Iterator for Dissassembler<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Instruction> {
        if self.cursor >= self.program.len() {
            return None;
        };
        let (first_byte, second_byte, third_byte, forth_byte, fifth_byte, sixth_byte) = (
            self.program[self.cursor],
            self.program.get(self.cursor + 1),
            self.program.get(self.cursor + 2),
            self.program.get(self.cursor + 3),
            self.program.get(self.cursor + 4),
            self.program.get(self.cursor + 5),
        );
        let opcode = parse_opcode(first_byte);
        println!("{:?}", opcode);
        match &opcode {
            Opcode::Move { variant } => match variant {
                MoveVariant::RegMemToFromReg => {
                    let second_byte = *second_byte.unwrap();
                    let mode = decode_mode(second_byte, 7);
                    let rm = decode_rm(second_byte, 2);
                    let w = decode_w(first_byte, 0);
                    let d = decode_d(first_byte, 1);
                    let first_operand = Operand::Register(decode_register(
                        second_byte,
                        5,
                        if w == 0b1 {
                            RegisterSize::SixteenBits
                        } else {
                            RegisterSize::EightBits
                        },
                    ));
                    let (second_operand, size) =
                        get_operand(mode, rm, w, third_byte.copied(), forth_byte.copied());
                    self.cursor += size as usize;
                    let mut instruction = Instruction {
                        opcode,
                        destination: Some(first_operand),
                        source: Some(second_operand),
                        total_bytes: size,
                    };
                    if d == 0b0 {
                        instruction = swap_operands(instruction)
                    }
                    Some(instruction)
                }
                MoveVariant::ImmToReg => {
                    let second_byte = *second_byte.unwrap();
                    let w = decode_w(first_byte, 3);
                    let size = if w == 0b1 { 3 } else { 2 };
                    let first_operand = Operand::Register(decode_register(
                        first_byte,
                        2,
                        if w == 0b1 {
                            RegisterSize::SixteenBits
                        } else {
                            RegisterSize::EightBits
                        },
                    ));
                    let second_operand = Operand::ImmediateValue(if w == 0b1 {
                        let third_byte = *third_byte.unwrap();
                        ImmediateValue::SixteenBits(((third_byte as i16) << 8) | second_byte as i16)
                    } else {
                        ImmediateValue::EightBits(second_byte as i8)
                    });
                    self.cursor += size as usize;
                    Some(Instruction {
                        opcode,
                        destination: Some(first_operand),
                        source: Some(second_operand),
                        total_bytes: if w == 0b1 { 3 } else { 2 },
                    })
                }
                _ => todo!(),
            },
            _ => todo!(),
            //
            // Opcode::Arithmetic { variant, family } => {
            // 	match variant {
            // 		ArithmeticVariant::RegMemAndRegEither(_) => (None, None, 2),
            // 		ArithmeticVariant::ImmRegMem(_) => (None, None, 2),
            // 		ArithmeticVariant::ImmAcc(_) => (None, None, 2),
            // 	}
            // },
            // Opcode::ConditionalJump { variant } => {
            // 	let operand = Operand::ImmediateByte(*second_byte.unwrap() as i8);
            // 	(Some(operand), None, 2)
            // },
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub destination: Option<Operand>,
    pub source: Option<Operand>,
    pub total_bytes: u8,
}

impl Instruction {
    fn new(
        opcode: Opcode,
        destination: Option<Operand>,
        source: Option<Operand>,
        total_bytes: u8,
    ) -> Self {
        Self {
            opcode,
            destination,
            source,
            total_bytes,
        }
    }
}

fn swap_operands(instruction: Instruction) -> Instruction {
    Instruction {
        opcode: instruction.opcode,
        destination: instruction.source,
        source: instruction.destination,
        total_bytes: instruction.total_bytes,
    }
}
