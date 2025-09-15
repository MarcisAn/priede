use std::process::exit;

use crate::Compiler;
use celsium::{block::Block, bytecode::BINOP};
use hime_redist::{ast::AstNode, symbols::SemanticElementTrait};

use super::parse_ast;

pub fn math_ops(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler, block: &mut Block
) {
    if title == "plus" || title == "minus" || title == "reiz" || title == "dal" || title == "atlik" {
        calculate(
            match title {
                "plus" => BINOP::Add,
                "minus" => BINOP::Subtract,
                "reiz" => BINOP::Multiply,
                "dal" => BINOP::Divide,
                "atlik" => BINOP::Remainder,
                _ => panic!(),
            },
            node,
            compiler,
            block
        );
    }
}
fn calculate(
    binop: BINOP,
    node: AstNode,
    compiler: &mut Compiler, block: &mut Block
) {
    parse_ast(node.child(0), compiler, block);
    // println!("span {:?}", node.child(0).get_total_position_and_span());
    parse_ast(node.child(2), compiler, block);
    // println!("span {:?}", node.child(2).get_total_position_and_span());

    let res = compiler.typestack.binop(binop.clone());
    if res.is_none() {
        compiler.add_error(
                crate::compiler::CompileErrorType::MathTypes,
                node.child(1).get_position().unwrap().line,
                node.child(1).get_position().unwrap().column,
                node.child(1).get_span().unwrap().length
            );
        exit(0);
    }
    block.binop(binop);
}
