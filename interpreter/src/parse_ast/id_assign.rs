use celsium::{
    block::Block, bytecode::BINOP, compile_time_checker::CompileTimeChecker
};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use super::parse_ast;

pub fn id_assign(
    node: AstNode,
    block: &mut Block,
    typestack: &mut CompileTimeChecker,
    is_wasm: bool,
) {
    let operator = node.child(1).get_value().unwrap();
        let var_name = node.child(0).get_value().unwrap();
        if operator == ":" {
            parse_ast(node.child(2), block, is_wasm, typestack);
        } else if operator == "+:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block, is_wasm, typestack);
            typestack.binop(BINOP::ADD);
            block.binop(BINOP::ADD);
        } else if operator == "-:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block, is_wasm, typestack);
            block.binop(BINOP::SUBTRACT);
        } else if operator == "*:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block, is_wasm, typestack);
            block.binop(BINOP::MULTIPLY);
        } else if operator == "/:" {
            block.load_variable(var_name);
            parse_ast(node.child(2), block, is_wasm, typestack);
            block.binop(BINOP::DIVIDE);
        } else if operator == "++" {
            block.load_variable(var_name);
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, "1");
            typestack.binop(BINOP::ADD);
            block.binop(BINOP::ADD);
        } else if operator == "--" {
            block.load_variable(var_name);
            block.load_const(celsium::BUILTIN_TYPES::MAGIC_INT, "1");
            block.binop(BINOP::SUBTRACT);
        }
        block.assign_variable(var_name);
}
