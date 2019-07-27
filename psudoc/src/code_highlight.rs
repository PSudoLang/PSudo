use crate::util::read_file;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::syntax_definition::SyntaxDefinition;
use syntect::parsing::{SyntaxReference, SyntaxSet, SyntaxSetBuilder};
pub use syntect::util::as_24_bit_terminal_escaped as render_style;

pub fn initialize() -> (SyntaxReference, SyntaxSet, Theme) {
    let mut builder = SyntaxSetBuilder::new();
    let path = Path::new("./src/psudo.sublime-syntax");
    let s = read_file(&path);

    let syntax =
        SyntaxDefinition::load_from_str(&s, false, path.file_stem().and_then(|x| x.to_str()))
            .unwrap();

    builder.add(syntax);

    let syntax_set = builder.build();
    let theme = ThemeSet::get_theme(Path::new("./src/Nord.tmTheme")).unwrap();

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
