use crate::coretypes::{CompileSession, RichDebug, Span, Spanned};
use crate::util::indented;

pub enum Expression {
    Literal(Literal),
    Unit(Span),
    Group(Span, Box<Expression>),
    Array(Span, Vec<Expression>),
    Tuple(Span, Vec<Expression>),
    Operator(OperatorExpression),
    Member(MemberExpression),
    FieldGet(Span, Box<Expression>, String),
    ControlFlow(ControlFlowExpression),
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Literal(literal) => literal.span(),
            Expression::Tuple(span, ..) => span.clone(),
            Expression::Unit(span) => span.clone(),
            Expression::Group(span, ..) => span.clone(),
            Expression::Operator(expression) => expression.span(),
            Expression::FieldGet(span, ..) => span.clone(),
            Expression::Member(expression) => expression.span(),
            Expression::ControlFlow(expression) => expression.span(),
            Expression::Array(span, ..) => span.clone(),
        }
    }
}

impl RichDebug for Expression {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Expression::Unit(..) => "()".into(),
            Expression::Group(_, expression) => expression.rich_debug(session),
            Expression::Tuple(_, expressions) => format!(
                "Tuple {{\n{}\n}}",
                indented(
                    expressions
                        .iter()
                        .map(|expression| expression.rich_debug(session))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            ),
            Expression::Literal(literal) => literal.rich_debug(session),
            Expression::Array(_, expressions) => format!(
                "Array {{\n{}\n}}",
                indented(
                    expressions
                        .iter()
                        .map(|expression| expression.rich_debug(session))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            ),
            Expression::Operator(operator_expression) => operator_expression.rich_debug(session),
            _ => "Unknown Expression".into(),
        }
    }
}

pub enum OperatorExpression {
    Unary(Span, UnaryOperator, Box<Expression>),
    Binary(Span, BinaryOperator, Box<Expression>, Box<Expression>),
    Call(Span, String, Vec<Expression>),
}

impl RichDebug for OperatorExpression {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            OperatorExpression::Binary(_, operator, lhs, rhs) => format!(
                "Binary Operator {} with {{\n{},\n{}\n}}",
                match operator {
                    BinaryOperator::LogicalAnd => "\"LogicalAnd\"(lhs && rhs)",
                    BinaryOperator::LogicalOr => "\"LogicalOr\"(lhs || rhs)",
                    BinaryOperator::Equals => "\"Equals\"(lhs == rhs)",
                    BinaryOperator::NotEquals => "\"NotEquals\"(lhs != rhs)",
                    BinaryOperator::LessThan => "\"LessThan\"(lhs < rhs)",
                    BinaryOperator::LessThanEquals => "\"LessThanEquals\"(lhs <= rhs)",
                    BinaryOperator::GreaterThan => "\"GreaterThan\"(lhs > rhs)",
                    BinaryOperator::GreaterThanEquals => "\"GreaterThanEquals\"(lhs >= rhs)",
                    BinaryOperator::ThreeWayComparison => "\"Three Way Comparison\"(lhs <=> rhs)",
                    BinaryOperator::BitwiseAnd => "\"BitwiseAnd\"(lhs & rhs)",
                    BinaryOperator::BitwiseXor => "\"BitwiseXor\"(lhs ^ rhs)",
                    BinaryOperator::BitwiseOr => "\"BitwiseOr\"(lhs | rhs)",
                    BinaryOperator::LeftShift => "\"LeftShift\"(lhs << rhs)",
                    BinaryOperator::RightShift => "\"RightShift\"(lhs >> rhs)",
                    BinaryOperator::Remainder => "\"Remainder\"(lhs % rhs)",
                    BinaryOperator::Times => "\"Times\"(lhs * rhs)",
                    BinaryOperator::Add => "\"Add\"(lhs + rhs)",
                    BinaryOperator::Subtract => "\"Subtract\"(lhs - rhs)",
                    BinaryOperator::Divide => "\"Divide\"(lhs / rhs)",
                    BinaryOperator::Index => "\"Index\"(lhs[rhs])",
                    BinaryOperator::RangeInclusive => "\"RangeInclusive\"(lhs..rhs)",
                    BinaryOperator::RangeExclusive => "\"RangeExclusive\"(lhs..<rhs)",
                },
                indented(lhs.rich_debug(session)),
                indented(rhs.rich_debug(session)),
            ),
            OperatorExpression::Unary(_, operator, expression) => format!(
                "Unary Operator {} with {{\n{}\n}}",
                match operator {
                    UnaryOperator::Reference => "\"Reference\"(&)",
                    UnaryOperator::Plus => "\"Plus\"(+)",
                    UnaryOperator::Minus => "\"Minus\"(-)",
                    UnaryOperator::LogicalNot => "\"LogicalNot\"(!)",
                    UnaryOperator::BitwiseNot => "\"BitwiseNot\"(~)",
                    UnaryOperator::PrefixIncrement => "\"PrefixIncrement\"(++x)",
                    UnaryOperator::PostfixIncrement => "\"PostfixIncrement\"(x++)",
                    UnaryOperator::PrefixDecrement => "\"PrefixDecrement\"(--x)",
                    UnaryOperator::PostfixDecrement => "\"PostfixDecrement\"(--x)",
                },
                indented(expression.rich_debug(session))
            ),
            _ => "Unknown Operator Expression".into(),
        }
    }
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
            Literal::Integer(span) | Literal::Decimal(span) => span.source_text(session),
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
    Reference,        // &
    Plus,             // +
    Minus,            // -
    LogicalNot,       // !
    BitwiseNot,       // ~
    PrefixIncrement,  // ++x
    PostfixIncrement, // x++
    PrefixDecrement,  // --x
    PostfixDecrement, // x--
}

pub enum BinaryOperator {
    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,     
    /// ==
    Equals,           
    /// !=
    NotEquals,         
    /// <
    LessThan,          
    /// <=
    LessThanEquals,    
    /// >
    GreaterThan,       
    /// >=
    GreaterThanEquals,
    /// <=>
    ThreeWayComparison,
    /// &
    BitwiseAnd,        
    /// ^
    BitwiseXor,        
    /// |
    BitwiseOr,         
    /// <<
    LeftShift,         
    /// >>
    RightShift,        
    /// %
    Remainder,         
    /// *
    Times,             
    /// +
    Add,               
    /// -
    Subtract,          
    /// /
    Divide,            
    /// [x]
    Index,             
    /// ..
    RangeInclusive,    
    /// ..<
    RangeExclusive,  
}
