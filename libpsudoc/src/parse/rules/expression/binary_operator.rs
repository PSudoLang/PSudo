use super::*;

use crate::coretypes::{
    BinaryOperator as BinaryOperatorKind, Expression as ExpressionNode, OperatorExpression,
    Spanned, TokenCategory,
};

macro_rules! binary_operator {
    ($name:tt, $level:expr, { $($token_category:path => $operator_kind:path),*, }) => {
        pub struct $name;

        impl ParseFunction for $name {
            type Output = Box<dyn FnOnce(ExpressionNode) -> ExpressionNode>;

            fn try_parse(
                context: &mut ParseContext,
                session: &mut CompileSession,
            ) -> ParseResult<Self::Output> {
                context.skip_whitespaces(true);
                let operator = if let Some(operator) = context.next_token_categoried(&[
                    $($token_category),*
                ]) {
                    operator.clone()
                } else {
                    return ParseResult::Fail(false);
                };
                let precedence_back = context.operator_precedence;
                context.operator_precedence = $level;
                context.skip_whitespaces(true);
                match Expression::try_parse(context, session) {
                    ParseResult::Success(rhs) => {
                        context.operator_precedence = precedence_back;
                        ParseResult::Success(Box::new(move |lhs| {
                        ExpressionNode::Operator(OperatorExpression::Binary(
                            lhs.span().joined(&rhs.span()).expect("In the same file"),
                            match operator.category {
                                $($token_category => $operator_kind,)*
                                _ => panic!("Can't reach."),
                            },
                            Box::new(lhs),
                            Box::new(rhs),
                        ))
                    }))},
                    ParseResult::Fail(_) => ParseResult::Fail(true),
                }
            }
        }
    };
}

binary_operator!(BinaryOperator1, 1, {
    TokenCategory::PunctuationAsterisk => BinaryOperatorKind::Times,
    TokenCategory::PunctuationSolidus => BinaryOperatorKind::Divide,
    TokenCategory::PunctuationPercentSign => BinaryOperatorKind::Remainder,
});

binary_operator!(BinaryOperator2, 2, {
    TokenCategory::PunctuationPlusSign => BinaryOperatorKind::Add,
    TokenCategory::PunctuationHyphenMinus => BinaryOperatorKind::Subtract,
});

binary_operator!(BinaryOperator3, 3, {
    TokenCategory::PunctuationsBitwiseLeftShift => BinaryOperatorKind::LeftShift,
    TokenCategory::PunctuationsBitwiseRightShift => BinaryOperatorKind::RightShift,
});

binary_operator!(BinaryOperator4, 4, {
    TokenCategory::PunctuationAmpersand => BinaryOperatorKind::BitwiseAnd,
});

binary_operator!(BinaryOperator5, 5, {
    TokenCategory::PunctuationCircumflexAccent => BinaryOperatorKind::BitwiseXor,
});

binary_operator!(BinaryOperator6, 6, {
    TokenCategory::PunctuationVerticalLine => BinaryOperatorKind::BitwiseOr,
});

binary_operator!(BinaryOperator7, 7, {
    TokenCategory::PunctuationsEqualTo => BinaryOperatorKind::Equals,
    TokenCategory::PunctuationsNotEqualTo => BinaryOperatorKind::NotEquals,
    TokenCategory::PunctuationLessThanSign => BinaryOperatorKind::LessThan,
    TokenCategory::PunctuationGreaterThanSign => BinaryOperatorKind::GreaterThan,
    TokenCategory::PunctuationsLessThanOrEqualTo => BinaryOperatorKind::LessThanEquals,
    TokenCategory::PunctuationsGreaterThanOrEqualTo => BinaryOperatorKind::GreaterThanEquals,
    TokenCategory::PunctuationsThreeWayComparison => BinaryOperatorKind::ThreeWayComparison,
});

binary_operator!(BinaryOperator8, 8, {
    TokenCategory::PunctuationsLogicalAnd => BinaryOperatorKind::LogicalAnd,
});

binary_operator!(BinaryOperator9, 9, {
    TokenCategory::PunctuationsLogicalOr => BinaryOperatorKind::LogicalOr,
});


binary_operator!(BinaryOperator10, 10, {
    TokenCategory::PunctuationsRangeExclusive => BinaryOperatorKind::RangeExclusive,
    TokenCategory::PunctuationsRangeInclusive => BinaryOperatorKind::RangeInclusive,
});
