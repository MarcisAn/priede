use celsium::{
    block::Block,
    compiletime_helper::CompileTimeHelper,
    module::{ FuncArg, FunctionSignature, VISIBILITY },
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use super::parse_ast;

pub fn func_def(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeHelper,
    is_wasm: bool
) {
    if title == "func_def" {
        let mut body = Block::new(
            if node.children_count() > 2 {
                node.child(2).id()
            } else {
                node.child(1).id()
            }
        );
        let mut args: Vec<FuncArg> = vec![];

        if node.children_count() > 2 {
            //when the function takes arguments
            parse_ast(node.child(2), &mut body, is_wasm, typestack);
            for arg in node.child(1).children() {
                args.push(FuncArg {
                    name: arg.child(1).get_value().unwrap().to_string(),
                    arg_type: match arg.child(0).to_string().as_str() {
                        "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
                        "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
                        "TEXT" => celsium::BUILTIN_TYPES::MAGIC_INT,
                        "FLOAT" => celsium::BUILTIN_TYPES::FLOAT,
                        _ => panic!(),
                    },
                });
            }
        } else {
            parse_ast(node.child(1), &mut body, is_wasm, typestack);
        }

        block.define_function(body, VISIBILITY::PUBLIC, FunctionSignature {
            name: node.child(0).get_value().unwrap().to_string(),
            return_type: celsium::module::FunctionReturnType::NONE,
            args: args,
        })
    }
}
