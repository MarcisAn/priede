use crate::generator::bytecode::BytecodeItem;
use crate::generator::bytecode::BytecodeItemType;
use crate::generator::bytecode::BytecodeItemType::*;
use crate::generator::bytecode::VarTypes;

use std::collections::LinkedList;

#[derive(Debug, PartialEq)]
pub enum StackItem {
    True,
    False,
    String { value: String },
    Number { value: f64 },
}
#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub typ: VarTypes,
    pub value: StackItem,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub start_bytecode: usize,
    pub len: usize,
}

pub(crate) struct Executor {
    pub stack: LinkedList<StackItem>,
    pub variables: Vec<Variable>,
    pub functions: Vec<Function>,
    pub bytecode: Vec<BytecodeItem>,
}
impl Executor {
    pub fn new() -> Executor {
        Executor {
            stack: LinkedList::new(),
            variables: vec![],
            functions: vec![],
            bytecode: vec![],
        }
    }

    pub fn execute_bytecode(&mut self, bytecode_i: Vec<BytecodeItem>) {
        if self.bytecode.len() == 0 {
            self.bytecode = bytecode_i.clone();
        }
        let mut iter_index = 0;
        let bytecode = bytecode_i;
        loop {
            let item = &bytecode[iter_index];
            match &item.typ {
                Add => Self::add(self),
                Divide => Self::divide(self),
                Subtract => Self::subtract(self),
                Multiply => Self::multiply(self),
                Remainder => Self::remainder(self),
                PushNumber { value_as_string } => {
                    Self::push_number(self, value_as_string.to_string())
                }
                PushString { value } => Self::push_string(self, value.to_string()),
                FunctionCall { name } => self.call_func(name.to_string()),
                IfExp => Self::ifexp(self),
                Equals => self.equals(),
                GreaterThan => self.greater(),
                GreaterEqual => self.greater_equals(),
                LessThan => self.less(),
                LessEqual => self.less_equal(),
                NotEqual => self.not_equal(),
                And => self.and(),
                Or => self.or(),
                ExclusiveOr => self.xor(),
                VarDef { name, typ } => self.var_def(name.to_string(), typ),
                LoadVar { name } => self.load_var(name.to_string()),
                JumpToIfFalse { jump_to } => {
                    let stacktitem = self.stack.pop_back().unwrap();
                    if stacktitem != StackItem::True {
                        iter_index = jump_to.clone()
                    }
                }
                JumpTo { jump_to } => iter_index = jump_to.clone(),
                MutateVar { name } => self.mutate_var(name.clone()),
                InvertStackItem => {
                    if self.stack.pop_back().unwrap() == StackItem::False {
                        self.stack.push_back(StackItem::True)
                    } else {
                        self.stack.push_back(StackItem::False)
                    }
                }
                FuncDef { name, jump_to, len } => {
                    self.functions.push(Function {
                        name: name.to_string(),
                        start_bytecode: iter_index + 1,
                        len: *len,
                    });
                    iter_index = jump_to.clone()
                }
                _ => print!(""),
            }
            iter_index += 1;
            if iter_index == bytecode.len() {
                break;
            }
        }
    }
}
