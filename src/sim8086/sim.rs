use super::{
    mode::{Sim8086InstMode, Sim8086InstModeDisplacement},
    reg::Sim8086Register,
};

pub struct Sim8086<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Sim8086<'a> {
    pub fn new(program: &'a [u8]) -> Self {
        Self {
            bytes: program,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for Sim8086<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.bytes.len() {
            return None;
        }
        let (inst_str, inc) = self.opcode();
        self.cursor += inc;
        Some(inst_str)
    }
}

impl<'a> Sim8086<'a> {
    fn opcode(&mut self) -> (String, usize) {
        let first_byte = self.bytes[self.cursor];
        // Mov instructions
        return if first_byte >> 2 == 0b100010 {
            // Register/memory to/from register
            let d = Sim8086::parse_d(first_byte);
            let w = Sim8086::parse_w(first_byte, 0);
            let second_byte = self.bytes[self.cursor + 1];
            let reg = Sim8086::parse_register(second_byte, 5, w);
            let rm = Sim8086::parse_rm(second_byte);
            let mode = Sim8086::parse_mod(second_byte, rm);
            match mode {
                Sim8086InstMode::Register => {
                    // from register to register
                    let reg2 = Sim8086::parse_register(second_byte, 2, w);
                    let (dest, src) = if d { (reg, reg2) } else { (reg2, reg) };
                    let inst_str = format!("mov {}, {}", dest, src);
                    (inst_str, 2)
                }
                Sim8086InstMode::Memory { displacement } => {
                    // from register to register
                    if let Some(disp) = displacement {
                        let mut mem_str = rm_to_eac(rm);
                        let third_byte = self.bytes[self.cursor + 2];
                        match disp {
                            Sim8086InstModeDisplacement::Bit8 => {
                                if third_byte as i8 != 0 {
                                    mem_str.push_str(&format!(" + {}", third_byte as i8));
                                }
                                let (src, dest) = if d {
                                    (format!("[{}]", mem_str), reg.to_string())
                                } else {
                                    (reg.to_string(), format!("[{}]", mem_str))
                                };
                                (format!("mov {}, {}", dest, src), 3)
                            }
                            Sim8086InstModeDisplacement::Bit16 => {
                                let forth_byte = self.bytes[self.cursor + 3];
                                let val: i16 = ((forth_byte as i16) << 8) | (third_byte as i16);
                                mem_str.push_str(&format!(" + {}", val));
                                let (src, dest) = if d {
                                    (format!("[{}]", mem_str), reg.to_string())
                                } else {
                                    (reg.to_string(), format!("[{}]", mem_str))
                                };
                                (format!("mov {}, {}", dest, src), 4)
                            }
                        }
                    } else {
                        let mem_str = rm_to_eac(rm);
                        let (src, dest) = if d {
                            (format!("[{}]", mem_str), reg.to_string())
                        } else {
                            (reg.to_string(), format!("[{}]", mem_str))
                        };
                        (format!("mov {}, {}", dest, src), 2)
                    }
                }
            }
        } else if first_byte >> 1 == 0b1100011 {
            // Immediate to register/memory
            ("Unknown".to_string(), 1)
        } else if first_byte >> 4 == 0b1011 {
            // Immediate to register
            let w = Sim8086::parse_w(first_byte, 3);
            let reg = Sim8086::parse_register(first_byte, 2, w);
            let second_byte = self.bytes[self.cursor + 1];
            if !w {
                // 8 bit data
                (format!("mov {}, {}", reg, second_byte as i8), 2)
            } else {
                // 16 bit data
                let third_byte = self.bytes[self.cursor + 2];
                let val: i16 = ((third_byte as i16) << 8) | (second_byte as i16);
                (format!("mov {}, {}", reg, val), 3)
            }
        } else if first_byte & 0b1010000_0 == 0b1010000_0 {
            // Memory to accumulator
            ("Unknown".to_string(), 1)
        } else if first_byte & 0b1010001_0 == 0b1010001_0 {
            // Accumulator to memory
            ("Unknown".to_string(), 1)
        } else if first_byte & 0b10001110 == 0b10001110 {
            // Register/memory to segment register
            ("Unknown".to_string(), 1)
        } else if first_byte & 0b10001100 == 0b10001100 {
            // Segment register to register/memory
            ("Unknown".to_string(), 1)
        }
        // ADD Instructions
        else if (first_byte >> 2) == 0b000000{
            let d = Sim8086::parse_d(first_byte);
            let w = Sim8086::parse_w(first_byte, 0);
            let second_byte = self.bytes[self.cursor + 1];
            let rm = Sim8086::parse_rm(second_byte);
            let mode = Sim8086::parse_mod(second_byte, rm);
            let reg = Sim8086::parse_register(second_byte, 5, w);
            match mode {
                Sim8086InstMode::Register => {
                    // from register to register
                    let reg2 = Sim8086::parse_register(second_byte, 2, w);
                    let (dest, src) = if d { (reg, reg2) } else { (reg2, reg) };
                    let inst_str = format!("add {}, {}", dest, src);
                    (inst_str, 2)
                }
                Sim8086InstMode::Memory { displacement } => {
                    // from register to register
                    if let Some(disp) = displacement {
                        let mut mem_str = rm_to_eac(rm);
                        let third_byte = self.bytes[self.cursor + 2];
                        match disp {
                            Sim8086InstModeDisplacement::Bit8 => {
                                if third_byte as i8 != 0 {
                                    mem_str.push_str(&format!(" + {}", third_byte as i8));
                                }
                                let (src, dest) = if d {
                                    (format!("[{}]", mem_str), reg.to_string())
                                } else {
                                    (reg.to_string(), format!("[{}]", mem_str))
                                };
                                (format!("add {}, {}", dest, src), 3)
                            }
                            Sim8086InstModeDisplacement::Bit16 => {
                                let forth_byte = self.bytes[self.cursor + 3];
                                let val: i16 = ((forth_byte as i16) << 8) | (third_byte as i16);
                                mem_str.push_str(&format!(" + {}", val));
                                let (src, dest) = if d {
                                    (format!("[{}]", mem_str), reg.to_string())
                                } else {
                                    (reg.to_string(), format!("[{}]", mem_str))
                                };
                                (format!("add {}, {}", dest, src), 4)
                            }
                        }
                    } else {
                        let mem_str = rm_to_eac(rm);
                        let (src, dest) = if d {
                            (format!("[{}]", mem_str), reg.to_string())
                        } else {
                            (reg.to_string(), format!("[{}]", mem_str))
                        };
                        (format!("add {}, {}", dest, src), 2)
                    }
                }
            }
        } else if (first_byte >> 2) & 0b100000 == 0b100000 {
            let s = Sim8086::parse_s(first_byte);
            let w = Sim8086::parse_w(first_byte, 0);
            let second_byte = self.bytes[self.cursor + 1];
            let rm = Sim8086::parse_rm(second_byte);
            let mode = Sim8086::parse_mod(second_byte, rm);
            
            ("Unknown".to_string(), 1)
        } else if (first_byte >> 1) & 0b0000010 == 0b0000010 {
            let w = Sim8086::parse_w(first_byte, 0);
            let second_byte = self.bytes[self.cursor + 1];
            ("Unknown".to_string(), 1)
        }
         else {
            ("Unknown".to_string(), 1)
        };
    }

