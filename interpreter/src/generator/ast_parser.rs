use std::collections::LinkedList;

use crate::generator::bytecode::BytecodeItem;
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait, text::TextPosition};

use super::bytecode::BytecodeItemType;

pub struct AstParser {
    bytecode: Vec<BytecodeItem>,
}

impl AstParser {
    pub fn new() -> AstParser {
        AstParser { bytecode: vec![] }
    }

    pub fn parse(mut self, ast: AstNode) -> Vec<BytecodeItem> {
        Self::parse_ast(&mut self, ast);
        /*for i in self.bytecode {
            println!("{:?}", i);
        }*/
        self.bytecode
    }

    fn push_to_bytecode(&mut self, item: BytecodeItem) {
        self.bytecode.push(item);
    }

    fn trim_string(value: String) -> String {
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        chars.as_str().to_string()
    }

    fn parse_ast(mut self: &mut AstParser, input: AstNode) {
        /*KONSTANTES */
        if input.to_string().starts_with("NUMBER") {
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::PushNumber {
                        value_as_string: input.get_value().unwrap(),
                    },
                    position: input.get_position(),
                },
            );
        } else if input.to_string().starts_with("STRING") {
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::PushString {
                        value: Self::trim_string(input.get_value().unwrap()),
                    },
                    position: input.get_position(),
                },
            );
        }
        /*MATEMĀTIKA */
        else if input.to_string() == "plus" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::Add,
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "minus" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::Subtract,
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "reiz" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::Multiply,
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "dal" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::Divide,
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "atlik" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::Remainder,
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "block" {
            let mut i = 0;
            while i < input.children().len() {
                Self::parse_ast(&mut self, input.children().at(i));
                i += 1;
            }
        } else if input.to_string() == "func_call" {
            if input.children().len() > 1 {
                Self::parse_ast(&mut self, input.children().at(1));
            }
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::FunctionCall {
                        name: input.children().at(0).get_value().unwrap(),
                    },
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "funcargs" {
            let mut i = 0;
            while i < input.children().len() {
                Self::parse_ast(&mut self, input.children().at(i));
                i += 1;
            }
        } else if input.to_string() == "if" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::JumpToIfFalse { jump_to: 0 },
                    position: input.get_position(),
                },
            );
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::EndIf { matched: false },
                    position: input.get_position(),
                },
            );
            if input.children().len() > 2 {
                Self::parse_ast(&mut self, input.children().at(0));
                Self::push_to_bytecode(
                    self,
                    BytecodeItem {
                        typ: BytecodeItemType::InvertStackItem,
                        position: input.get_position(),
                    },
                );
                Self::push_to_bytecode(
                    self,
                    BytecodeItem {
                        typ: BytecodeItemType::JumpToIfFalse { jump_to: 0 },
                        position: input.get_position(),
                    },
                );
                Self::parse_ast(&mut self, input.children().at(3));
                Self::push_to_bytecode(
                    self,
                    BytecodeItem {
                        typ: BytecodeItemType::EndIf { matched: false },
                        position: input.get_position(),
                    },
                );
            }
        } else if input.to_string() == "if_exp" {
            Self::parse_ast(&mut self, input.children().at(0));
            if input.children().at(0).to_string() != "comp_s" {
                Self::push_to_bytecode(
                    self,
                    BytecodeItem {
                        typ: BytecodeItemType::IfExp,
                        position: input.get_position(),
                    },
                );
            }
        } else if input.to_string() == "comp_s" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(2));
            let comp_sign = match input.children().at(1).get_value().unwrap().as_str() {
                "=" => BytecodeItem {
                    typ: BytecodeItemType::Equals,
                    position: input.get_position(),
                },
                "<" => BytecodeItem {
                    typ: BytecodeItemType::LessThan,
                    position: input.get_position(),
                },
                ">" => BytecodeItem {
                    typ: BytecodeItemType::GreaterThan,
                    position: input.get_position(),
                },
                ">=" => BytecodeItem {
                    typ: BytecodeItemType::GreaterEqual,
                    position: input.get_position(),
                },
                "<=" => BytecodeItem {
                    typ: BytecodeItemType::LessEqual,
                    position: input.get_position(),
                },
                "!=" => BytecodeItem {
                    typ: BytecodeItemType::NotEqual,
                    position: input.get_position(),
                },
                &_ => panic!("komp operators"),
            };
            Self::push_to_bytecode(self, comp_sign);
        } else if input.to_string() == "un" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(1));

            Self::push_to_bytecode(
                &mut self,
                BytecodeItem {
                    typ: BytecodeItemType::And,
                    position: input.get_position(),
                },
            );
            let mut i = 0;
            while i < input.children().len() - 2 {
                i += 1;
                Self::parse_ast(&mut self, input.children().at(i));
                Self::push_to_bytecode(
                    &mut self,
                    BytecodeItem {
                        typ: BytecodeItemType::And,
                        position: input.get_position(),
                    },
                );
            }
        } else if input.to_string() == "vai" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(1));

            Self::push_to_bytecode(
                &mut self,
                BytecodeItem {
                    typ: BytecodeItemType::Or,
                    position: input.get_position(),
                },
            );
            let mut i = 0;
            while i < input.children().len() - 2 {
                i += 1;
                Self::parse_ast(&mut self, input.children().at(i));
                Self::push_to_bytecode(
                    &mut self,
                    BytecodeItem {
                        typ: BytecodeItemType::Or,
                        position: input.get_position(),
                    },
                );
            }
        } else if input.to_string() == "xvai" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                &mut self,
                BytecodeItem {
                    typ: BytecodeItemType::ExclusiveOr,
                    position: input.get_position(),
                },
            )
        } else if input.to_string() == "var_def" {
            Self::parse_ast(&mut self, input.children().at(2));
            Self::push_to_bytecode(
                &mut self,
                BytecodeItem {
                    typ: BytecodeItemType::VarDef {
                        name: input.children().at(1).get_value().unwrap(),
                        typ: match input.children().at(0).to_string().as_str() {
                            "NUM" => super::bytecode::VarTypes::Number,
                            "TEXT" => super::bytecode::VarTypes::String,
                            "BOOL_DEF" => super::bytecode::VarTypes::Bool,
                            _ => panic!("neatpazīts mainīgo datu tips"),
                        },
                    },
                    position: input.get_position(),
                },
            )
        } else if input.to_string().starts_with("ID =") {
            let name = input.get_value().unwrap();
            Self::push_to_bytecode(
                &mut self,
                BytecodeItem {
                    typ: BytecodeItemType::LoadVar { name },
                    position: input.get_position(),
                },
            )
        } else if input.to_string() == "s_loop" {
            Self::parse_ast(&mut self, input.children().at(0));
            Self::push_to_bytecode(
                &mut self,
                BytecodeItem {
                    typ: BytecodeItemType::StartSimpleLoop,
                    position: input.get_position(),
                },
            );
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                &mut self,
                BytecodeItem {
                    typ: BytecodeItemType::EndSimpleLoop,
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "BOOL" {
            Self::push_to_bytecode(
                &mut self,
                BytecodeItem {
                    typ: BytecodeItemType::PushBool {
                        value: if input.children().at(0).to_string() == "TRUE" {
                            true
                        } else {
                            false
                        },
                    },
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "id_asign" {
            if input.children().at(1).get_value().unwrap() == ":=" {
                Self::parse_ast(&mut self, input.children().at(2));
                Self::push_to_bytecode(
                    &mut self,
                    BytecodeItem {
                        typ: BytecodeItemType::MutateVar {
                            name: input.children().at(0).get_value().unwrap(),
                        },
                        position: input.get_position(),
                    },
                );
            } else {
                Self::push_to_bytecode(
                    &mut self,
                    BytecodeItem {
                        typ: BytecodeItemType::LoadVar {
                            name: input.children().at(0).get_value().unwrap(),
                        },
                        position: input.get_position(),
                    },
                );
                Self::parse_ast(&mut self, input.children().at(2));
                Self::push_to_bytecode(
                    &mut self,
                    BytecodeItem {
                        typ: match input.children().at(1).get_value().unwrap().as_str() {
                            "+=" => BytecodeItemType::Add,
                            "-=" => BytecodeItemType::Subtract,
                            "*=" => BytecodeItemType::Multiply,
                            "/=" => BytecodeItemType::Divide,
                            &_ => todo!(),
                        },
                        position: input.get_position(),
                    },
                );

                Self::push_to_bytecode(
                    &mut self,
                    BytecodeItem {
                        typ: BytecodeItemType::MutateVar {
                            name: input.children().at(0).get_value().unwrap(),
                        },
                        position: input.get_position(),
                    },
                );
            }
        } else if input.to_string() == "w_loop" {
            self.push_to_bytecode(BytecodeItem {
                typ: BytecodeItemType::StartWhileLoop { matched: false },
                position: input.get_position(),
            });
            Self::parse_ast(&mut self, input.children().at(0));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::JumpToIfFalse { jump_to: 0 },
                    position: input.get_position(),
                },
            );
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::JumpTo { jump_to: 0 },
                    position: input.get_position(),
                },
            );
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::EndIf { matched: false },
                    position: input.get_position(),
                },
            );
        } else if input.to_string() == "func_def" {
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::FuncDef {
                        jump_to: 0,
                        name: input.children().at(0).get_value().unwrap(),
                        len: 0,
                    },
                    position: input.get_position(),
                },
            );
            Self::parse_ast(&mut self, input.children().at(1));
            Self::push_to_bytecode(
                self,
                BytecodeItem {
                    typ: BytecodeItemType::EndFuncDef { matched: false },
                    position: input.get_position(),
                },
            );
        } else {
            println!("nekerts {}", input.to_string());
        }
    }
}
