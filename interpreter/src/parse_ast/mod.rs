use celsium::{
    block::Block,
    compiletime_helper::CompileTimeHelper,
    ObjectFieldType,
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

mod id_assign;
use id_assign::id_assign;
mod math_ops;
use math_ops::math_ops;
mod var_def;
use var_def::var_def;
mod constants;
use constants::parse_constants;
mod comparisons;
use comparisons::comparisons;
mod array;
use array::array;
mod func_call;
use func_call::func_call;
mod func_def;
use func_def::func_def;
mod loops;
use loops::loops;
mod if_stat;
use if_stat::if_stat;
mod return_st;
use return_st::return_st;
mod id;
use id::id;
mod include;
use include::include;
mod array_def;

use crate::{ errors, util::{self, get_data_type_from_id} };

pub fn parse_ast(
    node: AstNode,
    block: &mut Block,
    is_wasm: bool,
    typestack: &mut CompileTimeHelper
) {
    let title = node.get_symbol().to_string();

    if title == "block" {
        for i in node.children() {
            parse_ast(i, block, is_wasm, typestack);
        }
    }

    if title == "dot_call" {
        let base = node.child(0).get_value().unwrap();
        //let dotcall = node.child(1).get_value().unwrap();

        let object_if_exists = typestack.get_object_if_exists(base);

        if object_if_exists.is_some(){

        }

        if node.child(1).get_value().unwrap() == "garums" {
            let array_name = node.child(0).get_value().unwrap().to_string();
            let array_id = util::get_closest_scope(
                array_name.clone(),
                block.scope.clone(),
                typestack,
                node
            );
            if array_id.is_none() {
                errors::undefined_var(
                    format!("Saraksts `{}` nav definēts", array_name),
                    typestack,
                    node
                );
            }
            block.get_array_length(array_id.unwrap());
            typestack.push(celsium::BuiltinTypes::MagicInt);
        }
    }

    if title == "object_def" {
        let object_title = node.child(0).get_value().unwrap().to_string();
        let mut field_counter = 1;
        let mut fields = vec![];
        while field_counter < node.children_count() {
            fields.push(ObjectFieldType {
                name: node.child(field_counter).child(1).get_value().unwrap().to_string(),
                data_type: get_data_type_from_id(typestack, node.child(field_counter).child(0).get_value().unwrap(), node),
            });
            field_counter +=1;
        }
        typestack.define_struct(object_title, fields);
    }

    if title == "object" {
        let mut fields = vec![];
        let mut field_names = vec![];
        for field in node.children().iter().rev() {
            parse_ast(field.child(1), block, is_wasm, typestack);
            let field_name = field.child(0).get_value().unwrap().to_string();
            let field = ObjectFieldType {
                name: field_name.clone(),
                data_type: typestack.pop().unwrap(),
            };
            field_names.push(field_name);
            fields.push(field);
        }
        typestack.push(celsium::BuiltinTypes::Object { fields });
        block.create_object(field_names);
    }


    id(node, &title, block, typestack);
    return_st(node, &title, block, typestack, is_wasm);
    if_stat(node, &title, block, typestack, is_wasm);
    loops(node, &title, block, typestack, is_wasm);
    func_def(node, &title, block, typestack, is_wasm);
    func_call(node, &title, block, typestack, is_wasm);
    array(node, &title, block, typestack, is_wasm);
    math_ops(node, &title, block, typestack, is_wasm);
    comparisons(node, &title, block, typestack, is_wasm);
    id_assign(node, &title, block, typestack, is_wasm);
    parse_constants(node, &title, typestack, block);
    var_def(node, &title, typestack, is_wasm, block);
    include(node, &title, is_wasm, typestack, block);
}
