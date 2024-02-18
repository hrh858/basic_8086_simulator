use core::fmt;

use crate::sim8086::opc::cond_jump::ConditionalJumpVariant;

use super::{
    dis::Instruction,
    opc::{
        arith::ArithmeticFamily,
        mov::{ImmediateValue, MoveVariant, Operand, Register},
        Opcode,
    },
};

pub struct Emulator {
    registers: EmulatorRegisters,
    flags: EmulatorFlags,
    pub instruction_pointer: u16,
    memory: [u8; 65536],
}

impl fmt::Debug for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Emulator")
            .field("registers", &self.registers)
            .field("flags", &self.flags)
            .field("ip", &self.instruction_pointer)
            .field("memory", &self.memory.iter().enumerate().filter(|(_, b)| **b != 0).collect::<Vec<(usize, &u8)>>())
            .finish()
    }
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            flags: EmulatorFlags::new(),
            registers: EmulatorRegisters::new(),
            instruction_pointer: 0,
            memory: [0; 65536],
        }
    }

    fn load_from_memory(&self, from: usize) -> u16 {
        let first_byte = self.memory[from];
        let second_byte = self.memory[from + 1];
        ((second_byte as u16) << 8) | (first_byte as u16)
    }

    fn store_into_memory(&mut self, to: usize, value: u16) {
        self.memory[to] = ((value & 0xFF00) >> 8) as u8;
        self.memory[to + 1] = (value & 0x00FF) as u8;
    }

    fn get_operand_value(&self, operand: &Operand) -> i16 {
        match operand {
            Operand::ImmediateValue(imm_val) => match imm_val {
                ImmediateValue::SixteenBits(val) => *val,
                ImmediateValue::EightBits(val) => *val as i16,
            },
            Operand::Register(reg) => match reg {
                Register::AL => self.registers.reg_a.get_low() as i16,
                Register::BL => self.registers.reg_b.get_low() as i16,
                Register::CL => self.registers.reg_c.get_low() as i16,
                Register::DL => self.registers.reg_d.get_low() as i16,
                Register::AH => self.registers.reg_a.get_high() as i16,
                Register::BH => self.registers.reg_b.get_low() as i16,
                Register::CH => self.registers.reg_c.get_high() as i16,
                Register::DH => self.registers.reg_d.get_high() as i16,
                Register::AX => self.registers.reg_a.get(),
                Register::BX => self.registers.reg_b.get(),
                Register::CX => self.registers.reg_c.get(),
                Register::DX => self.registers.reg_d.get(),
                Register::SP => self.registers.reg_sp.get(),
                Register::BP => self.registers.reg_bp.get(),
                Register::SI => self.registers.reg_si.get(),
                Register::DI => self.registers.reg_di.get(),
            },
            _ => todo!(),
        }
    }

    fn get_register_low(&self, register: Register) -> i8 {
        match register {
            Register::AL => self.registers.reg_a.get_low(),
            Register::BL => self.registers.reg_b.get_low(),
            Register::CL => self.registers.reg_c.get_low(),
            Register::DL => self.registers.reg_d.get_low(),
            _ => panic!("Can't get low part of {:?}", register),
        }
    }
    fn get_register_high(&self, register: Register) -> i8 {
        match register {
            Register::AH => self.registers.reg_a.get_high(),
            Register::BH => self.registers.reg_b.get_high(),
            Register::CH => self.registers.reg_c.get_high(),
            Register::DH => self.registers.reg_d.get_high(),
            _ => panic!("Can't get high part of {:?}", register),
        }
    }
    fn get_register(&self, register: Register) -> i16 {
        match register {
            Register::AX => self.registers.reg_a.get(),
            Register::BX => self.registers.reg_b.get(),
            Register::CX => self.registers.reg_c.get(),
            Register::DX => self.registers.reg_d.get(),
            Register::SP => self.registers.reg_sp.get(),
            Register::BP => self.registers.reg_bp.get(),
            Register::DI => self.registers.reg_di.get(),
            Register::SI => self.registers.reg_si.get(),
            _ => panic!("Can't get whole register value for {:?}", register),
        }
    }
    fn set_register_low(&mut self, register: Register, value: i8) {
        match register {
            Register::AL => self.registers.reg_a.set_low(value),
            Register::BL => self.registers.reg_b.set_low(value),
            Register::CL => self.registers.reg_c.set_low(value),
            Register::DL => self.registers.reg_d.set_low(value),
            _ => panic!("Low part of {:?} can't be set", register),
        }
    }

    fn set_register_high(&mut self, register: Register, value: i8) {
        match register {
            Register::AH => self.registers.reg_a.set_high(value),
            Register::BH => self.registers.reg_b.set_high(value),
            Register::CH => self.registers.reg_c.set_high(value),
            Register::DH => self.registers.reg_d.set_high(value),
            _ => panic!("High part of {:?} can't be set", register),
        }
    }

    fn set_register(&mut self, register: Register, value: i16) {
        match register {
            Register::AX => self.registers.reg_a.set(value),
            Register::BX => self.registers.reg_b.set(value),
            Register::CX => self.registers.reg_c.set(value),
            Register::DX => self.registers.reg_d.set(value),
            Register::SP => self.registers.reg_sp.set(value),
            Register::BP => self.registers.reg_bp.set(value),
            Register::DI => self.registers.reg_di.set(value),
            Register::SI => self.registers.reg_si.set(value),
            _ => panic!("Whole register value for {:?} can't be set", register),
        }
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        self.instruction_pointer += instruction.total_bytes as u16;
        match &instruction.opcode {
            Opcode::Move { variant } => {
                let source = instruction.source.as_ref().unwrap();
                let dest = instruction.destination.as_ref().unwrap();

                match variant {
                    MoveVariant::ImmToReg => match (dest, source) {
                        (
                            Operand::Register(reg),
                            Operand::ImmediateValue(ImmediateValue::SixteenBits(val)),
                        ) => self.set_register(*reg, *val),
                        (
                            Operand::Register(reg),
                            Operand::ImmediateValue(ImmediateValue::EightBits(val)),
                        ) => self.set_register_low(*reg, *val),
                        _ => panic!("This move variant expects a register and an immediate value, but a {:?} and a {:?} were received", dest, source),
                    },
                    MoveVariant::RegMemToFromReg => match (dest, source) {
                        (Operand::Register(dest_reg), Operand::Register(source_reg)) =>
                            match (dest_reg, source_reg) {
                                (Register::AH|Register::BH|Register::CH|Register::DH, Register::AH|Register::BH|Register::CH|Register::DH) => {self.set_register_low(*dest_reg, self.get_register_low(*source_reg))}
                                (Register::AL|Register::BL|Register::CL|Register::DL, Register::AL|Register::BL|Register::CL|Register::DL) => {self.set_register_high(*dest_reg, self.get_register_high(*source_reg))}
                                (Register::AX|Register::BX|Register::CX|Register::DX|Register::SP|Register::BP|Register::SI|Register::DI, Register::AX|Register::BX|Register::CX|Register::DX|Register::SP|Register::BP|Register::SI|Register::DI) => {self.set_register(*dest_reg, self.get_register(*source_reg))}
                                _ => todo!("I think there may be cases where you want to move the high part of a register into the low part of another or viceversa... right? Well, I will implement it later if necessary hehe"),
                            },
                        _ => todo!()
                    },
                    MoveVariant::ImmToRegMem => match (dest, source) {
                        (Operand::Address(addr), Operand::ImmediateValue(ImmediateValue::SixteenBits(val))) => {
                            self.store_into_memory(*addr as usize, *val as u16);
                        },
                         _ => todo!()
                    },
                    _ => todo!(),
                }
            }
            Opcode::Arithmetic { family, .. } => {
                let source = instruction.source.as_ref().unwrap();
                let dest = instruction.destination.as_ref().unwrap();
                let source_val = self.get_operand_value(source);
                let dest_val = self.get_operand_value(dest);
                let result = match family {
                    ArithmeticFamily::Add => {
                        let result = dest_val + source_val;
                        self.flags.sign = (result as u16 & 0x8000) == 0x8000;
                        self.flags.zero = result == 0;
                        result
                    }
                    ArithmeticFamily::Sub => {
                        let result = dest_val - source_val;
                        self.flags.sign = (result as u16 & 0x8000) == 0x8000;
                        self.flags.zero = result == 0;
                        result
                    }
                    ArithmeticFamily::Cmp => {
                        let result = dest_val - source_val;
                        self.flags.sign = (result as u16 & 0x8000) == 0x8000;
                        self.flags.zero = result == 0;
                        dest_val
                    }
                };
                match dest {
                    Operand::Register(reg) => match reg {
                        Register::AL | Register::AH | Register::AX => {
                            self.registers.reg_a.set(result)
                        }
                        Register::BL | Register::BH | Register::BX => {
                            self.registers.reg_b.set(result)
                        }
                        Register::CL | Register::CH | Register::CX => {
                            self.registers.reg_c.set(result)
                        }
                        Register::DL | Register::DH | Register::DX => {
                            self.registers.reg_d.set(result)
                        }
                        Register::SI => self.registers.reg_si.set(result),
                        Register::DI => self.registers.reg_di.set(result),
                        Register::SP => self.registers.reg_sp.set(result),
                        Register::BP => self.registers.reg_bp.set(result),
                    },
                    _ => todo!(),
                }
            }
            Opcode::ConditionalJump { variant } => {
                // self.instruction_pointer = ???
                match variant {
                    ConditionalJumpVariant::JeJz => todo!(),
                    ConditionalJumpVariant::JlJnge => todo!(),
                    ConditionalJumpVariant::JleJng => todo!(),
                    ConditionalJumpVariant::JbJnae => todo!(),
                    ConditionalJumpVariant::JbeJna => todo!(),
                    ConditionalJumpVariant::JpJpe => todo!(),
                    ConditionalJumpVariant::Jo => todo!(),
                    ConditionalJumpVariant::Js => todo!(),
                    ConditionalJumpVariant::JneJnz => {
                        let jump_to = instruction.destination.as_ref().unwrap();
                        if let Operand::ImmediateValue(imm_val) = jump_to {
                            let offset = match imm_val {
                                ImmediateValue::EightBits(offset) => *offset as i16,
                                ImmediateValue::SixteenBits(offset) => *offset as i16,
                            };
                            self.instruction_pointer =
                                ((self.instruction_pointer as i16) + offset) as u16;
                        }
                    }
                    ConditionalJumpVariant::JnlJge => todo!(),
                    ConditionalJumpVariant::JnleJg => todo!(),
                    ConditionalJumpVariant::JnbJae => todo!(),
                    ConditionalJumpVariant::JnbeJa => todo!(),
                    ConditionalJumpVariant::JnpJpo => todo!(),
                    ConditionalJumpVariant::Jno => todo!(),
                    ConditionalJumpVariant::Jns => todo!(),
                    ConditionalJumpVariant::Loop => todo!(),
                    ConditionalJumpVariant::LoopzLoope => todo!(),
                    ConditionalJumpVariant::LoopnzLoopne => todo!(),
                    ConditionalJumpVariant::Jcxz => todo!(),
                }
            }
            _ => todo!(),
        };
    }
}

