mod data;
mod lexer;
mod parser;

use data::*;
use lexer::*;
use parser::*;

fn main() {
    use std::io::{stdin, stdout, Write};
    let mut line = String::new();
    while stdout().write(b">").is_ok() && stdout().flush().is_ok() {
        if stdin().read_line(&mut line).is_err() { break }
        let s = &mut TwoWay::new(line.chars().collect());

        match read(s) {
            Ok(v) => {
                let s = &mut TwoWay::new(v);
                let x = if let Ok(x) = parse(s) {
                    x
                } else {
                    line.clear();
                    println!("Unexpected {:?}", s.read());
                    continue
                };
                println!("{} = {}", x, x.calc());
            },
            Err(_) => {
                line.clear();
                println!("Unexpected {:?}", s.read().map(char::escape_default).map(Iterator::collect::<String>))
            }
        }
        line.clear()
    }
}
