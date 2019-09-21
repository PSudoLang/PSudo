use crate::coretypes::{CompileSession, RichDebug, OperatorExpression, BinaryOperator, UnaryOperator};
use crate::util::indented;

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
                    UnaryOperator::NullAssertion => "\"NullAssertion\"(x!)",
                },
                indented(expression.rich_debug(session))
            ),
            _ => "Unknown Operator Expression".into(),
        }
    }
}