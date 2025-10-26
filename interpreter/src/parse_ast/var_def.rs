use celsium::{ block::Block, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };
use crate::{ errors, util::{ self, get_object_fields }, Compiler };

use super::parse_ast;



pub fn var_def(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "var_def" {
        let is_exported = node.child(0).get_symbol().to_string() == "EXPORT";

        //user marked data type
        let data_type_str = node
            .child(0 + (is_exported as usize))
            .get_value()
            .unwrap();

        let data_type_marked = util::get_data_type_from_id(compiler, data_type_str, node);

        //parse the init value
        parse_ast(node.child(2 + (is_exported as usize)), compiler, block);

        //get the type of the init value
        let typ_of_init_value = compiler.typestack.pop().unwrap();

        let is_object = util::is_type_object(&typ_of_init_value);

        // let are_types_correct = if is_object {
        //     util::compare_object_types(&typ_of_init_value, &data_type_marked).unwrap()
        // } else {
        //     typ_of_init_value.clone() == data_type_marked
        // };

        let are_types_correct = util::are_types_equal(&data_type_marked, &typ_of_init_value);


        let varname = node
            .child(1 + (is_exported as usize))
            .get_value()
            .unwrap()
            .to_string();

        if !are_types_correct {
            compiler.add_error(
                errors::CompileTimeErrorType::WrongVariableInitValue {
                    varname: varname.clone(),
                    expected_type: data_type_marked.clone(),
                    found_type: typ_of_init_value.clone(),
                },
                node
            );
        }

        if is_object {
            let fields = get_object_fields(&typ_of_init_value).unwrap();

            let object_id = compiler.helper.def_object(
                varname.clone(),
                block.scope.clone(),
                is_exported,
                fields.clone()
            );

            if object_id.is_none() {
                compiler.add_error(
                    errors::CompileTimeErrorType::VariableAlreadyDefined {
                        varname: varname.clone(),
                    },
                    node
                );
            }
            let mut field_names: Vec<String> = vec![];
            for field in fields.clone() {
                field_names.push(field.name);
            }
            block.define_object(object_id.unwrap());
        } else {
            let var_id = compiler.helper.def_var(
                varname.clone(),
                data_type_marked.clone(),
                block.scope.clone(),
                is_exported
            );
            if var_id.is_none() {
                compiler.add_error(
                    errors::CompileTimeErrorType::VariableAlreadyDefined {
                        varname: varname.clone(),
                    },
                    node
                );
            } else {
                block.define_variable(var_id.unwrap());
            }
        }
    }
}
