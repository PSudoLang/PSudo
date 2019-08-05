use std::io::Cursor;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::syntax_definition::SyntaxDefinition;
use syntect::parsing::{SyntaxReference, SyntaxSet, SyntaxSetBuilder};
pub use syntect::util::as_24_bit_terminal_escaped as render_style;

pub fn initialize() -> (SyntaxReference, SyntaxSet, Theme) {
    let mut builder = SyntaxSetBuilder::new();

    let syntax_string = include_str!("../assets/psudo.sublime-syntax");

    let syntax =
        SyntaxDefinition::load_from_str(&syntax_string, false, Some(syntax_string)).unwrap();

    builder.add(syntax);

    let mut theme = Cursor::new(include_bytes!("../assets/Nord.tmTheme").as_ref());

    let syntax_set = builder.build();
    let theme = ThemeSet::load_from_reader(&mut theme).unwrap();

    (syntax_set.syntaxes()[0].clone(), syntax_set, theme.clone())
}

pub fn highlight<'a, 'b>(
    syntax: &SyntaxReference,
    syntax_set: &SyntaxSet,
    theme: &'a Theme,
    line: &'b str,
) -> Vec<(Style, &'b str)> {
    HighlightLines::new(syntax, theme).highlight(line, syntax_set)
}
