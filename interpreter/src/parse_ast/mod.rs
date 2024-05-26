use std::process::exit;
use celsium::{ block::Block, compile_time_checker::CompileTimeChecker };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

mod id_assign;
use id_assign::id_assign;
mod math_ops;
use math_ops::math_ops;
mod stumbrs;
use stumbrs::stumbrs_define;
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

use crate::errors;

pub fn parse_ast(
    node: AstNode,
    block: &mut Block,
    is_wasm: bool,
    typestack: &mut CompileTimeChecker
) {
    let title = node.get_symbol().to_string();
    
    if title == "block" {
        for i in node.children() {
            parse_ast(i, block, is_wasm, typestack);
        }
    }

    id(node, &title, block, typestack);
    return_st(node, &title, block, typestack, is_wasm);
    if_stat(node, &title, block, typestack, is_wasm);
    stumbrs_define(node, &title, block, typestack, is_wasm);
    loops(node, &title, block, typestack, is_wasm);
    func_def(node, &title, block, typestack, is_wasm);
    func_call(node, &title, block, typestack, is_wasm);
    array(node, &title, block, typestack, is_wasm);
    math_ops(node, &title, block, typestack, is_wasm);
    comparisons(node, &title, block, typestack, is_wasm);
    id_assign(node, &title, block, typestack, is_wasm);
    parse_constants(node, &title, typestack, block);
    var_def(node, title, typestack, is_wasm, block);
}
