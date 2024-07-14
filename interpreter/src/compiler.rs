use celsium::{block::Block, compiletime_helper::CompileTimeHelper};


#[derive(Debug, Clone)]
pub struct CompileError {
    pub(crate) line: usize,
    pub(crate) char_start: usize,
    pub(crate) length: usize,
    pub(crate) error_type: CompileErrorType
}

#[derive(Debug, Clone)]
pub enum CompileErrorType{
    Parser{unexpected_string: String}

}

#[derive(Debug)]
pub struct Compiler {
    pub helper: CompileTimeHelper,
    pub block: Block,
    pub is_wasm: bool,
    pub errors: Vec<CompileError>
}

impl Compiler {
}