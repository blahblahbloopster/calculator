use std::collections::VecDeque;

pub struct Calculator {
    pub(crate) stack: VecDeque<f64>
}

pub enum Op {
    ADD, SUBTRACT, MULTIPLY, DIVIDE, EXP
}

impl Op {
    fn invoke(&self, a: f64, b: f64) -> f64 {
        return match self {
            Self::ADD => a + b,
            Self::SUBTRACT => a - b,
            Self::MULTIPLY => a * b,
            Self::DIVIDE => a / b,
            Self::EXP => a.powf(b)
        }
    }
}

impl Calculator {
    pub fn op(&mut self, op: Op) {
        let b = self.stack.pop_back().expect("Stack is empty!");  // TODO: single-arg function handling
        let a = self.stack.pop_back().expect("Stack is empty!");
        self.stack.push_back(op.invoke(a, b))
    }

    pub fn print(&self) {
        println!("==========================================");
        for item in &self.stack {
            println!("{}", item);
        }
        println!("==========================================");
    }

    pub fn new() -> Calculator {
        return Calculator { stack: VecDeque::new() };
    }

    pub fn push(&mut self, n: f64) {
        self.stack.push_back(n);
    }

    pub fn drop(&mut self) {
        self.stack.pop_back();
    }
}
