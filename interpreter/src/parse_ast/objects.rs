use celsium::{ ObjectFieldType, block::Block};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

use crate::{Compiler, parse_ast::parse_ast, util::get_data_type_from_id};

pub fn objects(node: AstNode, title: &str, compiler: &mut Compiler, block: &mut Block) {
    if title == "object_def" {
        let object_title = node.child(0).get_value().unwrap().to_string();
        let mut field_counter = 1;
        let mut fields = vec![];
        while field_counter < node.children_count() {
            fields.push(ObjectFieldType {
                name: node.child(field_counter).child(1).get_value().unwrap().to_string(),
                data_type: get_data_type_from_id(
                    compiler,
                    node.child(field_counter).child(0).get_value().unwrap(),
                    node
                ),
            });
            field_counter += 1;
        }
        compiler.helper.define_struct(object_title, fields);
    }

    if title == "object" {
        let mut fields = vec![];
        let mut field_names = vec![];
        for field in node.children().iter().rev() {
            parse_ast(field.child(1), compiler, block);
            let field_name = field.child(0).get_value().unwrap().to_string();
            let field = ObjectFieldType {
                name: field_name.clone(),
                data_type: compiler.typestack.pop().unwrap(),
            };
            field_names.push(field_name);
            fields.push(field);
        }
        compiler.typestack.push(celsium::BuiltinTypes::Object { fields });
        block.create_object(field_names);
    }
}
