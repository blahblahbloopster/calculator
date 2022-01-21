use crate::calculator::{Calculator, Op};
use std::io;
use std::io::Write;
use std::process::exit;
use rug::{Assign, Float};

mod calculator;

fn process(calc: &mut Calculator, inp: &str) {
    if inp == "q" || inp == "quit" {
        exit(0);
    }

    if inp == "d" || inp == "drop" {
        calc.drop();
        return;
    }

    if inp == "p" || inp == "dup" {
        calc.dup();
        return;
    }

    if inp == "s" || inp == "swap" {
        calc.swap();
        return;
    }

    let op = match inp {
        "+" => Option::Some(Op::ADD),
        "-" => Option::Some(Op::SUBTRACT),
        "*" => Option::Some(Op::MULTIPLY),
        "/" => Option::Some(Op::DIVIDE),
        "^" => Option::Some(Op::EXP),
        _ => Option::None
    };

    if op.is_some() {
        if op.is_none() {
            println!("Failed to parse operation");
            return;
        }
        if calc.stack.len() < 2 {
            println!("Stack is empty");
            return;
        }
        calc.op(op.expect("wtf"));
    } else {
        let parsed = Float::parse(inp);
        // let num = inp.parse::<f64>();
        if parsed.is_err() {
            return;
        }
        let res = parsed.expect("wtf");
        let mut num = Float::new(1024);  // TODO
        num.assign(res);
        calc.push(num.as_complex().clone());
    }
}

fn main() {
    let mut calc = Calculator::new();
    let stdin = io::stdin();
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        calc.print();
        print!("> ");
        io::stdout().flush().expect("failed to flush?");
        let mut inp = String::new();
        stdin.read_line(&mut inp).expect("what");
        inp = inp.replace("\n", "");
        let mut last = String::new();
        let mut last_was_num = false;
        let mut last_was_space = true;
        for c in inp.chars() {
            if "0123456789.".contains(c) || (c == '-' && last_was_space) {
                if !last_was_num {
                    if !last.is_empty() {
                        process(&mut calc, last.as_str());
                    }
                    last.clear();
                }
                last_was_num = true;
            } else {
                if !last.is_empty() {
                    process(&mut calc, last.as_str());
                }
                last_was_num = false;
                last.clear();
            }
            if !c.is_whitespace() {
                last.push(c);
            }
            last_was_space = c.is_whitespace();
        }
        if !last.is_empty() {
            process(&mut calc, last.as_str());
        }
    }
}
