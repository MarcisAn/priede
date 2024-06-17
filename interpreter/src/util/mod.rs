use celsium::{ compiletime_helper::{ CompileTimeHelper, CompileTimeImport }, Scope, BUILTIN_TYPES };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait, text::TextPosition };

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
pub fn get_ast_formated(node: AstNode) -> String {
    let input = String::new();
    format_ast(node, Vec::<bool>::new(), input)
}
pub fn format_ast<'a>(node: AstNode, crossings: Vec<bool>, string: String) -> String {
    let mut string_ = string.clone();
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            let old = string.to_owned();
            string_ += &(old.clone() + &format!("{:}", "  "));
            i += 1;
        }
        print!(" ");
    }

    let old = string.to_owned();
    string_ += &(old.clone() + &format!("{:}", node.to_string()));
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        format_ast(children.at(i), child_crossings, string.clone());
        i += 1;
    }
    return string_;
}
pub fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn get_closest_block(node: AstNode) -> usize {
    if node.get_symbol().to_string() == "block" {
        return node.id();
    } else {
        let parrent = node.parent().unwrap();
        //should not panic because the root is a block
        get_closest_block(parrent)
    }
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

pub fn get_closest_node_location(node: AstNode) -> TextPosition {
    if node.get_position().is_some() {
        return node.get_position().unwrap();
    }
    for child in node.children() {
        if child.get_position().is_some() {
            return child.get_position().unwrap();
        }
    }
    panic!();
}

pub fn data_type_from_str(inp: &str) -> Option<BUILTIN_TYPES> {
    println!("{}", inp);
    return Some(match inp {
        "sk" => celsium::BUILTIN_TYPES::MAGIC_INT,
        "skaitlis" => celsium::BUILTIN_TYPES::MAGIC_INT,
        "būls" => celsium::BUILTIN_TYPES::BOOL,
        "bl" => celsium::BUILTIN_TYPES::BOOL,
        "teksts" => celsium::BUILTIN_TYPES::STRING,
        "tk" => celsium::BUILTIN_TYPES::STRING,
        "decim" => celsium::BUILTIN_TYPES::FLOAT,
        _ => {
            return None;
        }
    });
}

pub fn str_from_data_type(inp: BUILTIN_TYPES) -> String {
    match inp {
        BUILTIN_TYPES::MAGIC_INT => "skaitlis".into(),
        BUILTIN_TYPES::BOOL => "būls".into(),
        BUILTIN_TYPES::STRING => "teksts".into(),
        BUILTIN_TYPES::OBJECT { fields: _, name: _ } => "objekts".into(),
        BUILTIN_TYPES::FLOAT => "decimālskaitlis".into(),
    }
}
