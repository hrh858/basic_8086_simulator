pub enum Sim8086InstMode {
    Memory {
        displacement: Option<Sim8086InstModeDisplacement>,
    },
    Register,
}

pub enum Sim8086InstModeDisplacement {
    Bit8,
    Bit16,
}
