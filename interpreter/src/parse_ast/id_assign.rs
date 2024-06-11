use celsium::{ block::Block, bytecode::BINOP, compiletime_helper::CompileTimeHelper };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::util::get_closest_scope;

use super::parse_ast;

pub fn id_assign(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "id_assign" {
        let operator = node.child(1).get_value().unwrap();
        let var_name = node.child(0).get_value().unwrap();
        let var_id = get_closest_scope(
            var_name.to_string(),
            block.scope.clone(),
            typestack,
            node
        ).unwrap();

        if operator == ":" {
            parse_ast(node.child(2), block, is_wasm, typestack);
        } else if operator == "+:" {
            block.load_variable(var_id);
            let data_type = typestack.get_var_type(var_id).unwrap();
            typestack.push(data_type);
            parse_ast(node.child(2), block, is_wasm, typestack);
            typestack.binop(BINOP::ADD);
            block.binop(BINOP::ADD);
        } else if operator == "-:" {
            block.load_variable(var_id);
            let data_type = typestack.get_var_type(var_id).unwrap();
            typestack.push(data_type);
            parse_ast(node.child(2), block, is_wasm, typestack);
            block.binop(BINOP::SUBTRACT);
            typestack.binop(BINOP::SUBTRACT);
        } else if operator == "*:" {
            block.load_variable(var_id);
            let data_type = typestack.get_var_type(var_id).unwrap();
            typestack.push(data_type);
            parse_ast(node.child(2), block, is_wasm, typestack);
            block.binop(BINOP::MULTIPLY);
            typestack.binop(BINOP::MULTIPLY);
        } else if operator == "/:" {
            block.load_variable(var_id);
            let data_type = typestack.get_var_type(var_id).unwrap();
            typestack.push(data_type);
            parse_ast(node.child(2), block, is_wasm, typestack);
            block.binop(BINOP::DIVIDE);
        } else if operator == "++" {
            block.load_variable(var_id);
            let data_type = typestack.get_var_type(var_id).unwrap();
            typestack.push(data_type);
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, "1");
            typestack.binop(BINOP::ADD);
            block.binop(BINOP::ADD);
        } else if operator == "--" {
            block.load_variable(var_id);
            let data_type = typestack.get_var_type(var_id).unwrap();
            typestack.push(data_type);
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, "1");
            block.binop(BINOP::SUBTRACT);
        }
        block.assign_variable(var_id);
    } else if title == "array_assign" {
        let var_name = node.child(0).child(0).get_value().unwrap();
        let var_id = get_closest_scope(
            var_name.to_string(),
            block.scope.clone(),
            typestack,
            node
        ).unwrap();

        parse_ast(node.child(1), block, is_wasm, typestack);
        parse_ast(node.child(0).child(1), block, is_wasm, typestack);
        block.assign_to_array(var_id);
    }
}
