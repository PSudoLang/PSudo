mod cli_argument;
mod code_highlight;
mod debug_diagnostic;

use cli_argument::*;
use debug_diagnostic::debug_diagnostic;

use libpsudoc::coretypes::{CompileSession, RichDebug, SourceFile};
use libpsudoc::parse::{ParseContext, ParseFunction, ParseResult, Root};
use libpsudoc::tokenize::Tokenizer;

fn main() {
    let options = PsudocCliArgument::from_args();

    let source = SourceFile::create_real(options.file).unwrap();
    let source_key = source.unique_key;
    let source_string = source.src.clone();

    let mut compile_session = CompileSession::new();

    let tokenized: Vec<_> = Tokenizer::new(&source).collect();

    compile_session.add_source_file(source);

    if options.print_tokens {
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
    }
    let parsed = Root::try_parse(&mut ParseContext::new(tokenized), &mut compile_session);

    for diagnostic in compile_session.diagnostics() {
        debug_diagnostic(
            diagnostic,
            compile_session
                .get_source_file(source_key)
                .expect("Added file"),
            &compile_session,
        );
    }

    match parsed {
        ParseResult::Success(node) => {
            if options.print_nodes {
                println!("{}", node.rich_debug(&compile_session));
            }
        }
        ParseResult::Fail(_) => {
            println!("Parse Failed");
            std::process::exit(1);
        }
    }

    if options.print_codes {
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
}
