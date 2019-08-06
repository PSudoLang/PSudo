use colored::{Color, Colorize};
use libpsudoc::coretypes::{CompileSession, Diagnostic, DiagnosticLevel, SourceFile};
use unicode_width::UnicodeWidthStr;

pub fn debug_diagnostic(
    diagnostic: &Diagnostic,
    source_file: &SourceFile,
    compile_session: &CompileSession,
) {
    let (label, color) = match diagnostic.level {
        DiagnosticLevel::Error => ("error", Color::Red),
        DiagnosticLevel::Warning => ("warning", Color::Yellow),
        DiagnosticLevel::Note => ("note", Color::Yellow),
        DiagnosticLevel::Help => ("help", Color::Cyan),
    };
    let start = diagnostic.span.start(&compile_session);
    let end = diagnostic.span.end(&compile_session);
    let line_length = std::cmp::max(start.line().to_string().len(), end.line().to_string().len());
    let indent = " ".repeat(line_length);
    println!(
        "{}: {}",
        label.color(color).bold(),
        diagnostic.message.bold()
    );
    println!(
        "{}{} {}:{}:{}",
        indent,
        "-->".cyan(),
        source_file.path(),
        start.line(),
        start.column()
    );
    println!("{} {}", indent, "|".cyan());
    let start_line_begin = source_file.line_begins[start.line() - 1];
    let end_line = if source_file.line_begins.len() == end.line() {
        source_file.src.chars().count()
    } else {
        source_file.line_begins[end.line()] - 1
    };
    let is_multilined = start.line() != end.line();
    let is_linefeed = is_multilined && end.column() == 0;
    println!(
        "{}",
        source_file
            .src
            .chars()
            .skip(start_line_begin)
            .take(end_line - start_line_begin)
            .collect::<String>()
            .split('\n')
            .enumerate()
            .map(|(index, line)| format!(
                "{} {} {}{}{}",
                start.line() + index,
                "|".cyan(),
                if is_multilined && !is_linefeed {
                    format!(
                        "{} ",
                        match (index, start.column()) {
                            (0, 0) => "/".color(color),
                            (0, _) => " ".into(),
                            _ => "|".color(color),
                        }
                    )
                } else {
                    "".to_string()
                },
                line,
                if is_linefeed && index == 0 {
                    format!(
                        "\n{} {} {}{}",
                        indent,
                        "|".cyan(),
                        " ".repeat(
                            line.chars()
                                .take(start.column())
                                .collect::<String>()
                                .width()
                        ),
                        "^".red()
                    )
                } else if is_multilined && index == 0 && start.column() != 0 {
                    format!(
                        "\n{} {}  {}{}",
                        indent,
                        "|".cyan(),
                        "_".repeat(
                            line.chars()
                                .take(start.column())
                                .collect::<String>()
                                .width()
                                + 1
                        )
                        .red(),
                        "^".red()
                    )
                } else {
                    "".into()
                }
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    if !is_linefeed {
        println!(
            "{} {} {}",
            indent,
            "|".cyan(),
            if is_multilined {
                format!("|{}^", "_".repeat(end.column()))
            } else {
                let line: String = source_file
                    .src
                    .chars()
                    .skip(source_file.line_begins[start.line() - 1])
                    .take(end.column())
                    .collect();
                format!(
                    "{}{}",
                    " ".repeat(
                        line.chars()
                            .take(start.column())
                            .collect::<String>()
                            .width()
                    ),
                    "^".repeat(
                        line.chars()
                            .skip(start.column())
                            .take(end.column() - start.column())
                            .collect::<String>()
                            .width()
                    )
                )
            }
            .red()
        );
    }
    println!("{} {}", indent, "|".cyan());
}
