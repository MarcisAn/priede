use celsium::{ BuiltinTypes, ObjectFieldType, block::Block };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ Compiler, parse_ast::{ func_call::func_call, id_assign::lookup_var_id_or_error, parse_ast } };

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
    if title == "dot_call_fn" {
        parse_ast(node.child(0), compiler, block);
        let self_type = Some(compiler.typestack.pop().unwrap());
        func_call(node.child(1), "func_call", compiler, block, self_type);
        let return_type = compiler.typestack.pop();
        if return_type.is_some() {
            if node.child(0).get_symbol().name == "ID" {
                let var_name = node.child(0).get_value().unwrap().to_string();
                let Some(var_id) = lookup_var_id_or_error(var_name, node, compiler, block) else {
                    return;
                };
                block.assign_variable(var_id);
            }
        }
    }
}
