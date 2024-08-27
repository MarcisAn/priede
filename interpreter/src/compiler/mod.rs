use celsium::{block::Block, compiletime_helper::{CompileTimeFunction, CompileTimeHelper}, module::Function, typestack::TypeStack};


#[derive(Debug, Clone)]
pub struct CompileError {
    pub(crate) line: usize,
    pub(crate) char_start: usize,
    pub(crate) length: usize,
    pub(crate) error_type: CompileErrorType
}

#[derive(Debug, Clone)]
pub enum CompileErrorType{
    Parser{unexpected_string: String},
    MathTypes,
    IncorrectVariableInitValue{expected: celsium::BuiltinTypes, found: celsium::BuiltinTypes},
    VariableAlreadyDefined{name: String},
    VariableAlreadyImported{name: String}
}

pub struct Compiler {
    pub helper: CompileTimeHelper,
    pub typestack: TypeStack,
    pub block: Block,
    pub is_wasm: bool,
    pub errors: Vec<CompileError>,
    pub functions: Vec<Function>
}

impl Compiler {
    pub fn add_error(&mut self, error: CompileErrorType, line: usize, column: usize, length: usize) {
        self.errors.push(CompileError { line, char_start: column, length, error_type: error});
    }
}