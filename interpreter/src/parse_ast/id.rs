use celsium::{ block::{self, Block}};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{errors, util::lookup_variable, util, Compiler};

pub fn id(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title.starts_with("ID") {
        let var_name = node.get_value().unwrap();
        let var_id = lookup_variable(var_name.to_string(), block.scope.clone(), &mut compiler.helper, node);
        let (line, col_start, length) = util::get_node_position_and_span_unicode(node);
        if var_id.is_none() {
           compiler.add_error(
                errors::CompileTimeErrorType::VariableNotDefined {
                    varname: node.get_value().unwrap().to_string(),
                },
                node
            );
        } else {
            let data_type = compiler.helper.get_var_type(var_id.unwrap()).unwrap();
            compiler.typestack.push(data_type.clone());
            block.load_variable(var_id.unwrap(), block::TextSpan { line, col_start, length });
        }
    }
}
