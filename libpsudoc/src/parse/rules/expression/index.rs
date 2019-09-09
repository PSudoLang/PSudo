use super::*;

use crate::coretypes::{
    BinaryOperator as BinaryOperatorKind, Expression as ExpressionNode, OperatorExpression,
    Spanned, TokenCategory,
};

pub struct Index;

impl ParseFunction for Index {
    type Output = Box<dyn FnOnce(ExpressionNode) -> ExpressionNode>;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        context.skip_whitespaces(false);
        if context
            .next_token_categoried(&[TokenCategory::PunctuationLeftSquareBracket])
            .is_none()
        {
            return ParseResult::Fail(false);
        };
        let precedence_back = context.operator_precedence;
        context.operator_precedence = usize::max_value();
        context.skip_whitespaces(true);
        match Expression::try_parse(context, session) {
            ParseResult::Success(rhs) => {
                context.operator_precedence = precedence_back;
                let right_square_bracket = if let Some(right_square_bracket) =
                    context.next_token_categoried(&[TokenCategory::PunctuationRightSquareBracket])
                {
                    right_square_bracket.clone()
                } else {
                    return ParseResult::Fail(true);
                };
                ParseResult::Success(Box::new(move |lhs| {
                    ExpressionNode::Operator(OperatorExpression::Binary(
                        lhs.span()
                            .joined(&right_square_bracket.span)
                            .expect("In the same file"),
                        BinaryOperatorKind::Index,
                        Box::new(lhs),
                        Box::new(rhs),
                    ))
                }))
            }
            ParseResult::Fail(_) => {
                context.operator_precedence = precedence_back;
                ParseResult::Fail(true)
            }
        }
    }
}
