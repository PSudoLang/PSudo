use super::*;

pub struct DiagnosticPrelude;

impl DiagnosticRenderer for DiagnosticPrelude {
    fn render(info: &DiagnosticRenderInformation) {
        let indent = " ".repeat(info.line_number_length);
        println!(
            "{label}: {message}",
            label = &info
                .diagnostic_label()
                .color(info.diagnostic_color())
                .bold(),
            message = info.diagnostic.message.bold()
        );
        println!(
            "{indent}{arrow} {path}:{line}:{column}",
            indent = indent,
            arrow = "-->".blue(),
            path = info.source_file.path(),
            line = info.start().line(),
            column = info.start().column()
        );
    }
}
