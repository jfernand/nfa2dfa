use colored::Colorize;
use std::fmt::{Display, Formatter};

pub enum Expr {
    Char(char),
    BinaryOp {
        left: Box<Expr>,
        op: Operation,
        right: Box<Expr>,
    },
    UnaryOp {
        operand: Box<Expr>,
        op: UnaryOp,
    },
}

pub enum UnaryOp {
    Kleene,
    Plus,
}
pub enum Operation {
    Or,
    And,
}

pub fn char(c: char) -> Expr {
    Expr::Char(c)
}
pub fn or(left: Expr, right: Expr) -> Expr {
    Expr::BinaryOp {
        left: Box::new(left),
        op: Operation::Or,
        right: Box::new(right),
    }
}

pub fn and(left: Expr, right: Expr) -> Expr {
    Expr::BinaryOp {
        left: Box::new(left),
        op: Operation::And,
        right: Box::new(right),
    }
}
pub fn star(expr: Expr) -> Expr {
    Expr::UnaryOp {
        operand: Box::new(expr),
        op: UnaryOp::Kleene,
    }
}

pub fn plus(expr: Expr) -> Expr {
    Expr::UnaryOp {
        operand: Box::new(expr),
        op: UnaryOp::Plus,
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Char(c) => write!(f, "{}", c),
            Expr::BinaryOp { left, op, right } => {
                write!(f, "{}{}{}{}{}", "(".yellow(), left, op, right, ")".yellow())
            }
            Expr::UnaryOp { operand, op } => write!(f, "({}{})", op, operand),
        }
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Kleene => write!(f, "{}", "*".yellow()),
            UnaryOp::Plus => write!(f, "{}", "+".yellow()),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Or => write!(f, "{}", "|".yellow()),
            Operation::And => write!(f, ""),
        }
    }
}

fn trim_str(input: &str) -> &str {
    if input.len() < 2 {
        "" // If the string has less than 2 characters, return an empty string
    } else {
        &input[1..input.len() - 1] // Slice the string excluding the first and last characters
    }
}

impl Expr {
    pub fn trim(&self) -> String {
        trim_str(&format!("{}", self)).to_string()
    }

    pub fn walk<F, T, R>(&self, action: &mut F, state: &T)
    where
        F: FnMut(&T) -> R,
    {
        match self {
            Expr::Char(c) => {
                println!("{}", c);
                let res = action(&state);
            }
            Expr::BinaryOp { left, op, right } => {
                println!("{} {} {}", left, op, right);
                action(&state);
                left.walk(action, state);
                right.walk(action, state);
            }
            Expr::UnaryOp { operand, op } => {
                println!("{}{}", op, operand);
                operand.walk(action, &state);
            }
        }
    }
}