    fn parse_register(byte: u8, at: usize, w_value: bool) -> Sim8086Register {
        Sim8086Register::from_u8((byte >> (at - 2)) & 0b00000_111, w_value)
    }

    fn parse_mod(byte: u8, rm: u8) -> Sim8086InstMode {
        match (byte & 0b11_000000) >> 6 {
            0b00 => match rm {
                0b110 => Sim8086InstMode::Memory {
                    displacement: Some(Sim8086InstModeDisplacement::Bit16),
                },
                _ => Sim8086InstMode::Memory { displacement: None },
            },
            0b01 => Sim8086InstMode::Memory {
                displacement: Some(Sim8086InstModeDisplacement::Bit8),
            },
            0b10 => Sim8086InstMode::Memory {
                displacement: Some(Sim8086InstModeDisplacement::Bit16),
            },
            0b11 => Sim8086InstMode::Register,
            _ => panic!("Invalid value for mode"),
        }
    }

    fn parse_rm(byte: u8) -> u8 {
        byte & 0b00000_111
    }

    fn parse_d(byte: u8) -> bool {
        ((byte & 0b000000_1_0) >> 1) == 1
    }

    fn parse_s(byte: u8) -> bool {
        Self::parse_d(byte)
    }

    fn parse_w(byte: u8, at: usize) -> bool {
        ((byte >> at) & 0b0000000_1) == 1
    }
}

fn rm_to_eac(rm: u8) -> String {
    match rm {
        0b000 => "bx + si".to_string(),
        0b001 => "bx + di".to_string(),
        0b010 => "bp + si".to_string(),
        0b011 => "bp + di".to_string(),
        0b100 => "si".to_string(),
        0b101 => "di".to_string(),
        0b110 => "bp".to_string(),
        0b111 => "bx".to_string(),
        _ => panic!("invalid rm value"),
    }
}

fn get_arith(byte: u8, from: usize) -> String {
    match (byte >> from) & 0b111 {
        000 => "add".to_string(),
        101 => "sub".to_string(),
        111 => "cmp".to_string(),
        _ => panic!("invalid arithmetic op value")
    }
}