use super::*;
use crate::coretypes::Node;

pub struct Root;

impl ParseFunction for Root {
    type Output = Node;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let mut nodes = Vec::new();

        let mut failed = false;

        while context.has_next() {
            println!("{}", context.last_read_token().span.source_text(session));
            context.skip_whitespaces();
            match try_all(
                vec![
                    LineComment::try_parse,
                    BlockComment::try_parse,
                    Whitespace::try_parse,
                    NodeStatement::try_parse,
                ],
                context,
                session,
            ) {
                ParseResult::Success(node) => {
                    nodes.push(node);
                }
                ParseResult::Skip => {}
                ParseResult::Fail(val) => {
                    if !val {
                        context.last_read_token()
                            .span
                            .diagnostic_error("Unexpected token")
                            .emit_to(session);
                        return ParseResult::Fail(true);
                    }

                    failed = true;
                }
            }
        }

        if failed {
            ParseResult::Fail(true)
        } else {
            ParseResult::Success(Node::Root(nodes))
        }
    }
}
