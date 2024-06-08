use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, BUILTIN_TYPES };
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
                celsium::BUILTIN_TYPES::FLOAT,
                &number_as_str.replace(",", "."),
            );
            typestack.push(BUILTIN_TYPES::FLOAT);
        } else {
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, &number_as_str);
            typestack.push(BUILTIN_TYPES::MAGIC_INT);
        }
    } else if title == "STRING" {
        block.load_const(
            celsium::BUILTIN_TYPES::STRING,
            rem_first_and_last(node.get_value().unwrap()),
        );
        typestack.push(BUILTIN_TYPES::STRING)
    } else if title == "BOOL" {
        block.load_const(
            BUILTIN_TYPES::BOOL,
            match node.child(0).to_string().as_str() {
                "TRUE" => "1",
                "FALSE" => "0",
                _ => panic!(),
            },
        );
        typestack.push(BUILTIN_TYPES::BOOL);
    }}