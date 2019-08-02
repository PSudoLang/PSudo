use super::Statement;
use crate::coretypes::{CompileSession, RichDebug, Span, Spanned};

pub enum Expression {
    Literal(Literal),
    Operator(OperatorExpression),
    Block(Span, Vec<Statement>, Box<Expression>),
    Member(MemberExpression),
    FieldGet(Span, Box<Expression>, String),
    ControlFlow(ControlFlowExpression),
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Literal(literal) => literal.span(),
            Expression::Operator(expression) => expression.span(),
            Expression::FieldGet(span, ..) => span.clone(),
            Expression::Block(span, ..) => span.clone(),
            Expression::Member(expression) => expression.span(),
            Expression::ControlFlow(expression) => expression.span(),
        }
    }
}

impl RichDebug for Expression {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Expression::Literal(literal) => literal.rich_debug(session),
            _ => "Unknown Expression".into(),
        }
    }
}

pub enum OperatorExpression {
    Unary(Span, UnaryOperator, Box<Expression>),
    Binary(Span, BinaryOperator, Box<Expression>, Box<Expression>),
    Call(Span, String, Vec<Expression>),
}

impl Spanned for OperatorExpression {
    fn span(&self) -> Span {
        match self {
            OperatorExpression::Unary(span, ..) => span.clone(),
            OperatorExpression::Binary(span, ..) => span.clone(),
            OperatorExpression::Call(span, ..) => span.clone(),
        }
    }
}

pub enum MemberExpression {
    Field(Span),
    Method(Span),
}

impl Spanned for MemberExpression {
    fn span(&self) -> Span {
        match self {
            MemberExpression::Field(span, ..) => span.clone(),
            MemberExpression::Method(span, ..) => span.clone(),
        }
    }
}

pub enum ControlFlowExpression {
    Loop(Span),
    If(Span),
}

impl Spanned for ControlFlowExpression {
    fn span(&self) -> Span {
        match self {
            ControlFlowExpression::Loop(span, ..) => span.clone(),
            ControlFlowExpression::If(span, ..) => span.clone(),
        }
    }
}

pub enum Literal {
    Boolean(Span, bool),
    String(Span, String),
    Integer(Span),
    Decimal(Span),
}

impl RichDebug for Literal {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Literal::Boolean(.., val) => format!("{:?}", val),
            Literal::String(.., val) => format!("{:?}", val),
            _ => "Unknown Literal".into(),
        }
    }
}

impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Literal::Boolean(span, ..) => span,
            Literal::String(span, ..) => span,
            Literal::Integer(span) => span,
            Literal::Decimal(span) => span,
        }
        .clone()
    }
}

pub enum UnaryOperator {
    Plus,       // +
    Minus,      // -
    LogicalNot, // !
    BitwiseNot, // ~
}

pub enum BinaryOperator {
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
    Index,             // [x]
}
