use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, BufWriter, Write},
};

use anyhow::{Context, Result};

use crate::instruction::Instruction;

pub struct VM {
    insts: Vec<Instruction>,
    stack: Vec<i64>,
    labels: HashMap<i64, i64>,
}

impl VM {
    pub fn new(insts: Vec<Instruction>) -> Result<Self> {
        let labels = Self::find_labels(&insts)?;
        Ok(Self {
            insts: insts,
            stack: vec![],
            labels: labels,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut pc = 0;
        while pc < self.insts.len() {
            match self.insts[pc] {
                Instruction::Push(x) => {
                    self.stack.push(x);
                }
                Instruction::Dup => {
                    let x = self.stack[self.stack.len() - 1];
                    self.stack.push(x);
                }
                Instruction::Swap => {
                    let x = self.pop()?;
                    let y = self.pop()?;
                    self.stack.push(x);
                    self.stack.push(y);
                }
                Instruction::Rotate => {
                    // |-> x y z
                    // ↓
                    // |-> z x y
                    let z = self.pop()?;
                    let y = self.pop()?;
                    let x = self.pop()?;
                    self.stack.push(z);
                    self.stack.push(x);
                    self.stack.push(y);
                }
                Instruction::Pop => {
                    let _ = self.pop()?;
                }
                Instruction::Add => {
                    let x = self.pop()?;
                    let y = self.pop()?;
                    self.stack.push(x + y);
                }
                Instruction::Sub => {
                    let x = self.pop()?;
                    let y = self.pop()?;
                    self.stack.push(x - y);
                }
                Instruction::Mul => {
                    let x = self.pop()?;
                    let y = self.pop()?;
                    self.stack.push(x * y);
                }
                Instruction::Div => {
                    let x = self.pop()?;
                    let y = self.pop()?;
                    self.stack.push(x / y);
                }
                Instruction::Mod => {
                    let x = self.pop()?;
                    let y = self.pop()?;
                    self.stack.push(x % y);
                }
                Instruction::NumOut => {
                    let x = self.pop()?;
                    let x = x.to_string();
                    let mut writer = BufWriter::new(io::stdout());
                    writer.write(x.as_bytes())?;
                    writer.flush()?;
                }
                Instruction::CharOut => {
                    let x = self.pop()?;
                    let mut writer = BufWriter::new(io::stdout());
                    // ASCIIコードとみなす
                    let x = x as u8;
                    writer.write(&[x])?;
                    writer.flush()?;
                }
                Instruction::NumIn => {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf)?;
                    let buf = buf.trim_end(); // 末尾の改行を除去
                    let x = buf.parse()?;
                    self.stack.push(x);
                }
                Instruction::CharIn => {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf)?;
                    let buf = buf.trim_end(); // 末尾の改行を除去
                    let buf = buf.as_bytes();
                    let x = buf[0] as i64;
                    self.stack.push(x);
                }
                // ラベルの位置はすでに調べているので何もしない
                Instruction::Label(_) => (),
                Instruction::JumpNonZero(label) => {
                    let x = self.pop()?;
                    if x != 0 {
                        pc = self.resolve_label(label)?;
                    }
                }
                Instruction::Dummy => {
                    return Err(anyhow::anyhow!("dummy instruction."));
                }
            }

            pc += 1;
        }

        Ok(())
    }

    fn find_labels(insts: &Vec<Instruction>) -> Result<HashMap<i64, i64>> {
        let mut labels = HashMap::new();
        for (i, inst) in insts.iter().enumerate() {
            match inst {
                Instruction::Label(label) => {
                    let label = *label;
                    let e = labels.entry(label);
                    match e {
                        Entry::Occupied(_) => {
                            let msg = format!("label <{}> is duplicate.", label);
                            return Err(anyhow::anyhow!(msg));
                        }
                        Entry::Vacant(_) => {
                            e.or_insert(i as i64);
                        }
                    }
                }
                _ => (),
            }
        }

        Ok(labels)
    }

    fn pop(&mut self) -> Result<i64> {
        let x = self
            .stack
            .pop()
            .with_context(|| "cannot pop from the empty stack.")?;
        Ok(x)
    }

    fn resolve_label(&self, label: i64) -> Result<usize> {
        let pc = self
            .labels
            .get(&label)
            .with_context(|| format!("label <{}> is not found.", label))?;
        Ok(*pc as usize)
    }
}
