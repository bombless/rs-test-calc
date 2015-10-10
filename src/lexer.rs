
use data::*;
pub fn read(s: &mut TwoWay<char>) -> Vec<Token> {
    let mut v = Vec::new();
    while true {
        if let Some(x) = parse_number(s) {
            v.push(Token::Number(x));
            continue
        }
        if parse_plus(s).is_some() {
            v.push(Token::Plus);
            continue
        }
        if parse_mul(s).is_some() {
            v.push(Token::Mul);
            continue
        }
        break
    }
    v
}

fn parse_number(s: &mut TwoWay<char>) -> Option<u8> {
    let ptr = s.pos();
    let mut num = None;
    while let Some(c) = s.read() {
        match c {
            '0' ... '9' if num == Some(0) => {
                s.set(ptr);
                return None
            },
            '0' ... '9' => num = Some((c as u8) - b'0' + num.unwrap_or(0) * 10),
            ' ' | '\t' | '\n' | '\r' => {},
            _ => {
                let last = s.pos() - 1;
                s.set(last);
                break
            }
        }

    }
    if num.is_none() { s.set(ptr) }
    num
}

fn parse_plus(s: &mut TwoWay<char>) -> Option<()> {
    let ptr = s.pos();
    let x = s.read();
    if let Some('+') = x {
        Some(())
    } else {
        s.set(ptr);
        None
    }
}

fn parse_mul(s: &mut TwoWay<char>) -> Option<()> {
    let ptr = s.pos();
    if let Some('*') = s.read() {
        Some(())
    } else {
        s.set(ptr);
        None
    }
}
