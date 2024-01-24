use std::{fs, process};

use hime_redist::ast::AstNode;

fn main() {}

mod ast;
mod gen;
mod hime;

pub fn compile(path: String) {
    let file_read = fs::read_to_string(&path);
    if file_read.is_err() {
        println!(
            "Neizdevās nolasīt failu {} \nPārlicinies, ka faila adrese ir pareiza!",
            path
        );
        process::exit(1);
    }
    let contents = file_read.unwrap();
    let result = hime::priede::parse_string(contents);
    let ast = result.get_ast();
    let root = ast.get_root();
    print(root, Vec::<bool>::new());
}

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
