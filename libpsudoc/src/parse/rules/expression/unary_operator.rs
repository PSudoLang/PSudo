use super::*;
use crate::coretypes::{
    Expression as ExpressionNode, OperatorExpression, Spanned, TokenCategory, UnaryOperator,
};

pub struct PrefixUnaryOperator;

impl ParseFunction for PrefixUnaryOperator {
    type Output = ExpressionNode;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        if let Some(token) = context
            .next_token_categoried(&[
                TokenCategory::PunctuationAmpersand,       // reference operator
                TokenCategory::PunctuationPlusSign,        // unary plus operator
                TokenCategory::PunctuationHyphenMinus,     // unary minus operator
                TokenCategory::PunctuationExclamationMark, // logical not operator
                TokenCategory::PunctuationTilde,           // bitwise not operator
                TokenCategory::PunctuationsIncrement,      // prefix increment operator
                TokenCategory::PunctuationsDecrement,      // prefix decrement operator
            ])
            .cloned()
        {
            match Expression::try_parse(context, session) {
                ParseResult::Success(node) => {
                    ParseResult::Success(ExpressionNode::Operator(OperatorExpression::Unary(
                        token.span.joined(&node.span()).expect("In the same file"),
                        match token.category {
                            TokenCategory::PunctuationAmpersand => UnaryOperator::Reference,
                            TokenCategory::PunctuationPlusSign => UnaryOperator::Plus,
                            TokenCategory::PunctuationHyphenMinus => UnaryOperator::Minus,
                            TokenCategory::PunctuationExclamationMark => UnaryOperator::LogicalNot,
                            TokenCategory::PunctuationTilde => UnaryOperator::BitwiseNot,
                            TokenCategory::PunctuationsIncrement => UnaryOperator::PrefixIncrement,
                            TokenCategory::PunctuationsDecrement => UnaryOperator::PrefixDecrement,
                            _ => panic!("Can't reach."),
                        },
                        Box::new(node),
                    )))
                }
                ParseResult::Fail(_) => ParseResult::Fail(true),
            }
        } else {
            ParseResult::Fail(false)
        }
    }
}

pub struct PostfixUnaryOperator;

impl ParseFunction for PostfixUnaryOperator {
    type Output = Box<dyn FnOnce(ExpressionNode) -> ExpressionNode>;

    fn try_parse(context: &mut ParseContext, _: &mut CompileSession) -> ParseResult<Self::Output> {
        context.skip_whitespaces(true);
        if let Some(token) = context
            .next_token_categoried(&[
                TokenCategory::PunctuationExclamationMark, // null assertion operator
                TokenCategory::PunctuationsIncrement,      // postfix increment operator
                TokenCategory::PunctuationsDecrement,      // postfix decrement operator
            ])
            .cloned()
        {
            ParseResult::Success(Box::new(move |lhs| {
                ExpressionNode::Operator(OperatorExpression::Unary(
                    token.span.joined(&lhs.span()).expect("In the same file"),
                    match token.category {
                        TokenCategory::PunctuationExclamationMark => UnaryOperator::NullAssertion,
                        TokenCategory::PunctuationsIncrement => UnaryOperator::PostfixIncrement,
                        TokenCategory::PunctuationsDecrement => UnaryOperator::PostfixDecrement,
                        _ => panic!("Can't reach."),
                    },
                    Box::new(lhs),
                ))
            }))
        } else {
            ParseResult::Fail(false)
        }
    }
}
