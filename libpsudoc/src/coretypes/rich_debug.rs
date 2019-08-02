use super::CompileSession;

pub trait RichDebug {
    fn rich_debug(&self, session: &CompileSession) -> String;
}

pub fn indented<T: ToString>(s: T) -> String {
    s.to_string()
        .split('\n')
        .map(|line| format!("  {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}
