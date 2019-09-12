use super::*;

use crate::coretypes::{
    ControlFlowExpression, Else, Expression as ExpressionNode, If as IfNode, Spanned, TokenCategory,
};
use crate::parse::rules::Block;

pub struct If;

impl ParseFunction for If {
    type Output = ExpressionNode;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let if_token =
            if let Some(token) = context.next_token_categoried(&[TokenCategory::KeywordIf]) {
                token.clone()
            } else {
                return ParseResult::Fail(false);
            };

        context.skip_whitespaces(true);

        let condition = match Expression::try_parse(context, session) {
            ParseResult::Success(node) => Box::new(node),
            ParseResult::Fail(val) => {
                if !val {
                    let (span, received) = if let Some(token) = context.peek() {
                        (
                            token.span.clone(),
                            token.span.source_text(session).escape_debug().to_string(),
                        )
                    } else {
                        (if_token.span, "No token".to_string())
                    };
                    span.diagnostic_error(format!(
                        "Expected expression as condition, but {} received",
                        received
                    ))
                    .emit_to(session);
                }
                return ParseResult::Fail(true);
            }
        };
        context.skip_whitespaces(true);

        let if_branch = match Block::try_parse(context, session) {
            ParseResult::Success(node) => node,
            ParseResult::Fail(val) => {
                if !val {
                    let (span, received) = if let Some(token) = context.peek() {
                        (
                            token.span.clone(),
                            token.span.source_text(session).escape_debug().to_string(),
                        )
                    } else {
                        (if_token.span, "No token".to_string())
                    };
                    span.diagnostic_error(format!("Expected if body, but {} received", received))
                        .emit_to(session);
                }
                return ParseResult::Fail(true);
            }
        };

        context.skip_whitespaces(true);

        let else_token =
            if let Some(token) = context.next_token_categoried(&[TokenCategory::KeywordElse]) {
                token.clone()
            } else {
                return ParseResult::Success(ExpressionNode::ControlFlow(
                    ControlFlowExpression::If(IfNode {
                        span: if_token
                            .span
                            .joined(&if_branch.span())
                            .clone()
                            .expect("In the same file"),
                        condition,
                        if_branch,
                        else_branch: None,
                    }),
                ));
            };

        context.skip_whitespaces(true);

        let else_branch = if context
            .peek()
            .map(|token| token.category == TokenCategory::KeywordIf)
            .unwrap_or(false)
        {
            match If::try_parse(context, session) {
                ParseResult::Success(node) => Else::ElseIf(
                    else_token
                        .span
                        .joined(&node.span())
                        .expect("In the same file"),
                    Box::new(match node {
                        ExpressionNode::ControlFlow(ControlFlowExpression::If(r#if)) => r#if,
                        _ => panic!("Never happen"),
                    }),
                ),
                ParseResult::Fail(_) => return ParseResult::Fail(true),
            }
        } else {
            match Block::try_parse(context, session) {
                ParseResult::Success(node) => Else::Else(
                    else_token
                        .span
                        .joined(&node.span())
                        .expect("In the same file"),
                    node,
                ),
                ParseResult::Fail(_) => return ParseResult::Fail(true),
            }
        };

        ParseResult::Success(ExpressionNode::ControlFlow(ControlFlowExpression::If(
            IfNode {
                span: if_token
                    .span
                    .joined(&if_branch.span())
                    .clone()
                    .expect("In the same file"),
                condition,
                if_branch,
                else_branch: Some(Box::new(else_branch)),
            },
        )))
    }
}
