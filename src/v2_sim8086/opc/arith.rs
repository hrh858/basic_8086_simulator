use crate::v2_sim8086::dis::Operand;

#[derive(Debug)]
pub enum ArithmeticFamily {
    Add,
    Sub,
    Cmp,
}

#[derive(Debug)]
pub enum ArithmeticVariant {
    RegMemAndRegEither(RegMemAndRegEitherStruct),
    ImmRegMem(ImmRegMemStruct),
    ImmAcc(ImmAccStruct),
}

#[derive(Debug)]
pub struct RegMemAndRegEitherStruct;
// impl RegMemAndRegEitherStruct {
//     pub fn dest_source_bytes(
//         &self,
//         b0: u8,
//         b1: u8,
//         b2: Option<&u8>,
//         b3: Option<&u8>,
//     ) -> (Option<Operand>, Option<Operand>, u8) {
//     }
// }

#[derive(Debug)]
pub struct ImmRegMemStruct;
// impl ImmRegMemStruct {
//     pub fn dest_source_bytes(
//         &self,
//         b0: u8,
//         b1: u8,
//         b2: Option<&u8>,
//         b3: Option<&u8>,
//         b4: Option<&u8>,
//         b5: Option<&u8>,
//     ) -> (Option<Operand>, Option<Operand>, u8) {
//     }
// }

#[derive(Debug)]
pub struct ImmAccStruct;
// impl ImmAccStruct {
//     pub fn dest_source_bytes(
//         &self,
//         b0: u8,
//         b1: u8,
//         b2: Option<&u8>,
//     ) -> (Option<Operand>, Option<Operand>, u8) {
//     }
// }
