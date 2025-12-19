use celsium::{ block::Block, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ util::rem_first_and_last, Compiler };

pub fn parse_constants(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "NUMBER" {
        let number_as_str = &node.get_value().unwrap();
        if number_as_str.contains(",") {
            block.load_float(number_as_str.replace(",", ".").parse().unwrap());
            compiler.typestack.push(BuiltinTypes::Float);
        } else {
            block.load_int(number_as_str.parse().unwrap());
            compiler.typestack.push(BuiltinTypes::Int);
        }
    } else if title == "STRING" {
        block.load_string(
            rem_first_and_last(node.get_value().unwrap())
        );
        compiler.typestack.push(BuiltinTypes::String)
    } else if title == "BOOL" {
        block.load_bool(match node.child(0).get_value().unwrap() {
            "PAT" => true,
            "PATIESS" => true,
            "NEPAT" => false,
            "NEPATIESS" => false,
            _ => panic!(),
        });
        compiler.typestack.push(BuiltinTypes::Bool);
    }
}
