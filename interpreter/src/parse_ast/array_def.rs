use celsium::{ block::Block, compiletime_helper::CompileTimeHelper, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ errors::{self, incorrect_variable_init_value}, util::{ self, get_closest_node_location, get_data_type_from_id }, Compiler };

use super::parse_ast;

pub fn array_def(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler,
    is_exported: bool,
    block: &mut Block
) {
    let data_type_str = node
        .child(0 + (is_exported as usize))
        .get_value()
        .unwrap();
    let data_type_marked = BuiltinTypes::Array {
        element_type: Box::new(
            util::get_data_type_from_id(&mut compiler.helper, data_type_str, node)
        ),
        length: None,
    };

    let varname = node
        .child(2 + (is_exported as usize))
        .get_value()
        .unwrap()
        .to_string();

    //parse the init value
    parse_ast(node.child(3 + (is_exported as usize)), compiler, block);
    let typ_of_init_value = compiler.typestack.pop().unwrap();

    let init_value_to_compare = match typ_of_init_value.clone() {
        BuiltinTypes::Array { element_type, length } =>
            BuiltinTypes::Array { element_type: element_type, length: None },
        _ => panic!(),
    };

    if init_value_to_compare.clone() != data_type_marked {
        let erroring_node = node.child(0 + (is_exported as usize));
        incorrect_variable_init_value(&data_type_marked, &typ_of_init_value, &mut compiler.helper, node);
        return;
    }

    let number_of_elements = match typ_of_init_value {
        BuiltinTypes::Array { element_type: _, length } => { length }
        _ => todo!(),
    };
    let var_id = compiler.helper.def_var(
        varname.clone(),
        data_type_marked.clone(),
        block.scope.clone(),
        is_exported
    );
    block.define_variable(var_id.unwrap());
}
