use std::{process::exit};

use celsium::{ block::Block, compile_time_checker::{self, CompileTimeChecker} };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{errors, util::get_closest_block, util::get_closest_scope};




pub fn id(node: AstNode, title: &str, block: &mut Block, typestack: &mut CompileTimeChecker) {
    if title.starts_with("ID") {
        let var_name = node.get_value().unwrap();
        let var_id = get_closest_scope(var_name.to_string(), block.ast_id, typestack, node);

        if var_id.is_none() {
            errors::undefined_var(
                format!("Mainīgais ar nosaukumu '{}' nav definēts šajā blokā.", node.get_value().unwrap()),
                &typestack.source_files[typestack.current_file],
                &typestack.source_file_paths[typestack.current_file],
                node.get_position().unwrap().line,
                node.get_position().unwrap().column
            );
            exit(0);
        } else {
            typestack.push(typestack.defined_variables[var_id.unwrap()].data_type.clone());
            block.load_variable(var_id.unwrap());
        }
    }
}
