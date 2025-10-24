use std::process::exit;

use celsium::{ block::{self, Block}, compiletime_helper::CompileTimeHelper};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait, utils };

use crate::{errors, util::get_closest_scope, util, Compiler};

pub fn id(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title.starts_with("ID") {
        let var_name = node.get_value().unwrap();
        let var_id = get_closest_scope(var_name.to_string(), block.scope.clone(), &mut compiler.helper, node);
        let (line, col_start, length) = util::get_node_position_and_span_unicode(node);
        if var_id.is_none() {
            errors::undefined_var(
                format!("Mainīgais ar nosaukumu '{}' nav definēts šajā blokā.", node.get_value().unwrap()),
                &mut compiler.helper,
                node
            );
            exit(0);
        } else {
            let data_type = compiler.helper.get_var_type(var_id.unwrap()).unwrap();
            compiler.typestack.push(data_type.clone());
            block.load_variable(var_id.unwrap(), block::TextSpan { line, col_start, length });
        }
    }
}
