use std::process::exit;

use celsium::{ block::Block, compile_time_checker::CompileTimeChecker };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::errors;


pub fn id(
    node: AstNode,
    title: &str,
    block: &mut Block,
    typestack: &mut CompileTimeChecker
) {
    if title.starts_with("ID") {
        let type_if_exists = typestack.check_var(node.get_value().unwrap());
        if type_if_exists.is_none() {
            errors::undefined_var(
                format!("Mainīgais ar nosaukumu '{}' nav definēts", node.get_value().unwrap()),
                &typestack.source_files[typestack.current_file],
                &typestack.source_file_paths[typestack.current_file],
                node.get_position().unwrap().line,
                node.get_position().unwrap().column
            );
            exit(0);
        } else {
            typestack.push(type_if_exists.unwrap());
        }
        block.load_variable(node.get_value().unwrap());
    }
}
