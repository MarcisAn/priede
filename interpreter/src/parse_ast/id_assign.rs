use celsium::bytecode::BINOP;
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{util::get_closest_scope, Compiler};

use super::parse_ast;

pub fn id_assign(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler
) {
    if title == "id_assign" {
        let operator = node.child(1).get_value().unwrap();
        let var_name = node.child(0).get_value().unwrap();
        let var_id = get_closest_scope(
            var_name.to_string(),
            compiler.block.scope.clone(),
            &mut compiler.helper,
            node
        ).unwrap();


        if operator == ":" {
            parse_ast(node.child(2), compiler);
        } else if operator == "+:" {
            //compiler.block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.helper.push(data_type, compiler.register_counter);
            let a_reg = compiler.helper.get_top().unwrap().register_id;
            compiler.register_counter += 1;
            parse_ast(node.child(2), compiler);
            let b_reg = compiler.helper.get_top().unwrap().register_id;
            compiler.helper.binop(BINOP::Add, compiler.register_counter);

            compiler.block.binop(BINOP::Add, a_reg, b_reg , compiler.register_counter);
            compiler.register_counter += 1;
        }/*  else if operator == "-:" {
            compiler.block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.helper.push(data_type);
            parse_ast(node.child(2), compiler);
            compiler.block.binop(BINOP::Subtract);
            compiler.helper.binop(BINOP::Subtract);
        } else if operator == "*:" {
            compiler.block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.helper.push(data_type);
            parse_ast(node.child(2), compiler);
            compiler.block.binop(BINOP::Multiply);
            compiler.helper.binop(BINOP::Multiply);
        } else if operator == "/:" {
            compiler.block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.helper.push(data_type);
            parse_ast(node.child(2), compiler);
            compiler.block.binop(BINOP::Divide);
        } else if operator == "++" {
            compiler.block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.helper.push(data_type);
            compiler.block.load_const(celsium::BuiltinTypes::MagicInt, "1");
            compiler.helper.binop(BINOP::Add);
            compiler.block.binop(BINOP::Add);
        } else if operator == "--" {
            compiler.block.load_variable(var_id);
            let data_type = compiler.helper.get_var_type(var_id).unwrap();
            compiler.helper.push(data_type);
            compiler.block.load_const(celsium::BuiltinTypes::MagicInt, "1");
            compiler.block.binop(BINOP::Subtract);
        }*/
        compiler.block.assign_variable(var_id, compiler.register_counter);
        compiler.register_counter += 1;
    } else if title == "array_assign" {
        let var_name = node.child(0).child(0).get_value().unwrap();
        let var_id = get_closest_scope(
            var_name.to_string(),
            compiler.block.scope.clone(),
            &mut compiler.helper,
            node
        ).unwrap();

        parse_ast(node.child(1), compiler);
        let value_reg = compiler.helper.get_top().unwrap().register_id;
        parse_ast(node.child(0).child(1), compiler);
        let index_reg = compiler.helper.get_top().unwrap().register_id;
        compiler.block.assign_to_array(var_id, value_reg, index_reg);
    }
}
