use super::{
    dec::Reg, opc::{parse_opcode, Opcode, mov::MoveVariant, arith::ArithmeticVariant},
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
        if self.cursor >= self.program.len() { return None };
        let (first_byte, second_byte, third_byte, forth_byte, fifth_byte, sixth_byte) = (
            self.program[self.cursor],
            self.program.get(self.cursor + 1),
            self.program.get(self.cursor + 2),
            self.program.get(self.cursor + 3),
            self.program.get(self.cursor + 4),
            self.program.get(self.cursor + 5),
        );
        let opcode = parse_opcode(first_byte);
        let (dest, source, total_bytes) = match &opcode {
			Opcode::Move { variant } => {
				match variant {
					MoveVariant::RegMemToFromReg(v) => v.dest_source_bytes(first_byte, *second_byte.unwrap(), third_byte, forth_byte),
					MoveVariant::ImmToRegMem(_) => todo!(),
					MoveVariant::ImmToReg(v) => v.dest_source_bytes(first_byte, *second_byte.unwrap(), third_byte),
					MoveVariant::MemToAcc(_) => (None, None, 2),
					MoveVariant::AccToMem(_) => (None, None, 2),
				}
			},
			Opcode::Arithmetic { variant, family } => {
				match variant {
					ArithmeticVariant::RegMemAndRegEither(_) => (None, None, 2),
					ArithmeticVariant::ImmRegMem(_) => (None, None, 2),
					ArithmeticVariant::ImmAcc(_) => (None, None, 2),
				}
			},
			Opcode::ConditionalJump { variant } => {
				let operand = Operand::ImmediateByte(*second_byte.unwrap() as i8);
				(Some(operand), None, 2)
			},
			Opcode::NotImplemented => panic!("This opcode is not supported and continuing the parsing may corrupt the following instructions"),
		};
        self.cursor += total_bytes as usize;
        Some(Instruction::new(opcode, dest, source, total_bytes))
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

    fn not_implemented(total_bytes: u8) -> Self {
        Self {
            total_bytes,
            opcode: Opcode::NotImplemented,
            destination: None,
            source: None,
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Register(Reg),
    ImmediateByte(i8),
    ImmediateWord(i16),
    EffectiveAddress(EffectiveAddress),
    Address(u16),
}

#[derive(Debug)]
pub enum EffectiveAddress {
    NoDisp,
    Disp(Displacement),
}

#[derive(Debug)]
pub enum Displacement {
    Byte(u8),
    Word(u16),
}
