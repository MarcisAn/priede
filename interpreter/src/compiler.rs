use celsium::{block::Block, compiletime_helper::CompileTimeHelper};


#[derive(Debug, Clone)]
pub enum CompileErrors {
    Parser{unexpected_string: String, line: usize, char_start: usize, lenght: usize}
}

#[derive(Debug)]
pub struct Compiler {
    pub helper: CompileTimeHelper,
    pub block: Block,
    pub is_wasm: bool,
    pub errors: Vec<CompileErrors>
}

impl Compiler {
}