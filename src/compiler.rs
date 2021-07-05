use anyhow::Result;

use crate::instruction::Instruction;
use crate::token::{self, Token};

#[derive(Debug)]
pub struct Compiler {
    src_code: String,
}

impl Compiler {
    pub fn new(src_code: String) -> Self {
        Self { src_code: src_code }
    }

    pub fn compile(&self) -> Result<Vec<Instruction>> {
        let tokens = token::tokenize(&self.src_code)?;
        let mut pos = 0;
        let mut insts = vec![];

        macro_rules! parse {
            ($lexpr:expr) => {{
                let (inst, p) = $lexpr?;
                insts.push(inst);
                pos = p;
            }};
        }

        while pos < tokens.len() {
            match tokens[pos] {
                Token::Space => parse!(Self::p_s(pos + 1, &tokens)),
                Token::Tab => parse!(Self::p_t(pos + 1, &tokens)),
                Token::Lf => parse!(Self::p_l(pos + 1, &tokens)),
            }
        }

        Ok(insts)
    }

    fn p_s(pos: usize, tokens: &[Token]) -> Result<(Instruction, usize)> {
        match tokens[pos] {
            // SS
            Token::Space => {
                let (n, p) = Self::p_num(pos + 1, tokens)?;
                Ok((Instruction::Push(n), p))
            }
            Token::Tab => {
                let (inst, p) = match tokens[pos + 1] {
                    // STS n
                    Token::Space => {
                        let (n, p) = Self::p_num(pos + 2, tokens)?;
                        (Instruction::Copy(n), p)
                    }
                    // STL n
                    Token::Lf => {
                        let (n, p) = Self::p_num(pos + 2, tokens)?;
                        (Instruction::Slide(n), p)
                    }
                    // STT
                    Token::Tab => return Err(anyhow::anyhow!("[STT] is grammar error.")),
                };
                Ok((inst, p))
            }
            Token::Lf => {
                let inst = match tokens[pos + 1] {
                    // SLS
                    Token::Space => Instruction::Dup,
                    // SLT
                    Token::Tab => Instruction::Swap,
                    // SLL
                    Token::Lf => Instruction::Discard,
                };
                Ok((inst, pos + 2))
            }
        }
    }

    fn p_num(pos: usize, tokens: &[Token]) -> Result<(i64, usize)> {
        let mut bin = String::new();
        match tokens[pos] {
            Token::Space => bin.push('+'),
            Token::Tab => bin.push('-'),
            Token::Lf => {
                return Err(anyhow::anyhow!(
                    "numbers must start with space or tab at least one."
                ))
            }
        }

        let mut pos = pos + 1;
        while pos < tokens.len() {
            match tokens[pos] {
                Token::Space => bin.push('0'),
                Token::Tab => bin.push('1'),
                Token::Lf => {
                    pos += 1;
                    break;
                }
            }
            pos += 1;
        }
        let n = i64::from_str_radix(&bin, 2)?;
        Ok((n, pos))
    }

    fn p_t(pos: usize, tokens: &[Token]) -> Result<(Instruction, usize)> {
        match tokens[pos] {
            Token::Space => Self::p_ts(pos + 1, tokens),
            Token::Tab => Self::p_tt(pos + 1, tokens),
            Token::Lf => Self::p_tl(pos + 1, tokens),
        }
    }

    fn p_ts(pos: usize, tokens: &[Token]) -> Result<(Instruction, usize)> {
        let inst = match tokens[pos] {
            Token::Space => match tokens[pos + 1] {
                // TSSS
                Token::Space => Instruction::Add,
                // TSST
                Token::Tab => Instruction::Sub,
                // TSSL
                Token::Lf => Instruction::Mul,
            },
            Token::Tab => match tokens[pos + 1] {
                // TSTS
                Token::Space => Instruction::Div,
                // TSTT
                Token::Tab => Instruction::Mod,
                Token::Lf => return Err(anyhow::anyhow!("TSTL is grammar error.")),
            },
            Token::Lf => return Err(anyhow::anyhow!("TSL is grammar error.")),
        };
        Ok((inst, pos + 2))
    }

    fn p_tt(pos: usize, tokens: &[Token]) -> Result<(Instruction, usize)> {
        let inst = match tokens[pos] {
            // TTS
            Token::Space => Instruction::HeapWrite,
            // TTT
            Token::Tab => Instruction::HeapRead,
            Token::Lf => return Err(anyhow::anyhow!("TTL is grammar error.")),
        };
        Ok((inst, pos + 1))
    }

    fn p_tl(pos: usize, tokens: &[Token]) -> Result<(Instruction, usize)> {
        let inst = match tokens[pos] {
            Token::Space => match tokens[pos + 1] {
                // TLSS
                Token::Space => Instruction::CharOut,
                // TLST
                Token::Tab => Instruction::NumOut,
                Token::Lf => return Err(anyhow::anyhow!("TLSL is grammar error.")),
            },
            Token::Tab => match tokens[pos + 1] {
                // TLTS
                Token::Space => Instruction::CharIn,
                // TLTT
                Token::Tab => Instruction::NumIn,
                Token::Lf => return Err(anyhow::anyhow!("TLTL is grammar error.")),
            },
            Token::Lf => return Err(anyhow::anyhow!("TLL is grammar error.")),
        };
        Ok((inst, pos + 2))
    }

    fn p_l(pos: usize, tokens: &[Token]) -> Result<(Instruction, usize)> {
        match tokens[pos] {
            Token::Space => {
                let (inst, p) = match tokens[pos + 1] {
                    // LSS l
                    Token::Space => {
                        let (label, p) = Self::p_label(pos + 2, tokens)?;
                        (Instruction::Label(label), p)
                    }
                    // LST l
                    Token::Tab => {
                        let (label, p) = Self::p_label(pos + 2, tokens)?;
                        (Instruction::Call(label), p)
                    }
                    // LSL l
                    Token::Lf => {
                        let (label, p) = Self::p_label(pos + 2, tokens)?;
                        (Instruction::Jump(label), p)
                    }
                };
                Ok((inst, p))
            }
            Token::Tab => {
                let (inst, p) = match tokens[pos + 1] {
                    // LTS l
                    Token::Space => {
                        let (label, p) = Self::p_label(pos + 2, tokens)?;
                        (Instruction::JumpZero(label), p)
                    }
                    // LTT l
                    Token::Tab => {
                        let (label, p) = Self::p_label(pos + 2, tokens)?;
                        (Instruction::JumpNeg(label), p)
                    }
                    // LTL
                    Token::Lf => (Instruction::Return, pos + 2),
                };
                Ok((inst, p))
            }
            Token::Lf => {
                let inst = match tokens[pos + 1] {
                    // LLL
                    Token::Lf => Instruction::Exit,
                    _ => return Err(anyhow::anyhow!("LLS and LLT are grammar error.")),
                };
                Ok((inst, pos + 2))
            }
        }
    }

    fn p_label(pos: usize, tokens: &[Token]) -> Result<(String, usize)> {
        match tokens[pos] {
            Token::Lf => {
                return Err(anyhow::anyhow!(
                    "labels must start with space or tag at least one."
                ))
            }
            _ => (),
        }

        let mut label = String::new();
        let mut pos = pos + 1;
        while pos < tokens.len() {
            match tokens[pos] {
                Token::Space => label.push('s'),
                Token::Tab => label.push('t'),
                Token::Lf => {
                    pos += 1;
                    break;
                }
            }
            pos += 1;
        }
        Ok((label, pos))
    }
}
