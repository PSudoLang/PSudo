use super::*;
use crate::coretypes::Expression as ExpressionNode;

pub struct Expression;

macro_rules! filter_binary_operator {
    ($level:expr, $type:ty, $ident:ident) => {
        if $ident.operator_precedence > $level {
            Some(<$type>::try_parse)
        } else {
            None
        }
    };
}

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
                If::try_parse,
                PrefixUnaryOperator::try_parse,
            ],
            context,
            session,
        );
        let mut mutated = true;

        while mutated {
            mutated = false;
            result = result.flat_map(|lhs| {
                let postfix_operations: &[Option<fn(&mut _, &mut _) -> _>] = &[
                    Some(FieldGet::try_parse),
                    Some(Index::try_parse),
                    filter_binary_operator!(10, BinaryOperator10, context),
                    filter_binary_operator!(9, BinaryOperator9, context),
                    filter_binary_operator!(8, BinaryOperator8, context),
                    filter_binary_operator!(7, BinaryOperator7, context),
                    filter_binary_operator!(6, BinaryOperator6, context),
                    filter_binary_operator!(5, BinaryOperator5, context),
                    filter_binary_operator!(4, BinaryOperator4, context),
                    filter_binary_operator!(3, BinaryOperator3, context),
                    filter_binary_operator!(2, BinaryOperator2, context),
                    filter_binary_operator!(1, BinaryOperator1, context),
                ];
                match try_all(
                    &postfix_operations
                        .iter()
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
