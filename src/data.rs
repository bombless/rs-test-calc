
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

pub struct TwoWay<T> {
    stream: Vec<T>,
    ptr: usize,
}

#[derive(Debug)]
pub struct Factor {
    pub left: u64,
    pub right: Option<Box<Factor>>,
}

#[derive(Debug)]
pub struct Term {
    pub left: Factor,
    pub right: Option<Box<Term>>,
}

#[derive(Copy, Clone, Debug)]
pub enum Token {
    Number(u64),
    Plus,
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
    pub fn calc(&self) -> u64 {
        match self.right.as_ref() {
            Some(x) => self.left * x.calc(),
            None => self.left
        }
    }
}

impl Term {
    pub fn calc(&self) -> u64 {
        match self.right.as_ref() {
            Some(x) => self.left.calc() + x.calc(),
            None => self.left.calc()
        }
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(ref r) = self.right {
            write!(f, "{} * {}", self.left, r)
        } else {
            write!(f, "{}", self.left)
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(ref r) = self.right {
            write!(f, "{} + {}", self.left, r)
        } else {
            write!(f, "{}", self.left)
        }
    }
}
