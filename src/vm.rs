use std::collections::HashMap;

use anyhow::{self, Context, Result};

use crate::instruction::Instruction;

#[derive(Debug)]
pub struct VM {
    insts: Vec<Instruction>,
    stack: Vec<i32>,
    /// K: address, V:
    heap: HashMap<u64, i32>,
    /// K: label, V: position
    labels: HashMap<String, usize>,
}

impl VM {
    pub fn new(insts: Vec<Instruction>) -> Self {
        let labels = Self::find_labels(&insts);
        Self {
            insts: insts,
            stack: Vec::new(),
            heap: HashMap::new(),
            labels: labels,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut pc = 0;
        while pc < self.insts.len() {
            let inst = &self.insts[pc];
            match inst {
                &Instruction::Push(n) => {
                    self.stack.push(n);
                }
                &Instruction::Dup => {
                    // if let Some(n) = self.stack.last() {
                    //     self.stack.push(*n);
                    // }
                    let n = self.stack[self.stack.len() - 1];
                    self.stack.push(n);
                }
                &Instruction::Copy(n) => {
                    // ケツからn番目（0 indexed）
                    let v = self.stack[self.stack.len() - (n as usize + 1)];
                    self.stack.push(v);
                }
                &Instruction::Swap => {
                    let x = self.pop()?;
                    let y = self.pop()?;
                    self.stack.push(x);
                    self.stack.push(y);
                }
                &Instruction::Discard => {
                    let _ = self.stack.pop();
                }
                &Instruction::Slide(n) => {
                    let x = self.pop()?;
                    for _ in 0..(n as usize) {
                        self.pop()?;
                    }
                    self.stack.push(x);
                }
                &Instruction::Add => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l + r);
                }
                &Instruction::Sub => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l - r);
                }
                &Instruction::Mul => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l * r);
                }
                &Instruction::Div => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l / r);
                }
                &Instruction::Mod => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l % r);
                }
                &Instruction::MemWrite => {
                    let value = self.pop()?;
                    let address = self.pop()?;
                }
                &Instruction::MemRead => todo!(),
                &Instruction::Label(_) => todo!(),
                &Instruction::Call(_) => todo!(),
                &Instruction::Jump(_) => todo!(),
                &Instruction::JumpZero(_) => todo!(),
                &Instruction::JumpNegs(_) => todo!(),
                &Instruction::Return => todo!(),
                &Instruction::Exit => todo!(),
                &Instruction::CharOut => todo!(),
                &Instruction::NumOut => todo!(),
                &Instruction::CharIn => todo!(),
                &Instruction::NumIn => todo!(),
            }
        }

        Err(anyhow::anyhow!(
            "exit command must be done in the last of Whitespace program."
        ))
    }

    fn find_labels(insts: &Vec<Instruction>) -> HashMap<String, usize> {
        let mut labels = HashMap::<String, usize>::new();
        for (i, inst) in insts.iter().enumerate() {
            match inst {
                Instruction::Label(name) => {
                    // ラベル名がだぶった場合は先に登録したほうを優先する
                    labels.entry(name.clone()).or_insert(i);
                }
                _ => (),
            }
        }
        labels
    }

    fn pop(&mut self) -> Result<i32> {
        let x = self
            .stack
            .pop()
            .context("cannot pop from the empty stack.")?;
        Ok(x)
    }
}
