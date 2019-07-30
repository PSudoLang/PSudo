use super::*;
use crate::coretypes::Node;

type ToContinue = bool;

pub struct Root;

impl ParseFunction for Root {
    fn try_parse(context: &mut ParseContext) -> ParseResult {
        let mut nodes = Vec::new();

        fn try_parse<F>(
            nodes: &mut Vec<Node>,
            context: &mut ParseContext,
            parse_function: F,
        ) -> ToContinue
        where
            F: Fn(&mut ParseContext) -> ParseResult,
        {
            let sandbox = &mut context.create_sandbox();
            let result = parse_function(sandbox);
            match result {
                ParseResult::Success(node) => {
                    nodes.push(node);
                    context.current += sandbox.current;
                    true
                }
                ParseResult::Skip => {
                    context.current += sandbox.current;
                    true
                }
                ParseResult::Fail => false,
            }
        }

        while context.has_next() {
            if try_parse(&mut nodes, context, LineComment::try_parse) {
                continue;
            }
            if try_parse(&mut nodes, context, BlockComment::try_parse) {
                continue;
            }
            if try_parse(&mut nodes, context, Whitespace::try_parse) {
                continue;
            }

            return ParseResult::Fail;
        }

        ParseResult::Success(Node::Root(nodes))
    }
}
