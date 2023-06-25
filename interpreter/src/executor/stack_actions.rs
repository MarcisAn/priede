use std::collections::LinkedList;

use colored::Colorize;

use crate::generator::bytecode::{self, BytecodeItem, BytecodeItemType, VarTypes};

use super::executor::{Executor, StackItem, Variable};
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}
impl Executor {
    pub fn call_func(&mut self, name: String) {
        if name == "drukāt" {
            let output: String = match self.stack.pop_back().unwrap() {
                StackItem::True => "1".to_string(),
                StackItem::False => "0".to_string(),
                StackItem::String { value } => value,
                StackItem::Number { value } => value.to_string(),
            };
            #[cfg(target_family = "wasm")]
            alert(&output);
            println!("{}", output.bright_red());
        } else {
            let mut iter_index = 0;
            loop {
                if self.functions[iter_index].name == name {
                    break;
                }
                if iter_index == self.functions.len() {
                    break;
                }
            }
            let i = &self.functions.clone()[iter_index];
            if i.name == name {
                let function_bytecode: Vec<BytecodeItem> =
                    self.bytecode[i.start_bytecode..i.start_bytecode + i.len].to_vec();
                //for j in function_bytecode.clone() {
                //    println!("{:?}", j.typ);
                //}
                self.execute_bytecode(function_bytecode);
            }
        }
    }
    pub fn push_number(&mut self, value_as_string: String) {
        self.stack.push_back(StackItem::Number {
            value: value_as_string.replace(",", ".").parse::<f64>().unwrap(),
        });
    }
    pub fn push_string(&mut self, value: String) {
        self.stack.push_back(StackItem::String { value })
    }
    pub fn push_bool(&mut self, value: bool) {
        self.stack.push_back(if value {
            StackItem::True
        } else {
            StackItem::False
        })
    }

    pub fn ifexp(&mut self) {
        let item = self.stack.pop_back().unwrap();
        let result = match item {
            StackItem::True => StackItem::True,
            StackItem::False => StackItem::False,
            StackItem::String { value } => {
                if value != "" {
                    StackItem::True
                } else {
                    StackItem::False
                }
            }
            StackItem::Number { value } => {
                if value != 0.0 {
                    StackItem::True
                } else {
                    StackItem::False
                }
            }
        };
        self.stack.push_back(result);
    }
    pub fn and(&mut self) {
        let b = self.stack.pop_back().unwrap();
        let a = self.stack.pop_back().unwrap();

        if a == StackItem::True && b == StackItem::True {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }
    pub fn or(&mut self) {
        let b = self.stack.pop_back().unwrap();
        let a = self.stack.pop_back().unwrap();

        if a == StackItem::True || b == StackItem::True {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }
    pub fn xor(&mut self) {
        let b = self.stack.pop_back().unwrap();
        let a = self.stack.pop_back().unwrap();

        if a == StackItem::True && b == StackItem::False
            || a == StackItem::False && b == StackItem::True
        {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }

    pub fn var_def(&mut self, name: String, typ: &VarTypes) {
        //self.stack.push_back();
        let typ = typ.clone();
        self.variables.push(Variable {
            name,
            typ,
            value: self.stack.pop_back().unwrap(),
        });
    }
    pub fn load_var(&mut self, name: String) {
        let mut i = 0;
        let lenght = self.variables.len();
        let ref variables = self.variables;
        while i < lenght {
            let ref var = self.variables[i];
            if var.name == name {
                break;
            }
            i += 1;
        }

        if i >= lenght {
            panic!("tāds mainīgais neeksistē")
        } else {
            let val = match &self.variables[i].value {
                StackItem::True => StackItem::True,
                StackItem::False => StackItem::False,
                StackItem::String { value } => StackItem::String {
                    value: value.to_string(),
                },
                StackItem::Number { value } => StackItem::Number {
                    value: value.to_owned(),
                },
            };
            self.stack.push_back(val);
        }
    }
    pub fn mutate_var(&mut self, name: String) {
        let mut i = 0;
        let lenght = self.variables.len();
        while i < lenght {
            let ref var = self.variables[i];
            if var.name == name {
                break;
            }
            i += 1;
        }

        if i >= lenght {
            panic!("tāds mainīgais neeksistē")
        } else {
            self.variables[i].value = self.stack.pop_back().unwrap();
        }
    }
}
