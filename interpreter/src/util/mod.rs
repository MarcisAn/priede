use celsium::{
    compiletime_helper::{ CompileTimeHelper, CompileTimeImport },
    BuiltinTypes,
    ObjectFieldType,
    Scope,
};
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait, text::TextPosition };

use crate::errors;

fn print<'a>(node: AstNode, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", if crossings[i] { "|   " } else { "    " });
            i += 1;
        }
        print!("+-> ");
    }
    println!("{:}", node);
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        print(children.at(i), child_crossings);
        i += 1;
    }
}
pub fn print_ast(node: AstNode) {
    print(node, Vec::<bool>::new());
}
pub fn get_unicode_chars(node: AstNode) -> usize {
    let ast = format_ast(node);
    let mut unicode_chars = 0;
    for character in ast.chars() {
        let is_unicode = character.is_ascii();
        if !is_unicode {
            unicode_chars += 1;
        }
    }
    unicode_chars
}
pub fn get_node_position_and_span_unicode(node: AstNode) -> (usize, usize, usize) {
    let init = node.get_total_position_and_span().unwrap();
    let uniode_char_count = get_unicode_chars(node);
    let span = init.1.length - uniode_char_count;
    return (init.0.line, init.0.column, span);
}
fn format_tree(node: AstNode, crossings: Vec<bool>, output: &mut String) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            if crossings[i] {
                output.push_str("|   ");
            } else {
                output.push_str("    ");
            }
            i += 1;
        }
        output.push_str("+-> ");
    }

    output.push_str(&node.to_string());
    output.push('\n');

    let children = node.children();
    i = 0;
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        format_tree(children.at(i), child_crossings, output);
        i += 1;
    }
}

pub fn format_ast(node: AstNode) -> String {
    let mut output = String::new();
    format_tree(node, Vec::new(), &mut output);
    output
}

pub fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn get_closest_scope(
    target_name: String,
    starting_scope: Scope,
    compilehelper: &mut CompileTimeHelper,
    node: AstNode
) -> Option<usize> {
    let starting_ast_id = starting_scope.ast_id;
    for var in compilehelper.defined_variables.clone() {
        let import_to_search_for = CompileTimeImport {
            name: var.clone().name,
            origin: var.clone().scope.module_path,
            imported_into: starting_scope.clone().module_path,
        };
        if compilehelper.imports.contains(&import_to_search_for) && var.is_exported {
            if var.name == target_name {
                return Some(var.id);
            }
        } else {
            if var.name == target_name && var.scope.ast_id == starting_ast_id {
                return Some(var.id);
            }
        }
    }
    for var in compilehelper.defined_arrays.clone() {
        let import_to_search_for = CompileTimeImport {
            name: var.clone().name,
            origin: var.clone().scope.module_path,
            imported_into: starting_scope.clone().module_path,
        };
        if compilehelper.imports.contains(&import_to_search_for) {
            if var.name == target_name {
                return Some(var.id);
            }
        } else {
            if var.name == target_name && var.scope.ast_id == starting_ast_id {
                return Some(var.id);
            }
        }
    }
    for var in compilehelper.defined_objects.clone() {
        let import_to_search_for = CompileTimeImport {
            name: var.clone().name,
            origin: var.clone().scope.module_path,
            imported_into: starting_scope.clone().module_path,
        };
        if compilehelper.imports.contains(&import_to_search_for) {
            if var.name == target_name {
                return Some(var.id);
            }
        } else {
            if var.name == target_name && var.scope.ast_id == starting_ast_id {
                return Some(var.id);
            }
        }
    }
    for function in &compilehelper.defined_functions {
        let import_to_search_for = CompileTimeImport {
            name: function.clone().name,
            origin: function.clone().scope.module_path,
            imported_into: starting_scope.clone().module_path,
        };
        if compilehelper.imports.contains(&import_to_search_for) {
            if function.name == target_name {
                return Some(function.id);
            }
        } else {
            if function.name == target_name && function.scope.ast_id == starting_ast_id {
                return Some(function.id);
            }
        }
    }
    let node_parrent = node.parent();
    if node_parrent.is_none() {
        return None;
    }
    get_closest_scope(
        target_name,
        Scope { ast_id: node_parrent.unwrap().id(), module_path: starting_scope.module_path },
        compilehelper,
        node_parrent.unwrap()
    )
}

pub fn get_closest_node_location(node: AstNode) -> Option<TextPosition> {
    if node.get_position().is_some() {
        return Some(node.get_position().unwrap());
    }
    for child in node.children() {
        return get_closest_node_location(child);
    }
    None
}

