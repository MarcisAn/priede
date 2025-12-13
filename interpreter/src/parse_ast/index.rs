use std::process::exit;

use celsium::{ block::Block, BuiltinTypes, ObjectFieldType };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::Compiler;

use super::parse_ast;

pub fn get_object_field_type(
    node: AstNode,
    block: &mut Block,
    fields: Vec<ObjectFieldType>
) -> BuiltinTypes {
    if node.child(1).get_symbol().name == "STRING" {
        let requested_field = node.child(1).get_value().unwrap().trim();
        let requested_field_no_quotes = requested_field[1..requested_field.len() - 1].to_string();
        for defined_field in fields {
            if defined_field.name == requested_field_no_quotes {
                block.get_object_field(requested_field_no_quotes.to_string());
                return defined_field.data_type;
            }
        }
        panic!("Tāds fields neeksistē");
    } else {
        panic!("Objektu var indeksēt tikai ar stirngu");
    }
}

pub fn index(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "indexable" {
        parse_ast(node.child(0), compiler, block);
        let type_of_index_attempt = compiler.typestack.pop().unwrap();
        let returnable_type = match type_of_index_attempt {
            BuiltinTypes::Array { element_type, length: _ } => {
                parse_ast(node.child(1), compiler, block);
                block.index();
                *element_type
            },
            BuiltinTypes::String => {
                parse_ast(node.child(1), compiler, block);
                block.index();
                BuiltinTypes::String
            }
            BuiltinTypes::Object { fields } => get_object_field_type(node, block, fields),
            _ => {
                compiler.add_error(
                    crate::errors::CompileTimeErrorType::ValueNotIndexable {
                        found_type: type_of_index_attempt,
                    },
                    node
                );
                exit(1);
            }
        };
        compiler.typestack.push(returnable_type);
    }
}
