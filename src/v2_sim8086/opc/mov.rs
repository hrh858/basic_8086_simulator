use crate::v2_sim8086::{dis::Operand, dec::{decode_d, decode_w, W, decode_mod, decode_rm, Mode, decode_reg, D}};

#[derive(Debug)]
pub enum MoveVariant {
    RegMemToFromReg(RegMemToFromRegStruct),
    ImmToRegMem(ImmToRegMemStruct),
    ImmToReg(ImmToRegStruct),
    MemToAcc(MemToAccStruct),
    AccToMem(AccToMemStruct),
}

// (dest, source, total_bytes)
#[derive(Debug)]
pub struct RegMemToFromRegStruct;
impl RegMemToFromRegStruct {
    pub fn dest_source_bytes(
        &self,
        b0: u8,
        b1: u8,
        b2: Option<&u8>,
        b3: Option<&u8>,
    ) -> (Option<Operand>, Option<Operand>, u8) {
    	let w = decode_w(b0, 0);
    	let d = decode_d(b0, 1);
    	let mode = decode_mod(b1, 7);
    	let reg = decode_reg(b1, 5, w);
    	match mode {
    		Mode::RegisterNoDisp => {
    			let reg2 = decode_reg(b1, 2, w);
    			match d {
    				D::RegIsSrc =>{
    					(Some(Operand::Register(reg2)), Some(Operand::Register(reg)), 2)
    				},
    				D::RegIsDst  => {
    					(Some(Operand::Register(reg)), Some(Operand::Register(reg2)), 2)
    				}
    			}
    		},
    		_ => (None, None, 2)
    	}
    }
}

#[derive(Debug)]
pub struct ImmToRegMemStruct;
// impl ImmToRegMemStruct {
//     pub fn dest_source_bytes(
//         &self,
//         b0: u8,
//         b1: u8,
//         b2: u8,
//         b3: u8,
//         b4: Option<&u8>,
//         b5: Option<&u8>,
//     ) -> (Option<Operand>, Option<Operand>, u8) {
//     	let w = decode_w(b0, 0);
//     }
// }

#[derive(Debug)]
pub struct ImmToRegStruct;
impl ImmToRegStruct {
    pub fn dest_source_bytes(
        &self,
        b0: u8,
        b1: u8,
        b2: Option<&u8>,
    ) -> (Option<Operand>, Option<Operand>, u8) {
    	let w = decode_w(b0, 3);
        let reg = decode_reg(b0, 2, w);
        match w {
            W::Byte => {
                let val = b1 as i8;
                (Some(Operand::Register(reg)), Some(Operand::ImmediateByte(val)), 2)
            },
            W::Word => {
                let b2 = *b2.unwrap() as i16;
                let val = ((b2 as i16) << 8) | b1 as i16;
                (Some(Operand::Register(reg)), Some(Operand::ImmediateWord(val)), 3)
            },
        }
    }
}

#[derive(Debug)]
pub struct MemToAccStruct;
// impl MemToAccStruct {
//     pub fn dest_source_bytes(
//         &self,
//         b0: u8,
//         b1: u8,
//         b2: Option<&u8>,
//     ) -> (Option<Operand>, Option<Operand>, u8) {
//     	let w = decode_w(b0, 0);
//     }
// }

#[derive(Debug)]
pub struct AccToMemStruct;
// impl AccToMemStruct {
//     pub fn dest_source_bytes(
//         &self,
//         b0: u8,
//         b1: u8,
//         b2: u8,
//         b3: Option<&u8>,
//     ) -> (Option<Operand>, Option<Operand>, u8) {
//     	let w = decode_w(b0, 0);
//     }
// }
