use core::panic;

use super::{
    dis::Instruction,
    opc::{
        mov::{ImmediateValue, MoveVariant, Operand, Register},
        Opcode,
    },
};

#[derive(Debug)]
pub struct Emulator {
    registers: EmulatorRegisters,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            registers: EmulatorRegisters::new(),
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
                                _ => panic!(""),
                            },
                        _ => todo!()
                    }
                    _ => todo!(),
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
