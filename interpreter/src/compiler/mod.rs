use celsium::{
    block::Block,
    compiletime_helper::{ CompileTimeFunction, CompileTimeHelper },
    module::Function,
    typestack::TypeStack,
};
use colored::Colorize;
use hime_redist::text::TextPosition;

#[derive(Debug, Clone)]
pub struct CompileError {
    pub(crate) line: usize,
    pub(crate) char_start: usize,
    pub(crate) length: usize,
    pub(crate) error_type: CompileErrorType,
    pub(crate) path: String
}

#[derive(Debug, Clone)]
pub enum CompileErrorType {
    Parser {
        unexpected_string: String,
    },
    MathTypes,
    IncorrectVariableInitValue {
        expected: celsium::BuiltinTypes,
        found: celsium::BuiltinTypes,
    },
    VariableAlreadyDefined {
        name: String,
    },
    VariableAlreadyImported {
        name: String,
    },
    NotIndexable {
        name: String,
    },
}

pub struct Compiler {
    pub helper: CompileTimeHelper,
    pub typestack: TypeStack,
    pub is_wasm: bool,
    pub errors: Vec<CompileError>,
    pub functions: Vec<Function>,
}

impl Compiler {
    pub fn add_error(
        &mut self,
        error: CompileErrorType,
        position: TextPosition,
        path: String
    ) {
        let line = position.line;
        let column = position.column;
        self.errors.push(CompileError { line, char_start: column, length: 1, error_type: error, path: path.to_string() });
    }
}
