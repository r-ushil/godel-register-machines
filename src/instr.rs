use std::fmt::Display;

type RegNum = u32;
type Label = u32;
pub enum RegInstr {
    Add(RegNum, Label),
    Sub(RegNum, Label, Label),
    Halt,
}

impl Display for RegInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegInstr::Add(i, j) => write!(f, "R{}+ -> L{}", i, j),
            RegInstr::Sub(i, j, k) => write!(f, "R{}- -> L{}, L{}", i, j, k),
            RegInstr::Halt => write!(f, "HALT"),
        }
    }
}