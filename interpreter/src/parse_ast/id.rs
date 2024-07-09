use std::process::exit;

use celsium::{ block::Block, compiletime_helper::CompileTimeHelper};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{errors, util::get_closest_scope, Compiler};

pub fn id(node: AstNode, title: &str, compiler: &mut Compiler) {
    if title.starts_with("ID") {
        let var_name = node.get_value().unwrap();
        let var_id = get_closest_scope(var_name.to_string(), compiler.block.scope.clone(), &mut compiler.helper, node);

        if var_id.is_none() {
            errors::undefined_var(
                format!("Mainīgais ar nosaukumu '{}' nav definēts šajā blokā.", node.get_value().unwrap()),
                &mut compiler.helper,
                node
            );
            exit(0);
        } else {
            let data_type = compiler.helper.get_var_type(var_id.unwrap()).unwrap();
            compiler.helper.push(data_type.clone());
            compiler.block.load_variable(var_id.unwrap());
        }
    }
}
