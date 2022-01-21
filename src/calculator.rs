use std::collections::VecDeque;
use rug::{Complex, Float};
use rug::ops::Pow;
use regex::Regex;
use rug::float::Round;

pub struct Calculator {
    pub(crate) stack: VecDeque<Complex>
}

pub enum Op {
    ADD, SUBTRACT, MULTIPLY, DIVIDE, EXP
}

impl Op {
    fn invoke(&self, a: Complex, b: Complex) -> Complex {
        return match self {
            Self::ADD => a + b,
            Self::SUBTRACT => a - b,
            Self::MULTIPLY => a * b,
            Self::DIVIDE => a / b,
            Self::EXP => a.pow(b.real())
        }
    }
}

impl Calculator {
    pub fn op(&mut self, op: Op) {
        let b = self.stack.pop_back().expect("Stack is empty!");  // TODO: single-arg function handling
        let a = self.stack.pop_back().expect("Stack is empty!");
        self.stack.push_back(op.invoke(a, b))
    }

    pub fn format_float(inp: &Float) -> String {
        let mut sig = inp.to_string_radix_round(10, Option::Some((inp.prec() as usize / 5) - 2), Round::Nearest);
        if !sig.contains('.') {
            sig.push_str(".0");
        }
        if !sig.contains('e') {
            sig.push_str("e0");
        }

        let is_negative = inp.is_sign_negative();
        if is_negative {
            sig = sig.strip_prefix("-").expect("what").to_string();
        }
        let prefix = if is_negative { "-" } else { "" };

        let split = sig.split_once('.').expect("wiofjioew");
        let int = split.0;
        let decimal_and_exp = split.1;
        let split2 = decimal_and_exp.split_once('e').expect("wiojfioejwoifew");
        let decimal = split2.0;
        let exponent = split2.1;
        let exp_int: isize = exponent.parse().expect("wtf");

        return if exp_int == 0 {
            format!("{}{}.{}", prefix, int, decimal)
        } else if exp_int > 0 {
            return if exp_int < decimal.len() as isize {
                let parts = decimal.split_at(exp_int as usize);
                format!("{}{}{}.{}", prefix, int, parts.0, parts.1)
            } else {
                format!("{}{}{}", prefix, int, decimal.to_owned() + &"0".repeat((exp_int - decimal.len() as isize) as usize))
            };
        } else {
            return if exp_int.abs() < int.len() as isize {
                let parts = decimal.split_at(exp_int as usize);
                format!("{}{}{}.{}", prefix, int, parts.0, parts.1)
            } else {
                format!("{}0.{}{}{}", prefix, "0".repeat(exp_int.abs() as usize - int.len()), int, decimal)
            };
        };
    }

    pub fn print(&self) {
        let trailing_zero_regex = Regex::new("\\.?0*$").expect("failed to make regex?");
        for item in &self.stack {
            let real_str = Calculator::format_float(item.real());
            let imag_str = Calculator::format_float(item.imag());
            let real_str2 = real_str.as_str();
            let imag_str2 = imag_str.as_str();
            let real = trailing_zero_regex.replace(real_str2, "").to_string();
            let imag = trailing_zero_regex.replace(imag_str2, "").to_string();
            let formatted = if item.imag().is_zero() { real } else { (if item.real().is_zero() { "".to_string() } else { real + " + " }) + (if item.imag().to_f64().abs() == 1.0 { if item.imag().is_sign_negative() { "-" } else { "" } } else { imag.as_str() }) + "i" };
            println!("{}", formatted);
        }
    }

    pub fn new() -> Calculator {
        return Calculator { stack: VecDeque::new() };
    }

    pub fn push(&mut self, n: Complex) {
        self.stack.push_back(n);
    }

    pub fn drop(&mut self) {
        self.stack.pop_back();
    }

    pub fn dup(&mut self) {
        let res = self.stack.back();
        if res.is_none() {
            return;
        }
        let clone = res.expect("wat").clone();
        self.stack.push_back(clone);
    }

    pub fn swap(&mut self) {
        let first = self.stack.pop_back();
        let second = self.stack.pop_back();
        if first.is_none() || second.is_none() {
            return;
        }
        let a = first.expect("a").clone();
        let b = second.expect("a").clone();
        self.stack.push_back(a);
        self.stack.push_back(b);
    }
}
