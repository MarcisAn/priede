//use colored::*;
use core::fmt;
use hime_redist::ast::AstNode;

//struct FuncDef {
//    id: String,
//    args: Vec<FuncArg>,
//}
//struct FuncArg {
//    arg_type: String,
//    arg_name: String,
//}

#[derive(Debug, Clone)]
struct FunCall {
    id: String,
    args: Vec<ValueNode>,
}

#[derive(Debug, Clone)]
struct Var {
    id: String,
    var_type: String,
    value: ValueNode,
}

#[derive(Debug, Clone)]
enum ValueNode {
    Number(i32),
    String(String),
    None(String),
    FunCall(FunCall),
    Id(String),
    VarDef(Box<Var>),
}
#[derive(Debug)]
enum AcrionNode {}
trait Eval {
    fn eval(&self) -> ValueNode;
}
trait Pop {
    fn pop_number(&self) -> i32;
    fn pop_str(&self) -> String;
}
fn func_return(input: &FunCall) -> ValueNode {
    if input.id == "aa" {
        return ValueNode::Number(5);
    } else if input.id == "drukāt" {
        print!("{}", input.args[0].eval());
        return ValueNode::None("".to_string());
    } else if input.id == "drukātJr" {
        println!("");
        print!("{}", input.args[0].eval());
        return ValueNode::None("".to_string());
    } else {
        return ValueNode::None("".to_string());
    }
}
fn id_return(input: String) -> ValueNode {
    let mut value: ValueNode = ValueNode::Number(9);
    unsafe {
        for i in &VARIABLES {
            if i.id == input {
                match i.var_type.as_str() {
                    "skaitlis" => value = i.value.clone(),
                    "teksts" => value = i.value.clone(),
                    &_ => todo!(),
                }
            }
        }
    }

    return value;
}
static mut VARIABLES: Vec<Var> = Vec::new();
fn define_variable(input: &Var) -> ValueNode {
    unsafe {
        VARIABLES.push(Var {
            id: input.id.clone(),
            var_type: input.var_type.clone(),
            value: input.value.clone(),
        });
    }

    return ValueNode::None("".to_string());
}
impl Eval for ValueNode {
    fn eval(&self) -> ValueNode {
        //print!("{:?}", &self);
        match &self {
            ValueNode::Number(value) => return ValueNode::Number(*value),
            ValueNode::None(_) => ValueNode::None("".to_string()),
            ValueNode::FunCall(value) => func_return(value),
            ValueNode::String(value) => return ValueNode::String(value.to_string()),
            ValueNode::Id(value) => id_return(value.to_string()),
            ValueNode::VarDef(value) => define_variable(value),
        }
    }
}

impl fmt::Display for ValueNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ValueNode::Number(value) => write!(f, "{:}", value),
            ValueNode::None(_) => write!(f, "NULL"),
            ValueNode::FunCall(value) => write!(f, "{:?}", func_return(value)),
            ValueNode::String(value) => write!(f, "{}", value),
            ValueNode::Id(_) => todo!(),
            ValueNode::VarDef(_) => todo!(),
        }
    }
}
impl Pop for ValueNode {
    fn pop_number(&self) -> i32 {
        match &self {
            ValueNode::Number(value) => return *value,
            ValueNode::String(_) => todo!(),
            ValueNode::None(_) => todo!(),
            ValueNode::FunCall(_) => todo!(),
            ValueNode::Id(_) => todo!(),
            ValueNode::VarDef(_) => todo!(),
        }
    }
    fn pop_str(&self) -> String {
        match &self {
            ValueNode::Number(value) => todo!(),
            ValueNode::String(_) => todo!(),
            ValueNode::None(_) => todo!(),
            ValueNode::FunCall(_) => todo!(),
            ValueNode::Id(_) => todo!(),
            ValueNode::VarDef(_) => todo!(),
        }
    }
}

