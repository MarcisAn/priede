use std::collections::LinkedList;
use BytecodeItemType::*;

use colored::Colorize;
use hime_redist::ast::AstNode;

use super::{
    ast_parser::{self, AstParser},
    bytecode::{BytecodeItem, BytecodeItemType},
    hime,
};

pub fn generate_bytcode(source_code: String, print_ast: bool) -> Result<Vec<BytecodeItem>, String> {
    let parse_result = hime::priede::parse_string(&source_code);

    if parse_result.is_success() {
        let ast = parse_result.get_ast();
        if print_ast {
            print!("\n");
            format_ast(ast.get_root(), Vec::<bool>::new());
        }
        let ast_parser: AstParser = AstParser::new();
        let mut bytecode = ast_parser.parse(ast.get_root());
        loop {
            let mut index_of_jump_statement: usize = 0;
            let mut index_to_jump_to: usize = 0;
            let mut iter_index: usize = 0;
            let mut endif_cnt_expected: u32 = 0;
            let loopable_bytecode = bytecode.clone();
            for i in loopable_bytecode.into_iter() {
                match i.typ {
                    JumpToIfFalse { jump_to } => {
                        if jump_to == 0 {
                            if index_of_jump_statement == 0 {
                                index_of_jump_statement = iter_index;
                            }
                            endif_cnt_expected += 1;
                        }
                    }
                    EndIf { matched } => {
                        if !matched {
                            if endif_cnt_expected == 1 {
                                index_to_jump_to = iter_index;
                                break;
                            } else {
                                endif_cnt_expected -= 1;
                            }
                        }
                    }
                    _ => (),
                }
                iter_index += 1;
            }
            if index_of_jump_statement == 0 && index_to_jump_to == 0 {
                break;
            } else {
                bytecode[index_of_jump_statement].typ = BytecodeItemType::JumpToIfFalse {
                    jump_to: index_to_jump_to,
                };
                bytecode[index_to_jump_to].typ = BytecodeItemType::EndIf { matched: true };
            }
        }
        loop {
            let mut index_of_jump_statement: usize = 0;
            let mut index_to_jump_to: usize = 0;
            let mut iter_index: usize = bytecode.len();
            let mut endif_cnt_expected: u32 = 0;
            let loopable_bytecode = bytecode.clone();
            for i in loopable_bytecode.into_iter() {
                match i.typ {
                    JumpTo { jump_to } => {
                        if jump_to == 0 {
                            if endif_cnt_expected == 1 {
                                index_to_jump_to = iter_index;
                                break;
                            } else {
                                endif_cnt_expected -= 1;
                            }
                        }
                    }
                    StartWhileLoop { matched } => {
                        if !matched {
                            if index_of_jump_statement == 0 {
                                index_of_jump_statement = iter_index;
                            }
                            endif_cnt_expected += 1;
                        }
                    }
                    _ => (),
                }
                iter_index -= 1;
            }
            if index_of_jump_statement == 0 && index_to_jump_to == 0 {
                break;
            } else {
                bytecode[index_of_jump_statement].typ = BytecodeItemType::JumpTo {
                    jump_to: index_to_jump_to,
                };
                bytecode[index_to_jump_to].typ = BytecodeItemType::StartWhileLoop { matched: true };
            }
        }
        loop {
            let mut index_of_jump_statement: usize = 0;
            let mut index_to_jump_to: usize = 0;
            let mut iter_index: usize = 0;
            let mut endif_cnt_expected: u32 = 0;
            let loopable_bytecode = bytecode.clone();
            for i in loopable_bytecode.into_iter() {
                match i.typ {
                    FuncDef { name, jump_to, len } => {
                        if jump_to == 0 {
                            if index_of_jump_statement == 0 {
                                index_of_jump_statement = iter_index;
                            }
                            endif_cnt_expected += 1;
                        }
                    }
                    EndFuncDef { matched } => {
                        if !matched {
                            if endif_cnt_expected == 1 {
                                index_to_jump_to = iter_index;
                                break;
                            } else {
                                endif_cnt_expected -= 1;
                            }
                        }
                    }
                    _ => (),
                }
                iter_index += 1;
            }
            if index_of_jump_statement == 0 && index_to_jump_to == 0 {
                break;
            } else {
                let len = index_to_jump_to - index_of_jump_statement;
                let func_name = match bytecode[index_of_jump_statement].clone().typ {
                    FuncDef { name, jump_to, len } => name,
                    _ => "".to_string(),
                };
                bytecode[index_of_jump_statement].typ = BytecodeItemType::FuncDef {
                    jump_to: index_to_jump_to,
                    name: func_name,
                    len: len,
                };
                bytecode[index_to_jump_to].typ = BytecodeItemType::EndFuncDef { matched: true };
            }
        }
        Ok(bytecode)
    } else {
        print!("{}", "Programmas izpilde atlikta".red());
        print!("{}", parse_result.get_errors()[0]);

        return Err("".to_string());
    }
}

fn format_ast<'a>(node: AstNode<'a>, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", "  ");
            i += 1;
        }
        print!(" ");
    }
    println!("{:}", node.to_string().blue());
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        format_ast(children.at(i), child_crossings);
        i += 1;
    }
}
