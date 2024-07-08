use array_def::array_def;
use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, BuiltinTypes, ObjectFieldType };
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

use crate::{ errors, util::{ self, get_data_type_from_id }, Compiler };

pub fn parse_ast(
    node: AstNode,
    compiler: &mut Compiler
) {
    let title = node.get_symbol().to_string();

    if title == "block" {
        for i in node.children() {
            parse_ast(i, compiler);
        }
    }
/* 
    if title == "dot_call" {
        parse_ast(node.child(0), block, is_wasm, typestack);

        match typestack.pop().unwrap() {
            BuiltinTypes::Object { fields } => block.get_object_field(node.child(1).get_value().unwrap().to_string()),
            BuiltinTypes::MagicInt => todo!(),
            BuiltinTypes::Bool => todo!(),
            BuiltinTypes::String => todo!(),
            BuiltinTypes::Float => todo!(),
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
    }*/

    if title == "object_def" {
        let object_title = node.child(0).get_value().unwrap().to_string();
        let mut field_counter = 1;
        let mut fields = vec![];
        while field_counter < node.children_count() {
            fields.push(ObjectFieldType {
                name: node.child(field_counter).child(1).get_value().unwrap().to_string(),
                data_type: get_data_type_from_id(
                    &mut compiler.helper,
                    node.child(field_counter).child(0).get_value().unwrap(),
                    node
                ),
            });
            field_counter += 1;
        }
        compiler.helper.define_struct(object_title, fields);
    }

    if title == "object" {
        let mut fields = vec![];
        let mut field_names = vec![];
        for field in node.children().iter().rev() {
            parse_ast(field.child(1), compiler);
            let field_name = field.child(0).get_value().unwrap().to_string();
            let field = ObjectFieldType {
                name: field_name.clone(),
                data_type: compiler.helper.pop().unwrap(),
            };
            field_names.push(field_name);
            fields.push(field);
        }
        compiler.helper.push(celsium::BuiltinTypes::Object { fields });
        compiler.block.create_object(field_names);
    }

    if title == "dot_call" {
        parse_ast(node.child(0), compiler);
        let origin_type = compiler.helper.pop().unwrap();
        println!("{:?}", origin_type);
        match origin_type {
            BuiltinTypes::MagicInt => todo!(),
            BuiltinTypes::Bool => todo!(),
            BuiltinTypes::String => todo!(),
            BuiltinTypes::Object { fields } =>
                compiler.block.get_object_field(node.child(1).get_value().unwrap().to_string()),
            BuiltinTypes::Float => todo!(),
            BuiltinTypes::Array { element_type } => {},
        }
        if node.child(1).get_value().unwrap() == "garums" {
            let array_name = node.child(0).get_value().unwrap().to_string();
            let array_id = util::get_closest_scope(
                array_name.clone(),
                compiler.block.scope.clone(),
                &mut compiler.helper,
                node
            );
            if array_id.is_none() {
                errors::undefined_var(
                    format!("Saraksts `{}` nav definēts", array_name),
                    &mut compiler.helper,
                    node
                );
            }
            compiler.block.get_array_length(array_id.unwrap());
            compiler.helper.push(celsium::BuiltinTypes::MagicInt);
        }
    }

    id(node, &title, compiler);
    return_st(node, &title, compiler);
    if_stat(node, &title, compiler);
    loops(node, &title, compiler);
    func_def(node, &title, compiler);
    func_call(node, &title, compiler);
    array(node, &title, compiler);
    math_ops(node, &title, compiler);
    comparisons(node, &title, compiler);
    id_assign(node, &title, compiler);
    parse_constants(node, &title, compiler);
    var_def(node, &title, compiler);
    include(node, &title, compiler);
}
