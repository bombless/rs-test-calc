
use data::*;
pub fn read(s: &mut TwoWay<char>) -> Result<Vec<Token>, ()> {
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
        if parse_minus(s).is_some() {
            v.push(Token::Minus);
            continue
        }
        if parse_mul(s).is_some() {
            v.push(Token::Mul);
            continue
        }
        if parse_whitespace(s).is_some() {
            continue
        }
        break
    }
    if s.end() {
        Ok(v)
    } else {
        Err(())
    }
}

fn parse_number(s: &mut TwoWay<char>) -> Option<i64> {
    let ptr = s.pos();
    let mut num = None;
    while let Some(c) = s.read() {
        match c {
            '0' ... '9' if num == Some(0) => {
                s.set(ptr);
                return None
            },
            '0' ... '9' => num = Some((c as i64) - b'0' as i64 + num.unwrap_or(0) * 10),
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

fn parse_minus(s: &mut TwoWay<char>) -> Option<()> {
    let ptr = s.pos();
    let x = s.read();
    if let Some('-') = x {
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

fn parse_whitespace(s: &mut TwoWay<char>) -> Option<()> {
    let ptr = s.pos();
    match s.read() {
        Some(' ') | Some('\t') | Some('\n') | Some('\r') => Some(()),
        _ => { s.set(ptr); None }
    }
}
