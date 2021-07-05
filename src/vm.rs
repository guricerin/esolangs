use std::{collections::HashMap, io, io::BufWriter, io::Write};

use anyhow::{self, Context, Result};

use crate::instruction::Instruction;

#[derive(Debug)]
pub struct VM {
    insts: Vec<Instruction>,
    stack: Vec<i64>,
    /// K: address, V: value
    heap: HashMap<u64, i64>,
    /// K: label, V: position
    labels: HashMap<String, u64>,
    /// 末尾はサブルーチンの戻り先
    call_stack: Vec<usize>,
}

impl VM {
    pub fn new(insts: Vec<Instruction>) -> Self {
        let labels = Self::find_labels(&insts);
        Self {
            insts: insts,
            stack: Vec::new(),
            heap: HashMap::new(),
            labels: labels,
            call_stack: vec![],
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut pc = 0;
        while pc < self.insts.len() {
            let inst = self.insts[pc].clone();
            match inst {
                Instruction::Push(n) => {
                    self.stack.push(n);
                }
                Instruction::Dup => {
                    // if let Some(n) = self.stack.last() {
                    //     self.stack.push(*n);
                    // }
                    let x = self.stack[self.stack.len() - 1];
                    self.stack.push(x);
                }
                Instruction::Copy(n) => {
                    // ケツからn番目（0 indexed）
                    let v = self.stack[self.stack.len() - (n as usize + 1)];
                    self.stack.push(v);
                }
                Instruction::Swap => {
                    let x = self.pop()?;
                    let y = self.pop()?;
                    self.stack.push(x);
                    self.stack.push(y);
                }
                Instruction::Discard => {
                    let _ = self.stack.pop();
                }
                Instruction::Slide(n) => {
                    let x = self.pop()?;
                    for _ in 0..(n as usize) {
                        self.pop()?;
                    }
                    self.stack.push(x);
                }
                Instruction::Add => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l + r);
                }
                Instruction::Sub => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l - r);
                }
                Instruction::Mul => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l * r);
                }
                Instruction::Div => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l / r);
                }
                Instruction::Mod => {
                    let r = self.pop()?;
                    let l = self.pop()?;
                    self.stack.push(l % r);
                }
                Instruction::HeapWrite => {
                    let value = self.pop()?;
                    let address = self.pop()?;
                    *self.heap.entry(address as u64).or_insert(value) = value;
                }
                Instruction::HeapRead => {
                    let address = self.pop()?;
                    let value = self
                        .heap
                        .get(&(address as u64))
                        .context("cannot read an uninitialized heap position.")?;
                    self.stack.push(*value);
                }
                // ラベルの位置はすでに調べているので何もしない
                Instruction::Label(_) => (),
                Instruction::Call(label) => {
                    self.call_stack.push(pc);
                    pc = self.resolve_label(&label)?;
                }
                Instruction::Jump(name) => {
                    pc = self.resolve_label(&name)?;
                }
                Instruction::JumpZero(name) => {
                    let x = self.pop()?;
                    if x == 0 {
                        pc = self.resolve_label(&name)?;
                    }
                }
                Instruction::JumpNegs(name) => {
                    let x = self.pop()?;
                    if x < 0 {
                        pc = self.resolve_label(&name)?;
                    }
                }
                Instruction::Return => match self.call_stack.pop() {
                    Some(x) => pc = x,
                    _ => return Err(anyhow::anyhow!("cannot return from the out of subroutine.")),
                },
                Instruction::Exit => {
                    return Ok(());
                }
                Instruction::CharOut => {
                    let x = self.pop()?;
                    let mut writer = BufWriter::new(io::stdout());
                    // ASCIIコードとみなす
                    writer.write(&[x as u8])?;
                }
                Instruction::NumOut => {
                    let x = self.pop()?;
                    let x = x.to_string();
                    let mut writer = BufWriter::new(io::stdout());
                    writer.write(x.as_bytes())?;
                }
                Instruction::CharIn => {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf)?;
                    let buf = buf.into_bytes();

                    let address = self.pop()?;
                    let n = buf[0] as i64;
                    *self.heap.entry(address as u64).or_insert(n) = n;
                }
                Instruction::NumIn => {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf)?;

                    let address = self.pop()?;
                    let n = buf.parse()?;
                    *self.heap.entry(address as u64).or_insert(n) = n;
                }
            }

            pc += 1;
        }

        Err(anyhow::anyhow!(
            "exit command must be done in the last of Whitespace program."
        ))
    }

    fn find_labels(insts: &Vec<Instruction>) -> HashMap<String, u64> {
        let mut labels = HashMap::new();
        for (i, inst) in insts.iter().enumerate() {
            match inst {
                Instruction::Label(name) => {
                    // ラベル名がだぶった場合は先に登録したほうを優先する
                    labels.entry(name.clone()).or_insert(i as u64);
                }
                _ => (),
            }
        }
        labels
    }

    fn pop(&mut self) -> Result<i64> {
        let x = self
            .stack
            .pop()
            .context("cannot pop from the empty stack.")?;
        Ok(x)
    }

    fn resolve_label(&self, label: &str) -> Result<usize> {
        let pc = self
            .labels
            .get(label)
            .with_context(|| format!("label is not found. label name: {}", label))?;
        Ok(*pc as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, vm::VM};

    #[test]
    fn test1() {
        let insts = vec![Instruction::Push(1), Instruction::NumOut, Instruction::Exit];
        let mut vm = VM::new(insts);
        vm.run().unwrap();
    }
}
