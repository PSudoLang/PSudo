use super::CompileSession;

pub trait RichDebug {
        fn rich_debug(&self, session: &CompileSession) -> String;
}
