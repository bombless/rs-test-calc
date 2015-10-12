
use data::*;
fn parse_number(s: &mut TwoWay<Token>) -> Option<Num> {
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

fn parse_minus(s: &mut TwoWay<Token>) -> Option<()> {
    let ptr = s.pos();
    if let Some(Token::Minus) = s.read() {
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
                return Some(Factor(Bin {
                    left: l,
                    right: Some(Box::new(r)),
                }))
            }
        } else {
            return Some(Factor(Bin {
                left: l,
                right: None,
            }))
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
                return Some(Term::Plus(Bin {
                    left: l,
                    right: Some(Box::new(r)),
                }))
            }
        }
        if parse_minus(s).is_some() {
            if let Some(r) = parse_term(s) {
                return Some(Term::Minus(Bin {
                    left: l,
                    right: Some(Box::new(r)),
                }))
            }
        }
        match parse_number(s) {
            Some(mut r) if r.neg > 0 => {
                return Some(Term::Minus(Bin {
                    left: l,
                    right: Some(Box::new(Term::Minus(Bin {
                        left: {
                            r.neg -= 1;
                            Factor(Bin { left: r, right: None })
                        },
                        right: None
                    }))),
                }))
            },
            _ => ()
        }
        return Some(Term::Plus(Bin {
            left: l,
            right: None,
        }))
    }
    s.set(ptr);
    None
}

pub fn parse(s: &mut TwoWay<Token>) -> Result<Term, ()> {
    if let Some(r) = parse_term(s) {
        if s.end() {
            return Ok(r)
        }
    }
    Err(())
}
