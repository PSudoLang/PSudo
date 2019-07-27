use super::{Statement, Type};
use crate::coretypes::{Span, Spanned};

pub enum Expression {
    Literal(Literal),
    UnaryOperator(Span, UnaryOperatorType, Box<Expression>),
    BinaryOperator(Box<Expression>, BinaryOperatorType, Box<Expression>),
    Block(Vec<Statement>, Box<Expression>),
    FieldGet(Span, Box<Expression>, String),
    FunctionCall(FunctionCall),
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Literal(literal) => literal.span(),
            Expression::UnaryOperator(span, ..) => span.clone(),
            Expression::BinaryOperator(lhs, _, rhs) => lhs
                .span()
                .joined(rhs.span())
                .expect("AST node must have span"),
            Expression::FieldGet(span, ..) => span.clone(),
            Expression::Block(statements, expression) => statements
                .span()
                .joined(expression.span())
                .expect("AST node must have span"),
            Expression::FunctionCall(function_call) => function_call.span.clone(),
        }
    }
}

pub struct FunctionCall {
    span: Span,
    parent_type: Type,
    name: String,
    parameters: Vec<Expression>,
}

pub enum Literal {
    Boolean(Span, bool),
    String(Span, String),
    Integer(Span, String),
    Decimal(Span, String),
}

impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Literal::Boolean(span, ..) => span,
            Literal::String(span, ..) => span,
            Literal::Integer(span, ..) => span,
            Literal::Decimal(span, ..) => span,
        }
        .clone()
    }
}

pub enum UnaryOperatorType {
    Plus,       // +
    Minus,      // -
    LogicalNot, // !
    BitwiseNot, // ~
}

pub enum BinaryOperatorType {
    LogicalAnd,        // &&
    LogicalOr,         // ||
    Equals,            // ==
    NotEquals,         // !=
    LessThan,          // <
    LessThanEquals,    // <=
    GreaterThan,       // >
    GreaterThanEquals, // >=
    BitwiseAnd,        // &
    BitwiseXor,        // ^
    BitwiseOr,         // |
    LeftShift,         // <<
    RightShift,        // >>
    Remainder,         // %
    Times,             // *
    Add,               // +
    Subtract,          // -
    Divide,            // /
}
