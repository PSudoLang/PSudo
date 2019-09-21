use crate::coretypes::{Block, Span, Spanned};

// Whether the field get syntax is `a?.b` not `a.b`
type IsNullSafe = bool;

pub enum Expression {
    Literal(Literal),
    Unit(Span),
    Group(Span, Box<Expression>),
    Array(Span, Vec<Expression>),
    Tuple(Span, Vec<Expression>),
    Operator(OperatorExpression),
    Member(MemberExpression),
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
            Expression::Member(expression) => expression.span(),
            Expression::ControlFlow(expression) => expression.span(),
            Expression::Array(span, ..) => span.clone(),
        }
    }
}

pub enum OperatorExpression {
    Unary(Span, UnaryOperator, Box<Expression>),
    Binary(Span, BinaryOperator, Box<Expression>, Box<Expression>),
    Call(Span, String, Vec<Expression>, IsNullSafe),
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
    Field(Span, Box<Expression>, String, IsNullSafe),
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
    If(If),
    Return(Span, Box<Expression>),
}

impl Spanned for ControlFlowExpression {
    fn span(&self) -> Span {
        match self {
            ControlFlowExpression::Loop(span, ..) => span.clone(),
            ControlFlowExpression::If(r#if) => r#if.span(),
            ControlFlowExpression::Return(span, ..) => span.clone(),
        }
    }
}

pub struct If {
    pub span: Span,
    pub condition: Box<Expression>,
    pub if_branch: Block,
    pub else_branch: Option<Box<Else>>,
}

impl Spanned for If {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

pub enum Else {
    ElseIf(Span, Box<If>),
    Else(Span, Block),
}

impl Spanned for Else {
    fn span(&self) -> Span {
        match &self {
            Else::ElseIf(span, ..) => span.clone(),
            Else::Else(span, ..) => span.clone(),
        }
    }
}

pub enum Literal {
    Boolean(Span, bool),
    String(Span, String),
    Integer(Span),
    Decimal(Span),
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
    /// &x
    Reference,
    /// +x
    Plus,
    /// -x
    Minus,
    /// !x
    LogicalNot,
    /// ~x
    BitwiseNot,
    /// ++x
    PrefixIncrement,
    // --x
    PrefixDecrement,
    /// x++
    PostfixIncrement,
    /// x--
    PostfixDecrement,
    /// x!
    NullAssertion,
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
