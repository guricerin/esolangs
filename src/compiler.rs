use anyhow::Result;

use crate::instruction::Instruction;
use crate::token::{self, Token};

pub struct Compiler {
    src_code: String,
}

static OP_CALC: &'static [Instruction] = &[
    Instruction::Add,
    Instruction::Sub,
    Instruction::Mul,
    Instruction::Div,
    Instruction::Mod,
];

static OP_OUTPUT: &'static [Instruction] = &[Instruction::NumOut, Instruction::CharOut];

static OP_INPUT: &'static [Instruction] = &[Instruction::NumIn, Instruction::CharIn];

static OP_STACK: &'static [Instruction] = &[
    Instruction::Dummy,
    Instruction::Dup,
    Instruction::Swap,
    Instruction::Rotate,
    Instruction::Pop,
];

impl Compiler {
    pub fn new(src_code: String) -> Self {
        Self { src_code: src_code }
    }

    pub fn compile(&self) -> Result<Vec<Instruction>> {
        let tokens = token::tokenize(&self.src_code)?;
        let mut insts = vec![];
        let mut nspaces = 0;
        for tok in tokens.iter() {
            match tok {
                Token::Space => {
                    nspaces += 1;
                }
                Token::Star => {
                    let op = OP_CALC[nspaces % OP_CALC.len()].clone();
                    insts.push(op);
                    nspaces = 0;
                }
                Token::Period => {
                    let op = OP_OUTPUT[nspaces % OP_OUTPUT.len()].clone();
                    insts.push(op);
                    nspaces = 0;
                }
                Token::Comma => {
                    let op = OP_INPUT[nspaces % OP_INPUT.len()].clone();
                    insts.push(op);
                    nspaces = 0;
                }
                Token::Plus => {
                    if nspaces == 0 {
                        return Err(anyhow::anyhow!("'+' needs at least one of spaces."));
                    }

                    let op = if nspaces < OP_STACK.len() {
                        OP_STACK[nspaces % OP_STACK.len()].clone()
                    } else {
                        let n = nspaces - OP_STACK.len();
                        Instruction::Push(n as i64)
                    };
                    insts.push(op);
                    nspaces = 0;
                }
                Token::BQuote => {
                    let inst = Instruction::Label(nspaces as i64);
                    insts.push(inst);
                    nspaces = 0;
                }
                Token::Quote => {
                    let inst = Instruction::JumpNonZero(nspaces as i64);
                    insts.push(inst);
                    nspaces = 0;
                }
            }
        }

        Ok(insts)
    }
}
