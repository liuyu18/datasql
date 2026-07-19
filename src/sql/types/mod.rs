use super::parser::ast::{Consts, Expression};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl Value {
    pub fn from_expression(expr: Expression) -> Self {
        match expr {
            Expression::Consts(Consts::Null) => Value::Null,
            Expression::Consts(Consts::Boolean(b)) => Value::Boolean(b),
            Expression::Consts(Consts::Integer(i)) => Value::Integer(i),
            Expression::Consts(Consts::Float(f)) => Value::Float(f),
            Expression::Consts(Consts::String(s)) => Value::String(s),
        }
    }
}
