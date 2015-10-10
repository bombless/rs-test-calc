mod data;
mod lexer;
mod parser;

use data::*;
use lexer::*;
use parser::*;

fn main() {
    use std::io::stdin;
    let mut line = String::new();
    while stdin().read_line(&mut line).is_ok() {
        let s = read(&mut TwoWay::new(line.chars().collect()));
        let x = parse(&mut TwoWay::new(s)).unwrap();
        println!("{} = {}", x, x.calc());
        line.clear()
    }
}
