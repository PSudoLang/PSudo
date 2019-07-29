mod code_highlight;
mod util;

use std::path::Path;

use crate::util::read_file;

use libpsudoc::tokenize::Tokenizer;
use libpsudoc::parse::{ParseContext, ParseResult, ParseFunction, Root};

fn main() {
    let source = read_file(&Path::new("./test.psudo"));
    let source: &'static str = Box::leak(source.into_boxed_str());

    // println!("{}", source);

    let tokenized: Vec<_> = Tokenizer::new(source.chars().collect::<Vec<_>>()).collect();

    for token in tokenized.iter() {
        println!(
            "Token({:?}, span={}, {:?})",
            token.category,
            token.span,
            source
                .chars()
                .skip(token.span.start.offset)
                .take(token.span.end.offset - token.span.start.offset)
                .collect::<String>()
        );
    }

    let parsed: ParseResult = Root::try_parse(&mut ParseContext::new(tokenized));

    let (syntax_reference, syntax_set, theme) = code_highlight::initialize();
    let highlighted_lines =
        code_highlight::highlight(&syntax_reference, &syntax_set, &theme, &source);

    println!();

    for highlighted_line in highlighted_lines {
        print!(
            "{}",
            code_highlight::render_style(&[highlighted_line], false)
        );
    }

    println!();
    println!();
}
