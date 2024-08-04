use celsium::{ block::Block, compiletime_helper::CompileTimeHelper };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ errors, util::{ self, get_data_type_from_id }, Compiler };

use super::parse_ast;

pub fn array_def(
    node: AstNode,
    title: &str,
    compiler: &mut Compiler,
    is_exported: bool
) {
    if title == "array_def" {
        let array_name = node
            .child(2 + (is_exported as usize))
            .get_value()
            .unwrap();
        //user marked data type
        let data_type_str = node
            .child(0 + (is_exported as usize))
            .get_value()
            .unwrap();

        let data_type_marked = get_data_type_from_id(&mut compiler.helper, data_type_str, node);

        let mut init_value_counter = 0;
        for i in node.child(3 + (is_exported as usize)).children() {
            parse_ast(i, compiler);
            let type_of_init_val = compiler.helper.pop();

            let mut should_objects_error = false;

            let are_object_types_eq = util::compare_object_types(
                &type_of_init_val.clone().unwrap(),
                &data_type_marked
            );

            if are_object_types_eq.is_ok() {
                should_objects_error = !are_object_types_eq.unwrap();
            }
            if
                type_of_init_val.clone().unwrap() != data_type_marked.clone() &&
                should_objects_error
            {
                errors::array_element_wrong_type(
                    array_name.to_owned(),
                    init_value_counter,
                    data_type_marked.clone(),
                    type_of_init_val.unwrap().clone(),
                    &mut compiler.helper,
                    node
                );
            }
            init_value_counter += 1;
        }
        let var_id = compiler.helper.def_array(
            array_name,
            data_type_marked,
            node
                .child(3 + (is_exported as usize))
                .children()
                .len(),
            compiler.block.scope.clone(),
            is_exported
        );
        compiler.block.define_array(
            node
                .child(3 + (is_exported as usize))
                .children()
                .len(),
            var_id
        )
    }
}