fn parse_function(input: AstNode<'_>) -> crate::ast_parser::ValueNode {
    if input.to_string() == "func" {
        let node = input.clone();
        let binding = node.children();
        let binding = binding.at(1);
        let args = binding.children(); //argumenti
        let id = node
            .children()
            .at(0)
            .to_string()
            .split("ID = ")
            .collect::<Vec<&str>>()[1]
            .to_string();
        let mut arguments: Vec<ValueNode> = Vec::new();
        let mut j = 0;
        while j < node.children().at(1).children().len() {
            if args.at(j).to_string().contains("NUMBER = ") {
                arguments.push(ValueNode::Number(
                    args //argumenti
                        .at(j)
                        .to_string()
                        .split("NUMBER = ")
                        .collect::<Vec<&str>>()[1]
                        .parse::<i32>()
                        .unwrap(),
                ))
            } else if args.at(j).to_string().contains("STRING = ") {
                let binding = args.at(j);
                let binding = binding.to_string();
                let string = binding.split("STRING = ").collect::<Vec<&str>>()[1];
                if string.starts_with("\"") && string.ends_with("\"") {
                    arguments.push(ValueNode::String(
                        string.split("\"").collect::<Vec<&str>>()[1]
                            .parse::<String>()
                            .unwrap(),
                    ))
                } else if string.starts_with("\'") && string.ends_with("\'") {
                    arguments.push(ValueNode::String(
                        string.split("\'").collect::<Vec<&str>>()[1]
                            .parse::<String>()
                            .unwrap(),
                    ))
                }
            } else if args //argumenti
                .at(j)
                .to_string()
                .contains("func")
            {
                //print!("{:?}", node.to_string());
                arguments.push(parse_function(
                    args //argumenti
                        .at(j)
                        .to_owned(),
                ));
            } else if args.at(j).to_string() == "exp_plusmin"
                || args.at(j).to_string() == "exp_reizdal"
            {
                arguments.push(parse_function(args.at(j).to_owned()))
            } else if args.at(j).to_string().starts_with("ID = ") {
                arguments.push(parse_function(args.at(j).to_owned()))
            }
            j += 1;
        }
        return ValueNode::FunCall(FunCall {
            id: id.to_string(),
            args: arguments,
        })
        .eval();
    } else if input.to_string() == "exp_plusmin" || input.to_string() == "exp_reizdal" {
        // aritmētiskās darbības
        let left_value = parse_function(input.children().at(0)).pop_number();
        let right_value = parse_function(input.children().at(2)).pop_number();
        if input.children().at(1).to_string() == "+ = +" {
            return ValueNode::Number(left_value + right_value).eval();
        } else if input.children().at(1).to_string() == "- = -" {
            return ValueNode::Number(left_value - right_value).eval();
        } else if input.children().at(1).to_string() == "* = *" {
            return ValueNode::Number(left_value * right_value).eval();
        } else if input.children().at(1).to_string() == "/ = /" {
            return ValueNode::Number(left_value / right_value).eval();
        } else {
            return ValueNode::None("".to_string()).eval();
        }
    } else if input.to_string().starts_with("NUMBER = ") {
        return ValueNode::Number(
            input.to_string().split("NUMBER = ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap(),
        );
    } else if input.to_string().starts_with("ID = ") {
        return ValueNode::Id(
            input.to_string().split("ID = ").collect::<Vec<&str>>()[1].to_string(),
        )
        .eval();
    } else if input.to_string().starts_with("STRING = ") {
        let string = input.to_string().split("STRING = ").collect::<Vec<&str>>()[1].to_string();
        if string.starts_with("\"") && string.ends_with("\"") {
            return ValueNode::String(
                string.split("\"").collect::<Vec<&str>>()[1]
                    .parse::<String>()
                    .unwrap(),
            )
            .eval();
        } else if string.starts_with("\'") && string.ends_with("\'") {
            return ValueNode::String(
                string.split("\'").collect::<Vec<&str>>()[1]
                    .parse::<String>()
                    .unwrap(),
            )
            .eval();
        } else {
            return ValueNode::None("".to_string());
        }
    } else if input.to_string() == "var_def" {
        let id = input
            .children()
            .at(1)
            .to_string()
            .split("ID = ")
            .collect::<Vec<&str>>()[1]
            .to_string();
        let var_type = input
            .children()
            .at(0)
            .to_string()
            .split("TYPE = ")
            .collect::<Vec<&str>>()[1]
            .to_string();

        return ValueNode::VarDef(Box::new(Var {
            id: id,
            var_type: var_type,
            value: parse_function(input.children().at(2)),
        }))
        .eval();
    } else {
        return ValueNode::None("".to_string()).eval();
    }
}
pub fn parse_ast(input: AstNode<'_>) {
    let mut i = 0;
    let children = input.children();
    while i < input.children().len() {
        parse_function(children.at(i));
        i += 1;
    }
}
