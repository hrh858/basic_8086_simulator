#[derive(Debug)]
pub enum ConditionalJumpVariant {
    JeJz,
    JlJnge,
    JleJng,
    JbJnae,
    JbeJna,
    JpJpe,
    Jo,
    Js,
    JneJnz,
    JnlJge,
    JnleJg,
    JnbJae,
    JnbeJa,
    JnpJpo,
    Jno,
}