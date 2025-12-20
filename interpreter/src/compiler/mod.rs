use std::process::exit;

use celsium::{ compiletime_helper::CompileTimeHelper, module::Function, typestack::TypeStack };
use hime_redist::{ ast::AstNode, text::TextPosition };

use crate::{errors::print_error, util};

use super::errors::CompileTimeErrorType;
#[derive(Clone, Debug)]
pub struct CompileTimeError {
    pub error_type: CompileTimeErrorType,
    pub file_name: String,
    pub position: Option<TextPosition>,
}

#[derive(Clone, Debug)]
pub struct Compiler {
    pub helper: CompileTimeHelper,
    pub typestack: TypeStack,
    pub is_wasm: bool,
    pub functions: Vec<Function>,
    pub errors: Vec<CompileTimeError>,
}

impl Compiler {
    pub fn add_error(&mut self, error_type: CompileTimeErrorType, node: AstNode) {
        let path = &self.helper.source_file_paths[self.helper.current_file];
        let position = util::get_closest_node_location(node);
        print_error(&CompileTimeError { error_type, file_name: path.to_string(), position }, &mut self.helper);
        exit(1);
    }
    pub fn add_parser_error(
        &mut self,
        unexpected_token: String,
        position: TextPosition,
        path: String
    ) {

        self.errors.push(CompileTimeError {
            error_type: CompileTimeErrorType::ParserError { unexpected: unexpected_token },
            file_name: path.to_string(),
            position: Some(position),
        });
    }
}
