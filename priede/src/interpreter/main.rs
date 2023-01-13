mod ast_parser;
mod priede;
extern crate hime_redist;
use hime_redist::ast::AstNode;
//use hime_redist::symbols::SemanticElementTrait;
static mut AST_STR: String = String::new();
use std::fs;

pub fn main(print_ast: bool, src_file: String) {
    let contents = fs::read_to_string(src_file).unwrap();
    //print!("{:?}", contents);
    let result = priede::parse_string(&contents);
    let ast = result.get_ast();
    if print_ast {
        format_ast(ast.get_root(), Vec::<bool>::new());
    }
    ast_parser::parse_ast(ast.get_root());
}
//fn write_to_file(input: String) -> io::Result<()> {
//    unsafe {
//        let data = input.as_bytes();
//
//        let mut pos = 0;
//        let mut buffer = File::open("foo.txt")?;
//
//        while pos < data.len() {
//            let bytes_written = buffer.write(&data[pos..])?;
//            pos += bytes_written;
//        }
//        Ok(())
//    }
//}

fn concat(input: String) {
    let text = input;
    unsafe {
        AST_STR = AST_STR.clone() + &text;
    }
}

fn format_ast<'a>(node: AstNode<'a>, crossings: Vec<bool>) {
    let mut i = 0;
    if !crossings.is_empty() {
        while i < crossings.len() - 1 {
            print!("{:}", " ");
            concat("\x00".to_string());
            i += 1;
        }
        print!(" ");
        concat("\x00".to_string());
    }
    println!("{:}", node);
    concat(node.to_string() + "\n");
    i = 0;
    let children = node.children();
    while i < children.len() {
        let mut child_crossings = crossings.clone();
        child_crossings.push(i < children.len() - 1);
        format_ast(children.at(i), child_crossings);
        i += 1;
    }
}
