use anyhow::Result;

#[derive(Debug)]
pub struct Compiler {
    src_code: String,
}

impl Compiler {
    pub fn new(src_code: String) -> Self {
        Self { src_code: src_code }
    }

    pub fn compile() -> Result<()> {
        Ok(())
    }
}
