use std::fs::{self, File};

use hime_redist::{
    ast::{Ast, AstNode},
    symbols::SemanticElementTrait,
};

mod hime;

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
fn print_ast(node: AstNode) {
    print(node, Vec::<bool>::new());
}
#[derive(Clone, Debug)]
pub struct StumbrsData {
    pub units: Vec<StumbrsUnit>,
}
#[derive(Clone, Debug)]
pub struct StumbrsUnit {
    pub data_type: String,
    pub value: String,
}

pub fn load_stumbrs_data(path: String) -> StumbrsData {
    print!("{}", path);
    let data_file_content = fs::read_to_string(path).unwrap();
    let parse_res = hime::stumbrs::parse_string(data_file_content);
    println!("{:?}", parse_res.errors.errors);
    let ast = parse_res.get_ast();
    let root = ast.get_root();
    print_ast(root);

    let number_of_units = root.child(0).children_count();
    let mut counter = 0;
    let mut units: Vec<StumbrsUnit> = vec![];
    while counter < number_of_units {
        units.push(StumbrsUnit {
            data_type: root
                .child(0)
                .child(counter)
                .child(0)
                .get_symbol()
                .to_string(),
            value: root.child(1).child(counter).child(0).get_value().unwrap().to_string(),
        });
        counter += 1;
    }

    println!("{:?}", units);

    return StumbrsData {
        units: units
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let parse_res =
            hime::stumbrs::parse_string("{ sk aaa tk rr } {2 \"aa\" [2;4;5]}".to_string());
        println!("{:?}", parse_res.errors.errors);
        let ast = parse_res.get_ast();
        let root = ast.get_root();
        print_ast(root);
    }
}
