use super::*;
use crate::coretypes::Expression as ExpressionNode;

pub struct Expression;

impl ParseFunction for Expression {
    type Output = ExpressionNode;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let mut result = try_all(
            &[
                BooleanLiteral::try_parse,
                StringLiteral::try_parse,
                NumberLiteral::try_parse,
                Group::try_parse,
                Array::try_parse,
                PrefixUnaryOperator::try_parse,
            ],
            context,
            session,
        );
        let mut mutated = true;

        while mutated {
            mutated = false;
            result = result.flat_map(|lhs| {
                let hmm: &[Option<fn(&mut _, &mut _) -> _>] = &[
                    if context.operator_precedence > 2 {
                        Some(BinaryOperator2::try_parse)
                    } else {
                        None
                    },
                    if context.operator_precedence > 1 {
                        Some(BinaryOperator1::try_parse)
                    } else {
                        None
                    },
                ];
                match try_all(
                    &hmm.iter()
                        .filter(|it| it.is_some())
                        .map(|it| it.unwrap())
                        .collect::<Box<[_]>>(),
                    context,
                    session,
                ) {
                    ParseResult::Success(f) => {
                        mutated = true;
                        ParseResult::Success(f(lhs))
                    }
                    ParseResult::Fail(val) => {
                        if val {
                            ParseResult::Fail(true)
                        } else {
                            ParseResult::Success(lhs)
                        }
                    }
                }
            });
        }

        result
    }
}
