use super::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref BAR: ColoredString = "|".blue();
}

pub struct SourceView;

impl DiagnosticRenderer for SourceView {
    fn render(info: &DiagnosticRenderInformation) {
        EmptySource::render(info);
        if info.start().line() == info.end().line() {
            SinglelineSource::render(info);
        } else {
            MultilineSource::render(info);
        }
        EmptySource::render(info);
    }
}

pub struct EmptySource;

impl DiagnosticRenderer for EmptySource {
    fn render(info: &DiagnosticRenderInformation) {
        println!(
            "{indent} {bar}",
            indent = " ".repeat(info.line_number_length),
            bar = *BAR
        );
    }
}

pub struct SinglelineSource;

impl DiagnosticRenderer for SinglelineSource {
    fn render(info: &DiagnosticRenderInformation) {
        let line_text = info.source_file.src.slice_by_char(
            info.source_file.line_begins[info.start().line() - 1]
                ..info.source_file.line_begins[info.start().line()],
        );
        println!(
            "{line_number} {bar} {line_text}",
            line_number = info.start().line(),
            bar = *BAR,
            line_text = line_text,
        );
        println!(
            "{indent} {bar} {spaces}{carets}",
            indent = " ".repeat(info.line_number_length),
            bar = *BAR,
            spaces = " ".repeat(line_text.take_by_char(info.start().column()).width()),
            carets = "^"
                .repeat(
                    line_text
                        .slice_by_char(info.start().column()..=info.end().column())
                        .width()
                )
                .red()
        );
    }
}

pub struct MultilineSource;

impl DiagnosticRenderer for MultilineSource {
    fn render(info: &DiagnosticRenderInformation) {
        MultilineSourceHead::render(info);
        MultilineSourceBody::render(info);
        MultilineSourceTail::render(info);
    }
}

pub struct MultilineSourceHead;

impl DiagnosticRenderer for MultilineSourceHead {
    fn render(info: &DiagnosticRenderInformation) {
        let line = info.source_file.src.slice_by_char(
            info.source_file.line_begins[info.start().line() - 1]
                ..info.source_file.line_begins[info.start().line()],
        );
        println!(
            "{line_number} {bar} {source_range_bar}{line_text}",
            line_number = info.start().line(),
            bar = *BAR,
            source_range_bar =
                if info.start().column() == 0 { "/" } else { " " }.color(info.diagnostic_color()),
            line_text = line,
        );
        if info.start().column() == 0 {
            return;
        }
        println!(
            "{indent} {bar}  {source_range_indicator}",
            indent = " ".repeat(info.line_number_length),
            bar = *BAR,
            source_range_indicator = format!(
                "{horizontal_line}^",
                horizontal_line = "_".repeat(line.take_by_char(info.start().column()).width()),
            )
            .red()
        );
    }
}

pub struct MultilineSourceBody;

impl DiagnosticRenderer for MultilineSourceBody {
    fn render(info: &DiagnosticRenderInformation) {
        match info.end().line() - info.start().line() {
            1 => (),
            2 | 3 | 4 => {
                info.source_file
                    .src
                    .slice_by_char(
                        info.source_file.line_begins[info.start().line() + 1]
                            ..=info.source_file.line_begins[info.end().line() - 1],
                    )
                    .split('\n')
                    .enumerate()
                    .for_each(|(index, line_text)| {
                        println!(
                            "{line_number} {bar} {source_range_bar}{line_text}",
                            line_number = info.start().line() + index,
                            bar = *BAR,
                            source_range_bar = "|".color(info.diagnostic_color()),
                            line_text = line_text
                        )
                    });
            }
            _ => {
                info.source_file
                    .src
                    .slice_by_char(
                        info.source_file.line_begins[info.start().line() + 1]
                            ..=info.source_file.line_begins[info.start().line() + 3],
                    )
                    .split('\n')
                    .enumerate()
                    .for_each(|(index, line_text)| {
                        println!(
                            "{line_number} {bar} {source_range_bar}{line_text}",
                            line_number = info.start().line() + index,
                            bar = *BAR,
                            source_range_bar = "|".color(info.diagnostic_color()),
                            line_text = line_text
                        )
                    });
                println!(
                    "{dotdotdot} {indent} {source_range_bar}",
                    dotdotdot = "...".blue(),
                    indent = " ".repeat(std::cmp::max(info.line_number_length, 3) - 3),
                    source_range_bar = "|".color(info.diagnostic_color()),
                );
                info.source_file
                    .src
                    .slice_by_char(
                        info.source_file.line_begins[info.end().line() - 3]
                            ..=info.source_file.line_begins[info.end().line() - 1],
                    )
                    .split('\n')
                    .enumerate()
                    .for_each(|(index, line_text)| {
                        println!(
                            "{line_number} {bar} {source_range_bar}{line_text}",
                            line_number = info.end().line() - 3 + index + 1,
                            bar = *BAR,
                            source_range_bar = "|".color(info.diagnostic_color()),
                            line_text = line_text
                        )
                    });
            }
        }
    }
}

pub struct MultilineSourceTail;

impl DiagnosticRenderer for MultilineSourceTail {
    fn render(info: &DiagnosticRenderInformation) {
        let line = info.source_file.src.slice_by_char(
            info.source_file.line_begins[info.end().line() - 1]
                ..info.source_file.line_begins[info.end().line()],
        );
        println!(
            "{line_number} {bar} {source_range_bar}{line_text}",
            line_number = info.end().line(),
            bar = *BAR,
            source_range_bar =
                if line.is_empty() { "\\" } else { "|" }.color(info.diagnostic_color()),
            line_text = line,
        );
        if info.end().column() == 0 {
            return;
        }
        println!(
            "{indent} {bar}  {source_range_indicator}",
            indent = " ".repeat(info.line_number_length),
            bar = *BAR,
            source_range_indicator = format!(
                "{horizontal_line}^",
                horizontal_line = "_".repeat(line.width()),
            )
            .red()
        );
    }
}
