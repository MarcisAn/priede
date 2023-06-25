use super::executor::{Executor, StackItem};

impl Executor {
    fn get_top_two_values_f64(&mut self) -> (f64, f64) {
        let b = self.stack.pop_back().unwrap();
        let a = self.stack.pop_back().unwrap();
        let a_value = match a {
            StackItem::True => 1.0,
            StackItem::False => 0.0,
            StackItem::Number { value } => value,
            StackItem::String { value } => todo!(),
        };
        let b_value = match b {
            StackItem::True => 1.0,
            StackItem::False => 0.0,
            StackItem::String { value } => todo!(),
            StackItem::Number { value } => value,
        };
        return (a_value, b_value);
    }
    pub fn add(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();
        self.stack.push_back(StackItem::Number {
            value: a_value + b_value,
        });
    }
    pub fn divide(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        self.stack.push_back(StackItem::Number {
            value: a_value / b_value,
        });
    }
    pub fn multiply(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        self.stack.push_back(StackItem::Number {
            value: a_value * b_value,
        });
    }
    pub fn subtract(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        self.stack.push_back(StackItem::Number {
            value: a_value - b_value,
        });
    }
    pub fn remainder(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        self.stack.push_back(StackItem::Number {
            value: a_value % b_value,
        });
    }
    pub fn equals(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        if a_value == b_value {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }
    pub fn greater(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        if a_value > b_value {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }
    pub fn greater_equals(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        if a_value >= b_value {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }
    pub fn less(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        if a_value < b_value {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }
    pub fn less_equal(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        if a_value <= b_value {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }
    pub fn not_equal(&mut self) {
        let (a_value, b_value) = self.get_top_two_values_f64();

        if a_value != b_value {
            self.stack.push_back(StackItem::True);
        } else {
            self.stack.push_back(StackItem::False);
        }
    }
}
