use color_eyre::Result;
use eyre::{eyre, Ok};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(isize),
    Symbol(String),
    String(String),
    List(Vec<Expression>),
    Function {
        params: Vec<Expression>,
        body: Box<Expression>,
    },
    Nil,
}

impl Expression {
    pub fn as_number(&self) -> Result<isize> {
        if let Expression::Number(x) = self {
            Ok(*x)
        } else {
            Err(eyre!("'{}' is not a number", self))
        }
    }
    pub fn as_symbol(&self) -> Result<String> {
        if let Expression::Symbol(x) = self {
            Ok(x.clone())
        } else {
            Err(eyre!("'{}' is not a symbol", self))
        }
    }
    pub fn as_boolean(&self) -> Result<bool> {
        Ok(!matches!(self, Expression::Nil))
    }
    pub fn as_string(&self) -> Result<String> {
        if let Expression::String(x) = self {
            Ok(x.clone())
        } else {
            Err(eyre!("'{}' is not a string", self))
        }
    }
    pub fn as_list(&self) -> Result<Vec<Expression>> {
        if let Expression::List(x) = self {
            Ok(x.clone())
        } else {
            Err(eyre!("'{}' is not a list", self))
        }
    }
}

impl From<bool> for Expression {
    fn from(b: bool) -> Self {
        if b {
            Expression::Symbol("t".to_string())
        } else {
            Expression::Nil
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Number(n) => format!("{n}"),
                Expression::String(s) => format!("\"{s}\""),
                Expression::Symbol(s) => s.to_string(),
                Expression::List(list) => format!(
                    "({})",
                    list.iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ),
                Expression::Nil => "nil".to_string(),
                bombo => format!("{:?}", bombo), // Expression::Function { params, body } => format!("{:?}", ),
            }
        )
    }
}
