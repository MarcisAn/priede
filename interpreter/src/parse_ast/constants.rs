use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ util::rem_first_and_last, Compiler };

pub fn parse_constants(node: AstNode, title: &str, compiler: &mut Compiler) {
    if title == "NUMBER" {
        let number_as_str = &node.get_value().unwrap();
        if number_as_str.contains(",") {
            compiler.block.load_const(celsium::BuiltinTypes::Float, &number_as_str.replace(",", "."));
            compiler.helper.push(BuiltinTypes::Float);
        } else {
            compiler.block.load_const(celsium::BuiltinTypes::MagicInt, &number_as_str);
            compiler.helper.push(BuiltinTypes::MagicInt);
        }
    } else if title == "STRING" {
        compiler.block.load_const(
            celsium::BuiltinTypes::String,
            rem_first_and_last(node.get_value().unwrap())
        );
        compiler.helper.push(BuiltinTypes::String)
    } else if title == "BOOL" {
        compiler.block.load_const(BuiltinTypes::Bool, match node.child(0).to_string().as_str() {
            "TRUE" => "1",
            "FALSE" => "0",
            _ => panic!(),
        });
        compiler.helper.push(BuiltinTypes::Bool);
    }
}
