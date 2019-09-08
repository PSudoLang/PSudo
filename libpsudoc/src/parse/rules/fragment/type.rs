use super::*;
use crate::coretypes::{TokenCategory, Type as TypeData, TypeName};

pub struct Type;

impl ParseFunction for Type {
    type Output = TypeData;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        // TODO: Support of path like std::HashMap?
        // TODO: Unit and never type
        let type_name = if let Some(token) =
            context.next_if_matched(|token| token.category == TokenCategory::IdentifierIdentifier)
        {
            token.clone()
        } else {
            return ParseResult::Fail(false);
        };

        let name = type_name.span.source_text(session);

        ParseResult::Success(TypeData {
            span: type_name.span.clone(),
            name: match name.as_str() {
                "!" => TypeName::Never,
                "()" => TypeName::Unit,
                _ => TypeName::Path(name),
            },
        })
    }
}
