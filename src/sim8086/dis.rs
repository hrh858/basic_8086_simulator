use crate::sim8086::opc::{arith::ArithmeticVariant, mov::Register};

use super::opc::{
    mov::{get_operand, ImmediateValue, MoveVariant, Operand},
    parse_opcode, Opcode,
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

impl<'a> Dissassembler<'a> {
    pub fn get_instruction_at(&mut self, position: usize) -> Option<Instruction> {
if position >= self.program.len() {
            return None;
        };
        let (first_byte, second_byte, third_byte, forth_byte, fifth_byte, sixth_byte) = (
            self.program[position],
            self.program.get(position + 1),
            self.program.get(position + 2),
            self.program.get(position + 3),
            self.program.get(position + 4),
            self.program.get(position + 5),
        );
        let opcode = parse_opcode(first_byte, *second_byte.unwrap());
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
            Opcode::Arithmetic { variant, .. } => match variant {
                ArithmeticVariant::RegMemAndRegEither => {
                    let second_byte = *second_byte.unwrap();
                    let d = decode_d(first_byte, 1);
                    let w = decode_w(first_byte, 0);
                    let mode = decode_mode(second_byte, 7);
                    let rm = decode_rm(second_byte, 2);

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
                ArithmeticVariant::ImmAcc => {
                    let w = decode_w(first_byte, 0);
                    let second_byte = *second_byte.unwrap();
                    let first_operand =
                        Operand::Register(if w == 0b1 { Register::AX } else { Register::AL });
                    let size: u8;
                    let second_operand = Operand::ImmediateValue(if w == 0b1 {
                        let third_byte = *third_byte.unwrap();
                        size = 3;
                        ImmediateValue::SixteenBits(
                            ((third_byte as i16) << 8) | (second_byte as i16),
                        )
                    } else {
                        size = 2;
                        ImmediateValue::EightBits(second_byte as i8)
                    });
                    self.cursor += size as usize;
                    Some(Instruction {
                        opcode,
                        destination: Some(first_operand),
                        source: Some(second_operand),
                        total_bytes: size,
                    })
                }
                ArithmeticVariant::ImmRegMem => {
                    let second_byte = *second_byte.unwrap();
                    let s = decode_s(first_byte, 1);
                    let w = decode_w(first_byte, 0);
                    let mode = decode_mode(second_byte, 7);
                    let rm = decode_rm(second_byte, 2);
                    let (first_operand, mut size) =
                        get_operand(mode, rm, w, third_byte.copied(), forth_byte.copied());
                    let second_operand = if s == 0b0 && w == 0b1 {
                        // let fifth_byte = *fifth_byte.unwrap();
                        // let sixth_byte = *sixth_byte.unwrap();
                        // size += 2;
                        // Operand::ImmediateValue(ImmediateValue::SixteenBits(
                        //     ((sixth_byte as i16) << 8) | fifth_byte as i16,
                        // ))
                        let third_byte = *third_byte.unwrap();
                        let forth_byte = *forth_byte.unwrap();
                        size += 2;
                        Operand::ImmediateValue(ImmediateValue::SixteenBits(
                            ((forth_byte as i16) << 8) | (third_byte as i16),
                        ))
                    } else {
                        let third_byte = *third_byte.unwrap();
                        size += 1;
                        Operand::ImmediateValue(ImmediateValue::EightBits(third_byte as i8))
                    };
                    let instruction = Instruction {
                        opcode,
                        destination: Some(first_operand),
                        source: Some(second_operand),
                        total_bytes: size,
                    };
                    self.cursor += size as usize;
                    Some(instruction)
                }
            },
            Opcode::ConditionalJump { .. } => {
                self.cursor += 2;
                Some(Instruction {
                    opcode,
                    destination: Some(Operand::ImmediateValue(ImmediateValue::EightBits(
                        *second_byte.unwrap() as i8,
                    ))),
                    source: None,
                    total_bytes: 2,
                })
            }
            _ => todo!(),
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

fn swap_operands(instruction: Instruction) -> Instruction {
    Instruction {
        opcode: instruction.opcode,
        destination: instruction.source,
        source: instruction.destination,
        total_bytes: instruction.total_bytes,
    }
}

pub fn decode_w(byte: u8, at: u8) -> u8 {
    (byte >> at) & 0b1
}

pub fn decode_d(byte: u8, at: u8) -> u8 {
    (byte >> at) & 0b1
}

pub fn decode_s(byte: u8, at: u8) -> u8 {
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
