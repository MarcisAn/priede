use std::process::exit;

use celsium::{ block::Block, compiletime_helper::CompileTimeHelper};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{errors, util::get_closest_scope};




pub fn id(node: AstNode, title: &str, block: &mut Block, typestack: &mut CompileTimeHelper) {
    if title.starts_with("ID") {
        let var_name = node.get_value().unwrap();
        let var_id = get_closest_scope(var_name.to_string(), block.scope.clone(), typestack, node);

        if var_id.is_none() {
            errors::undefined_var(
                format!("Mainīgais ar nosaukumu '{}' nav definēts šajā blokā.", node.get_value().unwrap()),
                typestack,
                node
            );
            exit(0);
        } else {
            let data_type = typestack.get_var_type(var_id.unwrap()).unwrap();
            typestack.push(data_type);
            block.load_variable(var_id.unwrap());
        }
    }
}
