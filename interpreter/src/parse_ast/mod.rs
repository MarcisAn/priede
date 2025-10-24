use std::process::exit;

use celsium::{ block::Block, BuiltinTypes, ObjectFieldType };
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
use array_def::array_def;
mod index;
use index::index;

use crate::{
    errors::variable_not_indexable,
    util::get_data_type_from_id,
    Compiler,
};

pub fn parse_ast(node: AstNode, compiler: &mut Compiler, block: &mut Block) {
    let title = node.get_symbol().to_string();

    if title == "block" {
        for i in node.children() {
            parse_ast(i, compiler, block);
        }
    }

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
            parse_ast(field.child(1), compiler, block);
            let field_name = field.child(0).get_value().unwrap().to_string();
            let field = ObjectFieldType {
                name: field_name.clone(),
                data_type: compiler.typestack.pop().unwrap(),
            };
            field_names.push(field_name);
            fields.push(field);
        }
        compiler.typestack.push(celsium::BuiltinTypes::Object { fields });
        block.create_object(field_names);
    }

    
    index(node, &title, compiler, block);
    id(node, &title, compiler, block);
    return_st(node, &title, compiler, block);
    if_stat(node, &title, compiler, block);
    loops(node, &title, compiler, block);
    func_def(node, &title, compiler, block);
    func_call(node, &title, compiler, block);
    array(node, &title, compiler, block);
    array_def(node, &title, compiler, block);
    math_ops(node, &title, compiler, block);
    comparisons(node, &title, compiler, block);
    id_assign(node, &title, compiler, block);
    parse_constants(node, &title, compiler, block);
    var_def(node, &title, compiler, block);
    include(node, &title, compiler, block);
}
