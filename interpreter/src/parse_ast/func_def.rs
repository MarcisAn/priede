use celsium::{ block::Block, module::{ FuncArg, Function, FunctionSignature }, BuiltinTypes };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{ util::{ self, get_data_type_from_id }, Compiler };

use super::parse_ast;

pub fn func_def(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "func_def" {
        let is_exported = node.child(0).get_symbol().to_string() == "EXPORT";

        let func_name = node
            .child(0 + (is_exported as usize))
            .get_value()
            .unwrap()
            .to_string();

        let mut body = Block::new(
            block.scope.change_ast_id(node.child(node.children_count() - 1).id())
        );
        let mut args: Vec<FuncArg> = vec![];

        if node.children_count() >= 3 + (is_exported as usize) {
            //when the function takes arguments
            let mut return_type: Option<BuiltinTypes> = None;

            let is_returning =
                node.child(2 + (is_exported as usize)).to_string() == "func_return_type";
            let args_tree = node.child(1 + (is_exported as usize)).children();
            for arg in args_tree.iter().rev() {
                let is_mutable = if arg.children_count() == 3 { true } else { false };
                let arg_name = arg
                    .child(1 + (is_mutable as usize))
                    .get_value()
                    .unwrap()
                    .to_string();

                let data_type_str = arg.child(0 + (is_mutable as usize)).get_value().unwrap();

                let data_type_marked = get_data_type_from_id(compiler, data_type_str, node);
                let var_id = compiler.helper.def_var(
                    arg_name.clone(),
                    data_type_marked.clone(),
                    body.scope.clone(),
                    is_exported
                );
                args.push(FuncArg {
                    name: arg_name.clone(),
                    arg_type: data_type_marked,
                    mutable: is_mutable,
                    local_var_id: Some(var_id.unwrap())
                });
                
                body.define_variable(var_id.unwrap(), arg_name, arg.id());
            }

            if is_returning {
                return_type = Some(
                    util::get_data_type_from_id(
                        compiler,
                        node
                            .child(2 + (is_exported as usize))
                            .child(0)
                            .get_value()
                            .unwrap(),
                        node
                    )
                );
            }

            compiler.helper.def_function(
                func_name.clone(),
                args.clone(),
                block.scope.clone(),
                is_exported,
                return_type
            );
            parse_ast(
                node.child(2 + (is_returning as usize) + (is_exported as usize)),
                compiler,
                &mut body
            );
        } else {
            let is_returning =
                node.child(1 + (is_exported as usize)).to_string() == "func_return_type";

            let mut return_type: Option<BuiltinTypes> = None;
            if is_returning {
                return_type = Some(
                    util::get_data_type_from_id(
                        compiler,
                        node
                            .child(1 + (is_exported as usize))
                            .child(0)
                            .get_value()
                            .unwrap(),
                        node
                    )
                );
            }

            compiler.helper.def_function(
                func_name.clone(),
                args.clone(),
                block.scope.clone(),
                is_exported,
                return_type
            );
            parse_ast(node.child(1 + (is_exported as usize)), compiler, &mut body);
        }

        compiler.functions.push(Function {
            signature: FunctionSignature {
                name: func_name,
                return_type: None,
                args: args,
            },
            body: body,
        });
    }
}
