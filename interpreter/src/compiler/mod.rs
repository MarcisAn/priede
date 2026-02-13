use std::process::exit;

use celsium::BuiltinTypes;
use celsium::module::FuncArg;
use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, module::Function, typestack::TypeStack };
use hime_redist::{ ast::AstNode, text::TextPosition };
use crate::errors::{self, common_error};

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
    pub fn check_function_signature(
    &mut self,
    return_type: Option<BuiltinTypes>,
    args: Vec<FuncArg>,
    args_found: Vec<BuiltinTypes>,
    node: AstNode,
    name: String
) {
    if return_type.is_some() {
        self.typestack.push(return_type.unwrap());
    }

    let arg_count_error = errors::CompileTimeErrorType::WrongFunctionArgumentCount {
        function_name: name.to_string(),
        expected_count: args.len(),
        found_count: args_found.len(),
    };

    if name == "izvade" || name == "izvadetp" || name == "garums" {
        if args_found.len() != 1 {
            self.add_error(arg_count_error.clone(), node);
        } else {
            return;
        }
    }
    //first check if argument cound is valid
    if args.len() != args_found.len() {
        self.add_error(arg_count_error, node);
    }
    //then check if arguement types are valid
    let mut counter = 0;
    for expected_arg in args {
        let found_arg = args_found[counter].clone();
        if expected_arg.arg_type != found_arg {
            self.add_error(
                errors::CompileTimeErrorType::WrongFunctionArgumentType {
                    function_name: name.to_string(),
                    arg_index: counter + 1,
                    expected_type: expected_arg.arg_type,
                    found_type: found_arg,
                },
                node
            );
        }
        counter += 1;
    }
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