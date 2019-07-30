mod code_highlight;
mod util;

use std::path::Path;

use libpsudoc::coretypes::{CompileSession, RichDebug, SourceFile};
use libpsudoc::parse::{ParseContext, ParseFunction, ParseResult, Root};
use libpsudoc::tokenize::Tokenizer;

fn main() {
    let source = SourceFile::create_real(Path::new("./test.psudo")).unwrap();
    let source_string = source.src.clone();

    let mut compile_session = CompileSession::new();

    let tokenized: Vec<_> = Tokenizer::new(&source).collect();

    compile_session.add_source_file(source);

    for token in tokenized.iter() {
        println!(
            "Token({:?}, span={}, {:?})",
            token.category,
            // token.span,
            token.span.rich_debug(&compile_session),
            source_string
                .chars()
                .skip(token.span.start_offset)
                .take(token.span.length())
                .collect::<String>()
        );
    }

    let parsed: ParseResult = Root::try_parse(&mut ParseContext::new(tokenized));

    match parsed {
        ParseResult::Success(node) => {
            println!("{}", node.rich_debug(&compile_session));
        }
        ParseResult::Skip => {
            println!("Parse Skipped");
        }
        ParseResult::Fail => {
            println!("Parse Failed");
        }
    }

    let (syntax_reference, syntax_set, theme) = code_highlight::initialize();
    let highlighted_lines =
        code_highlight::highlight(&syntax_reference, &syntax_set, &theme, &source_string);

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
