use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::util::rem_first_and_last;


pub fn parse_constants(node: AstNode,
    title: &str,
    typestack: &mut CompileTimeHelper,
    block: &mut Block,) {
 if title == "NUMBER" {
        let number_as_str = &node.get_value().unwrap();
        if number_as_str.contains(",") {
            block.load_const(
                celsium::BuiltinTypes::Float,
                &number_as_str.replace(",", "."),
            );
            typestack.push(BuiltinTypes::Float);
        } else {
            block.load_const(celsium::BuiltinTypes::MagicInt, &number_as_str);
            typestack.push(BuiltinTypes::MagicInt);
        }
    } else if title == "String" {
        block.load_const(
            celsium::BuiltinTypes::String,
            rem_first_and_last(node.get_value().unwrap()),
        );
        typestack.push(BuiltinTypes::String)
    } else if title == "Bool" {
        block.load_const(
            BuiltinTypes::Bool,
            match node.child(0).to_string().as_str() {
                "TRUE" => "1",
                "FALSE" => "0",
                _ => panic!(),
            },
        );
        typestack.push(BuiltinTypes::Bool);
    }}