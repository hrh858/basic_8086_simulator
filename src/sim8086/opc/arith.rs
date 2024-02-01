#[derive(Debug)]
pub enum ArithmeticFamily {
    Add,
    Sub,
    Cmp,
}

#[derive(Debug)]
pub enum ArithmeticVariant {
    RegMemAndRegEither,
    ImmRegMem,
    ImmAcc,
}
