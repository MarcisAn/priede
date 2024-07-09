use std::process::exit;

use crate::{errors::math_error, Compiler};
use celsium::{ block::Block, bytecode::BINOP, compiletime_helper::CompileTimeHelper };
use hime_redist::ast::AstNode;

use super::parse_ast;

pub fn math_ops(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler
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
            compiler
        );
    }
}
fn calculate(
    binop: BINOP,
    node: AstNode,
    compiler: &mut Compiler
) {
    println!("calculate called");
    parse_ast(node.child(0), compiler);
    parse_ast(node.child(1), compiler);
    println!("{:?} sec", compiler.helper.stack);
    let res = compiler.helper.binop(binop.clone());
    if res.is_none() {
        math_error(&mut compiler.helper, node);
        exit(0);
    }
    println!("{:?}", binop);
    compiler.block.binop(binop);
}
