
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

#[derive(Debug)]
pub struct Factor (pub Bin<i64, Factor>);

#[derive(Debug)]
pub enum Term {
    Plus(Bin<Factor, Term>),
    Minus(Bin<Factor, Term>),
}

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Number(i64),
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


impl Factor {
    pub fn calc(&self) -> i64 {
        match self.0.right.as_ref() {
            Some(x) => self.0.left * x.calc(),
            None => self.0.left
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
