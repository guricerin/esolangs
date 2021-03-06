#[derive(Debug, Clone)]
pub enum Instruction {
    Push(i64),
    Dup,
    Copy(i64),
    Swap,
    Discard,
    Slide(i64),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    HeapWrite,
    HeapRead,
    Label(String),
    Call(String),
    Jump(String),
    JumpZero(String),
    JumpNeg(String),
    Return,
    Exit,
    CharOut,
    NumOut,
    CharIn,
    NumIn,
}
