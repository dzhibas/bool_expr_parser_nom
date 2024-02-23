use core::fmt;

use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub enum Atom {
    String(String),
    Number(i32),
    Float(f64),
    Boolean(bool),
    Variable(String),
    Date(NaiveDate),
    DateTime(String),
}

impl PartialEq<Atom> for Atom {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Atom::String(s1), Atom::String(s2)) => s1 == s2,
            (Atom::Variable(v1), Atom::Variable(v2)) => v1 == v2,
            (Atom::String(v1), Atom::Variable(v2)) => v1 == v2,
            (Atom::Variable(v1), Atom::String(v2)) => v1 == v2,
            (Atom::Number(n1), Atom::Number(n2)) => n1 == n2,
            (Atom::Float(f1), Atom::Float(f2)) => f1 == f2,
            (Atom::Boolean(b1), Atom::Boolean(b2)) => b1 == b2,
            (Atom::Date(d1), Atom::Date(d2)) => d1 == d2,
            (Atom::DateTime(t1), Atom::DateTime(t2)) => t1 == t2,
            _ => false,
        }
    }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Atom::Number(v) => match other {
                Atom::Number(v2) => Some(v.cmp(v2)),
                Atom::Float(v2) => f64::from(*v).partial_cmp(v2),
                _ => None,
            },
            Atom::Float(v) => match other {
                Atom::Float(v2) => v.partial_cmp(v2),
                Atom::Number(v2) => v.partial_cmp(&f64::from(*v2)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::String(string) => write!(f, "{string}"),
            Atom::Number(number) => write!(f, "{number}"),
            Atom::Float(float) => write!(f, "{float}"),
            Atom::Boolean(bool) => write!(f, "{bool}"),
            Atom::Variable(var) => write!(f, "{var}"),
            Atom::Date(var) => write!(f, "{var}"),
            Atom::DateTime(var) => write!(f, "{var}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    Eq,
    More,
    Less,
    MoreEq,
    LessEq,
    NotEq,
}

impl ComparisonOp {
    pub fn from_str(expr: &str) -> Self {
        match expr {
            "==" | "=" => ComparisonOp::Eq,
            ">" => ComparisonOp::More,
            ">=" => ComparisonOp::MoreEq,
            "<" => ComparisonOp::Less,
            "<=" => ComparisonOp::LessEq,
            "!=" | "<>" => ComparisonOp::NotEq,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self) {
            ComparisonOp::Eq => write!(f, "=="),
            ComparisonOp::More => write!(f, ">"),
            ComparisonOp::Less => write!(f, "<"),
            ComparisonOp::MoreEq => write!(f, ">="),
            ComparisonOp::LessEq => write!(f, "<="),
            ComparisonOp::NotEq => write!(f, "<>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicOp {
    And,
    Or,
}

impl LogicOp {
    pub fn from_str(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "and" | "&&" => LogicOp::And,
            "or" | "||" => LogicOp::Or,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayOp {
    In,
    NotIn,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnCall {
    Upper,
    Lower,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Void,
    Variable(Atom),
    Function(FnCall, Box<AstNode>),
    Constant(Atom),
    List(Vec<Atom>),
    Compare(Box<AstNode>, ComparisonOp, Box<AstNode>),
    Array(Box<AstNode>, ArrayOp, Box<AstNode>),
    Logic(Box<AstNode>, LogicOp, Box<AstNode>),
    Scope { expr: Box<AstNode>, negate: bool },
}

impl AstNode {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AstNode::Variable(Atom::Variable(s)) => Some(s.as_str()),
            AstNode::Constant(Atom::String(s)) => Some(s.as_str()),
            AstNode::Constant(Atom::Variable(s)) => Some(s.as_str()),
            _ => None,
        }
    }
}
