use celsium::{ compiletime_helper::CompileTimeHelper, BUILTIN_TYPES };
use hime_redist::{ ast::AstNode, symbols::SemanticElementTrait };

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
    starting_scope: usize,
    compilehelper: &mut CompileTimeHelper,
    node: AstNode
) -> Option<usize> {
    let mut counter = 0;
    for var in compilehelper.defined_variables.clone() {
        if var.name == target_name && var.scope == starting_scope {
            return Some(counter);
        }
        counter += 1;
    }
    for var in compilehelper.defined_arrays.clone() {
        if var.name == target_name && var.scope == starting_scope {
            return Some(counter);
        }
        counter += 1;
    }
    let node_parrent = node.parent();
    if node_parrent.is_none() {
        return None;
    }
    get_closest_scope(target_name, node_parrent.unwrap().id(), compilehelper, node_parrent.unwrap())
}

pub fn data_type_from_str(inp: &str) -> BUILTIN_TYPES {
    return match inp {
        "NUM" => celsium::BUILTIN_TYPES::MAGIC_INT,
        "BOOL_DEF" => celsium::BUILTIN_TYPES::BOOL,
        "TEXT" => celsium::BUILTIN_TYPES::STRING,
        "FLOAT" => celsium::BUILTIN_TYPES::FLOAT,
        _ => panic!(),
    };
}

pub fn str_from_data_type(inp: BUILTIN_TYPES) -> String {
    match inp {
        BUILTIN_TYPES::MAGIC_INT => "skaitlis".into(),
        BUILTIN_TYPES::BOOL => "būls".into(),
        BUILTIN_TYPES::STRING => "teksts".into(),
        BUILTIN_TYPES::OBJECT => "objekts".into(),
        BUILTIN_TYPES::FLOAT => "decimālskaitlis".into(),
    }
}
