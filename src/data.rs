
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

pub struct TwoWay<T> {
    stream: Vec<T>,
    ptr: usize,
}

#[derive(Debug)]
pub struct Bin<Left, Right> {
    pub left: Left,
    pub right: Option<Box<Right>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Num {
    pub neg: u64,
    pub num: i64
}

#[derive(Debug)]
pub struct Factor (pub Bin<Num, Factor>);

#[derive(Debug)]
pub enum Term {
    Plus(Bin<Factor, Term>),
    Minus(Bin<Factor, Term>),
}

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Number(Num),
    Plus,
    Minus,
    Mul,
}


impl<T: Copy> TwoWay<T> {
    pub fn new(v: Vec<T>) -> TwoWay<T> { TwoWay { stream: v, ptr: 0 } }
    pub fn set(&mut self, ptr: usize) {
        self.ptr = ptr
    }
    pub fn read(&mut self) -> Option<T> {
        if self.stream.len() > self.ptr {
            self.ptr += 1;
            Some(self.stream[self.ptr - 1])
        } else {
            None
        }
    }
    pub fn end(&self) -> bool { self.ptr == self.stream.len() }
    pub fn pos(&self) -> usize { self.ptr }
}

impl Num {
    pub fn calc(&self) -> i64 {
        if self.neg % 2 == 1 { - self.num } else { self.num }
    }
}

impl Factor {
    pub fn calc(&self) -> i64 {
        match self.0.right.as_ref() {
            Some(x) => self.0.left.calc() * x.calc(),
            None => self.0.left.calc()
        }
    }
}

impl Term {
    pub fn calc(&self) -> i64 {
        match *self {
            Term::Plus(Bin {left: ref l, right: Some(ref r) }) => l.calc() + r.calc(),
            Term::Minus(Bin {left: ref l, right: Some(ref r) }) => l.calc() - r.calc(),
            Term::Plus(Bin {left: ref l, right: None }) |
            Term::Minus(Bin {left: ref l, right: None }) => l.calc()
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", match *self {
            Token::Number(ref n) => n.to_string(),
            Token::Plus => '+'.to_string(),
            Token::Minus => '-'.to_string(),
            Token::Mul => '*'.to_string(),
        })
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        for _ in (0 .. self.neg) {
            try!(write!(f, "-"))
        }
        write!(f, "{}", self.num)
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(ref r) = self.0.right {
            write!(f, "{} * {}", self.0.left, r)
        } else {
            write!(f, "{}", self.0.left)
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match *self {
            Term::Plus(Bin { left: ref l, right: Some(ref r)}) => {
                write!(f, "{} + {}", l, r)
            }
            Term::Minus(Bin { left: ref l, right: Some(ref r)}) => {
                write!(f, "{} - {}", l, r)
            }
            Term::Plus(Bin { left: ref l, right: None }) => {
                write!(f, "{}", l)
            }
            Term::Minus(Bin { left: ref l, right: None }) => {
                write!(f, "{}", l)
            }
        }
    }
}