pub fn get_furthest_node_location(node: AstNode) -> Option<TextPosition> {
    if node.get_position().is_some() {
        return Some(node.get_position().unwrap());
    }
    for child in node.children().iter().rev() {
        return get_closest_node_location(child);
    }
    panic!();
}

pub fn data_type_from_str(inp: &str) -> Option<BuiltinTypes> {
    return Some(match inp {
        "sk" => celsium::BuiltinTypes::Int,
        "skaitlis" => celsium::BuiltinTypes::Int,
        "būls" => celsium::BuiltinTypes::Bool,
        "bl" => celsium::BuiltinTypes::Bool,
        "teksts" => celsium::BuiltinTypes::String,
        "tk" => celsium::BuiltinTypes::String,
        "decim" => celsium::BuiltinTypes::Float,
        _ => {
            return None;
        }
    });
}

fn format_object_fields(fields: &Vec<ObjectFieldType>) -> String {
    let mut formated = "{\n".to_string();
    for field in fields.iter().rev() {
        formated += &format!("{}: {}\n", str_from_data_type(&field.data_type.clone()), &field.name);
    }
    formated += "}";
    formated
}

pub fn str_from_data_type(inp: &BuiltinTypes) -> String {
    match inp {
        BuiltinTypes::Int => "skaitlis".into(),
        BuiltinTypes::Bool => "būls".into(),
        BuiltinTypes::String => "teksts".into(),
        BuiltinTypes::Object { fields } => format!("objekts\n{}", format_object_fields(fields)),
        BuiltinTypes::Float => "decimālskaitlis".into(),
        BuiltinTypes::Array { element_type, length: _ } =>
            format!("Saraksts[`{}`]", str_from_data_type(element_type)),
    }
}

pub fn get_data_type_from_id(
    compilehelper: &mut CompileTimeHelper,
    data_type_str: &str,
    node: AstNode
) -> BuiltinTypes {
    let data_type_marked_option = data_type_from_str(data_type_str);

    let data_type_marked: BuiltinTypes;
    if data_type_marked_option.is_none() {
        let struct_exists = compilehelper.struct_exists(data_type_str);
        if struct_exists.is_some() {
            data_type_marked = BuiltinTypes::Object {
                fields: struct_exists.unwrap().fields,
            };
        } else {
            errors::notexistant_type(data_type_str.to_owned(), node, compilehelper);
            panic!(); //to get rid of undefined error. Not needed, because error exits.
        }
    } else {
        data_type_marked = data_type_marked_option.unwrap();
    }
    return data_type_marked;
}

pub fn compare_object_types(a: &BuiltinTypes, b: &BuiltinTypes) -> Result<bool, String> {
    let mut fields_a: Vec<ObjectFieldType>;
    let mut fields_b: Vec<ObjectFieldType>;

    match a {
        BuiltinTypes::Object { fields } => {
            fields_a = fields.to_vec();
        }
        _ => {
            return Err("not an object".into());
        }
    }

    match b {
        BuiltinTypes::Object { fields } => {
            fields_b = fields.to_vec();
        }
        _ => {
            return Err("not an object".into());
        }
    }

    fields_a.sort();
    fields_b.sort();


    if fields_a == fields_b {
        return Ok(true);
    } else {
        Ok(false)
    }
}

pub fn is_type_object(a: &BuiltinTypes) -> bool {
    match a {
        BuiltinTypes::Object { fields: _ } => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn get_object_fields(a: &BuiltinTypes) -> Option<Vec<ObjectFieldType>> {
    match a {
        BuiltinTypes::Object { fields } => {
            return Some(fields.to_vec());
        }
        _ => {
            return None;
        }
    }
}

pub fn stackvalue_to_json(stackval: &celsium::vm::StackValue) -> String {
    let mut result = String::new();
    result += &(match stackval {
        celsium::vm::StackValue::Bool { value } =>
            format!("{{type: bool, value: {}}}", value.to_string()),
        celsium::vm::StackValue::Int { value } =>
            format!("{{type: int, value: {}}}", value.to_string()),
        celsium::vm::StackValue::Float { value } =>
            format!("{{type: float, value: {}}}", value.to_string()),
        celsium::vm::StackValue::String { value } =>
            format!("{{type: string, value: {}}}", value.to_string()),
        celsium::vm::StackValue::Array { value } => {
            let mut arrayres = String::from("{type: array, value: [");
            let mut counter = 0;
            for element in value {
                arrayres += &format!("{}", stackvalue_to_json(element));
                counter += 1;
                if counter != value.len() {
                    arrayres += ",";
                }
            }
            arrayres += "]}";
            arrayres
        }
        celsium::vm::StackValue::Object { value: _ } => todo!(),
    });
    result
}
