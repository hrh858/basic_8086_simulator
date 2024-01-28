// use std::fmt::Display;

// pub struct Sim8086Inst {

// }

// impl Sim8086Inst {
//     pub fn parse(&self) -> Self {
//         todo!()
//     }

//     fn opcode(&self) -> Sim8086Opcode {
//         match self.bytes[0] >> 4 {
//             0b1011 => Sim8086Opcode::MovImmediate,
//             _ => match (self.bytes[0] & 0b11111100) >> 2 {
//                 0b100010 => Sim8086Opcode::Mov,
//                 _ => Sim8086Opcode::Unknonw,
//             },
//         }
//     }

//     fn imm_reg(&self) -> u8 {
//         self.bytes[0] & 0b111
//     }

//     fn imm_w(&self) -> bool {
//         self.bytes[0] & 0b1 == 1
//     }

//     fn d(&self) -> bool {
//         (self.bytes[0] & 0b00000010) >> 1 == 1
//     }

//     fn w(&self) -> bool {
//         self.bytes[0] & 0b00000001 == 1
//     }

//     fn rm(&self) -> u8 {
//         self.bytes[1] & 0b111
//     }

//     fn reg(&self) -> u8 {
//         (self.bytes[1] & 0b111000) >> 3
//     }

//     fn mode(&self) -> Sim8086InstMode {
//         match (self.bytes[1] & 0b11000000) >> 6 {
//             0b00 => match self.rm() {
//                 0b110 => Sim8086InstMode::Memory {
//                     displacement: Some(Sim8086InstModeDisplacement::Bit16),
//                 },
//                 _ => Sim8086InstMode::Memory { displacement: None },
//             },
//             0b01 => Sim8086InstMode::Memory {
//                 displacement: Some(Sim8086InstModeDisplacement::Bit8),
//             },
//             0b10 => Sim8086InstMode::Memory {
//                 displacement: Some(Sim8086InstModeDisplacement::Bit16),
//             },
//             0b11 => Sim8086InstMode::Register,
//             _ => panic!("Invalid value for mode"),
//         }
//     }
// }

// impl<'a> Display for Sim8086Inst<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.opcode() {
//             Sim8086Opcode::MovImmediate => {
//                 print_imm2reg(f, Sim8086Register::from_u8(self.imm_reg(), self.imm_w()))
//             }
//             Sim8086Opcode::Mov => match self.mode() {
//                 Sim8086InstMode::Register => print_reg2reg(
//                     f,
//                     self.d(),
//                     Sim8086Register::from_u8(self.reg(), self.w()),
//                     Sim8086Register::from_u8(self.rm(), self.w()),
//                 ),
//                 Sim8086InstMode::Memory { displacement } => match displacement {
//                     Some(displacement) => match self.rm() {
//                         _ => todo!(),
//                     },
//                     None => match self.rm() {
//                         _ => todo!(),
//                     },
//                 },
//             },
//             _ => writeln!(f, "Unsupported opcode"),
//         }
//     }
// }

// fn print_reg2reg(
//     f: &mut std::fmt::Formatter<'_>,
//     d: bool,
//     reg0: Sim8086Register,
//     reg1: Sim8086Register,
// ) -> std::fmt::Result {
//     let (dest_reg, source_reg) = if d { (reg0, reg1) } else { (reg1, reg0) };
//     writeln!(f, "mov {}, {}", dest_reg, source_reg,)
// }

// fn print_imm2reg(f: &mut std::fmt::Formatter<'_>, reg: Sim8086Register) -> std::fmt::Result {
//     writeln!(f, "mov {}, {}", reg, 'Y')
// }

// pub enum Sim8086Opcode {
//     Mov,
//     MovImmediate,
//     Unknonw,
// }

// enum Sim8086InstMode {
//     Memory {
//         displacement: Option<Sim8086InstModeDisplacement>,
//     },
//     Register,
// }

// enum Sim8086InstModeDisplacement {
//     Bit8,
//     Bit16,
// }

// #[derive(Debug, PartialEq)]
// enum Sim8086Register {
//     AL,
//     CL,
//     DL,
//     BL,
//     AH,
//     CH,
//     DH,
//     BH,
//     AX,
//     CX,
//     DX,
//     BX,
//     SP,
//     BP,
//     SI,
//     DI,
// }

// impl Sim8086Register {
//     fn from_u8(reg_value: u8, w_value: bool) -> Sim8086Register {
//         if !w_value {
//             match reg_value {
//                 0b000 => Self::AL,
//                 0b001 => Self::CL,
//                 0b010 => Self::DL,
//                 0b011 => Self::BL,
//                 0b100 => Self::AH,
//                 0b101 => Self::CH,
//                 0b110 => Self::DH,
//                 0b111 => Self::BH,
//                 _ => panic!("Invalid value for register {:b}", reg_value),
//             }
//         } else {
//             match reg_value {
//                 0b000 => Self::AX,
//                 0b001 => Self::CX,
//                 0b010 => Self::DX,
//                 0b011 => Self::BX,
//                 0b100 => Self::SP,
//                 0b101 => Self::BP,
//                 0b110 => Self::SI,
//                 0b111 => Self::DI,
//                 _ => panic!("Invalid value for register {:b}", reg_value),
//             }
//         }
//     }
// }

// impl Display for Sim8086Register {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::AL => write!(f, "al"),
//             Self::CL => write!(f, "cl"),
//             Self::DL => write!(f, "dl"),
//             Self::BL => write!(f, "bl"),
//             Self::AH => write!(f, "ah"),
//             Self::CH => write!(f, "ch"),
//             Self::DH => write!(f, "dh"),
//             Self::BH => write!(f, "bh"),
//             Self::AX => write!(f, "ax"),
//             Self::CX => write!(f, "cx"),
//             Self::DX => write!(f, "dx"),
//             Self::BX => write!(f, "bx"),
//             Self::SP => write!(f, "sp"),
//             Self::BP => write!(f, "bp"),
//             Self::SI => write!(f, "si"),
//             Self::DI => write!(f, "di"),
//         }
//     }
// }
