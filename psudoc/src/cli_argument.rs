pub use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
/// PSudo: The Programming Language for Competitive Programming
pub struct PsudocCliArgument {
    #[structopt(parse(from_os_str))]
    /// The file which to compile or run
    pub file: PathBuf,
    #[structopt(long = "print-tokens")]
    /// Debug Only: Whether to print tokens or not.
    pub print_tokens: bool,
    #[structopt(long = "print-nodes")]
    /// Debug Only: Whether to print nodes or not.
    pub print_nodes: bool,
    #[structopt(long = "print-codes")]
    /// Debug Only: Whether to print codes with highlight or not.
    pub print_codes: bool,
}
