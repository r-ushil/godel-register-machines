type RegNum = u32;
type Label = u32;
pub enum RegInstr {
    Add(RegNum, Label),
    Sub(RegNum, Label, Label),
    Halt,
}