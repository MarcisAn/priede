use celsium::{ BuiltinTypes, ObjectFieldType, block::Block };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, parse_ast::parse_ast };

pub fn get_object_field_type(
    node: AstNode,
    block: &mut Block,
    fields: Vec<ObjectFieldType>
) -> BuiltinTypes {
    let requested_field = node.child(1).get_value().unwrap().trim();
    for defined_field in fields {
        if defined_field.name == requested_field {
            block.get_object_field(requested_field.to_string());
            return defined_field.data_type;
        }
    }
    panic!("Tāds objekta lauks neeksistē");
}

pub fn dot_syntax(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "dot_call" {
        parse_ast(node.child(0), compiler, block);
        let type_of_dot_attempt = compiler.typestack.pop().unwrap();

        let returnable_type = match type_of_dot_attempt {
            celsium::BuiltinTypes::Int => todo!(),
            celsium::BuiltinTypes::Bool => todo!(),
            celsium::BuiltinTypes::String => todo!(),
            celsium::BuiltinTypes::Object { fields } => get_object_field_type(node, block, fields),
            celsium::BuiltinTypes::Array { element_type, length } => todo!(),
            celsium::BuiltinTypes::Float => todo!(),
        };
        compiler.typestack.push(returnable_type);
    }
}
