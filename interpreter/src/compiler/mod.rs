use std::process::exit;

use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, module::Function, typestack::TypeStack };
use hime_redist::{ ast::AstNode, text::TextPosition };
use crate::errors::common_error;

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

pub fn proces_break_and_continue(main_block: &mut Block, compiler: &mut Compiler){
    let mut i = 0;
    let mut jump_target_if_break_called = 0;
    while i < main_block.bytecode.len() {
        match &main_block.bytecode[i] {
            celsium::bytecode::OPTCODE::JumpIfFalse {
                steps,
                jump_target_line: _,
                jump_target_column: _,
                is_skipable,
            } => {
                if *is_skipable {
                    jump_target_if_break_called = i + steps;
                }
            }
            celsium::bytecode::OPTCODE::Break { span } => {
                if i > jump_target_if_break_called {
                    let span_c = span.clone();
                    common_error(
                        "Pārtraukums izmantots ārpus cikla",
                        Some(TextPosition { column: span_c.col_start, line: span_c.line }),
                        &mut compiler.helper
                    );
                    exit(1);
                }
                main_block.bytecode[i] = celsium::bytecode::OPTCODE::Jump {
                    steps: jump_target_if_break_called - i,
                };
            }
            celsium::bytecode::OPTCODE::Continue { span } => {
                if i > jump_target_if_break_called {
                    let span_c = span.clone();

                    common_error(
                        "Izlaišana izmantota ārpus cikla",
                        Some(TextPosition { column: span_c.col_start, line: span_c.line }),
                        &mut compiler.helper
                    );
                    exit(1);
                }
                main_block.bytecode[i] = celsium::bytecode::OPTCODE::Jump {
                    steps: jump_target_if_break_called - i - 1,
                };
            }
            _ => print!(""),
        }
        // println!("{} {:?}", i, main_block.bytecode[i]);
        i += 1;
    }
}