#[derive(Debug)]
pub struct EmulatorRegisters {
    reg_a: GeneralRegister,
    reg_b: GeneralRegister,
    reg_c: GeneralRegister,
    reg_d: GeneralRegister,
    reg_sp: SpecialRegister,
    reg_bp: SpecialRegister,
    reg_si: SpecialRegister,
    reg_di: SpecialRegister,
}

impl EmulatorRegisters {
    pub fn new() -> Self {
        EmulatorRegisters {
            reg_a: GeneralRegister(0),
            reg_b: GeneralRegister(0),
            reg_c: GeneralRegister(0),
            reg_d: GeneralRegister(0),
            reg_sp: SpecialRegister(0),
            reg_bp: SpecialRegister(0),
            reg_si: SpecialRegister(0),
            reg_di: SpecialRegister(0),
        }
    }
}

#[derive(Debug)]
pub struct EmulatorFlags {
    zero: bool,
    sign: bool,
}

impl EmulatorFlags {
    fn new() -> Self {
        Self {
            zero: false,
            sign: false,
        }
    }
}

#[derive(Debug)]
pub struct GeneralRegister(i16);
impl GeneralRegister {
    pub fn get_high(&self) -> i8 {
        ((self.0 as u16 & 0xFF00) >> 4) as i8
    }
    pub fn get_low(&self) -> i8 {
        ((self.0 as u16 & 0x00FF) >> 4) as i8
    }
    pub fn get(&self) -> i16 {
        self.0
    }
    pub fn set_high(&mut self, value: i8) {
        self.0 = (((value as u16) << 8) | ((self.0 as u16) & 0x00FF)) as i16
    }
    pub fn set_low(&mut self, value: i8) {
        self.0 = ((value as u16) | ((self.0 as u16) & 0xFF00)) as i16
    }
    pub fn set(&mut self, value: i16) {
        self.0 = value
    }
}

#[derive(Debug)]
pub struct SpecialRegister(i16);
impl SpecialRegister {
    pub fn get(&self) -> i16 {
        self.0
    }
    pub fn set(&mut self, value: i16) {
        self.0 = value
    }
}

#[derive(Debug)]
pub struct FlagsRegister(u16);
impl FlagsRegister {
    fn zero(&self) -> bool {
        false
    }
    fn set_zero(&mut self, value: bool) {}

    fn negative(&self) -> bool {
        false
    }
    fn set_negative(&mut self, value: bool) {}
}
