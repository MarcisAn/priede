use celsium::BuiltinTypes;
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ util::rem_first_and_last, Compiler };

pub fn parse_constants(node: AstNode, title: &str, compiler: &mut Compiler) {
    let target_reg = compiler.register_counter;
    if title == "NUMBER" {
        let number_as_str = &node.get_value().unwrap();
        if number_as_str.contains(",") {
            compiler.block.load_const(
                celsium::BuiltinTypes::Float,
                &number_as_str.replace(",", "."),
                target_reg
            );
            compiler.helper.push(BuiltinTypes::Float, target_reg);
        } else {
            compiler.block.load_const(celsium::BuiltinTypes::MagicInt, &number_as_str, target_reg);
            compiler.helper.push(BuiltinTypes::MagicInt, target_reg);
        }
        compiler.register_counter += 1;
    } else if title == "STRING" {
        compiler.block.load_const(
            celsium::BuiltinTypes::String,
            rem_first_and_last(node.get_value().unwrap()),
            target_reg
        );
        compiler.helper.push(BuiltinTypes::String, target_reg);
        compiler.register_counter += 1;
    } else if title == "BOOL" {
        compiler.block.load_const(
            BuiltinTypes::Bool,
            match node.child(0).to_string().as_str() {
                "TRUE" => "1",
                "FALSE" => "0",
                _ => panic!(),
            },
            target_reg
        );
        compiler.helper.push(BuiltinTypes::Bool, target_reg);
        compiler.register_counter += 1;
    }
}
