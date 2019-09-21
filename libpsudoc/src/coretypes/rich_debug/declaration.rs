use crate::coretypes::{CompileSession, Declaration, RichDebug};
use crate::util::indented;

impl RichDebug for Declaration {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Declaration::Function(_, name, parameters, return_type, block) => format!(
                "Function {:?} returns {} {{\n{}\n}}",
                name,
                return_type.name.to_string(),
                indented(format!(
                    "{}\n{}",
                    if parameters.is_empty() {
                        "no parameters".into()
                    } else {
                        format!(
                            "parameters = (\n{}\n)",
                            indented(
                                parameters
                                    .iter()
                                    .map(|(name, r#type)| format!(
                                        "{}: {}",
                                        name,
                                        r#type.name.to_string()
                                    ))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            )
                        )
                    },
                    if block.1.is_empty() {
                        "empty body".into()
                    } else {
                        format!(
                            "body {{\n{}\n}}",
                            indented(
                                block
                                    .1
                                    .iter()
                                    .map(|statement| statement.rich_debug(session))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            )
                        )
                    }
                ))
            ),
            _ => "Unknown Declaration".into(),
        }
    }
}
