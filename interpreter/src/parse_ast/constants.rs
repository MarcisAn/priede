use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ util::rem_first_and_last, Compiler };

pub fn parse_constants(node: AstNode, title: &str, compiler: &mut Compiler) {
    if title == "NUMBER" {
        let number_as_str = &node.get_value().unwrap();
        if number_as_str.contains(",") {
            compiler.block.load_float(number_as_str.replace(",", ".").parse().unwrap());
            compiler.typestack.push(BuiltinTypes::Float);
        } else {
            compiler.block.load_int(number_as_str.parse().unwrap());
            compiler.typestack.push(BuiltinTypes::Int);
        }
    } else if title == "STRING" {
        compiler.block.load_string(
            rem_first_and_last(node.get_value().unwrap())
        );
        compiler.typestack.push(BuiltinTypes::String)
    } else if title == "BOOL" {
        compiler.block.load_bool(match node.child(0).to_string().as_str() {
            "TRUE" => true,
            "FALSE" => false,
            _ => panic!(),
        });
        compiler.typestack.push(BuiltinTypes::Bool);
    }
}
