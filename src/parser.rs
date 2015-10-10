
use data::*;
fn parse_number(s: &mut TwoWay<Token>) -> Option<u8> {
    let ptr = s.pos();
    if let Some(Token::Number(x)) = s.read() {
        Some(x)
    } else {
        s.set(ptr);
        None
    }
}

fn parse_plus(s: &mut TwoWay<Token>) -> Option<()> {
    let ptr = s.pos();
    if let Some(Token::Plus) = s.read() {
        Some(())
    } else {
        s.set(ptr);
        None
    }
}

fn parse_mul(s: &mut TwoWay<Token>) -> Option<()> {
    let ptr = s.pos();
    if let Some(Token::Mul) = s.read() {
        Some(())
    } else {
        s.set(ptr);
        None
    }
}

fn parse_factor(s: &mut TwoWay<Token>) -> Option<Factor> {
    let ptr = s.pos();
    if let Some(l) = parse_number(s) {
        if parse_mul(s).is_some() {
            if let Some(r) = parse_factor(s) {
                return Some(Factor {
                    left: l,
                    right: Some(Box::new(r)),
                })
            }
        } else {
            return Some(Factor {
                left: l,
                right: None,
            })
        }
    }
    s.set(ptr);
    None
}

fn parse_term(s: &mut TwoWay<Token>) -> Option<Term> {
    let ptr = s.pos();
    if let Some(l) = parse_factor(s) {
        if parse_plus(s).is_some() {
            if let Some(r) = parse_term(s) {
                return Some(Term {
                    left: l,
                    right: Some(Box::new(r)),
                })
            }
        } else {
            return Some(Term {
                left: l,
                right: None,
            })
        }
    }
    s.set(ptr);
    None
}

pub fn parse(s: &mut TwoWay<Token>) -> Option<Term> { parse_term(s) }
