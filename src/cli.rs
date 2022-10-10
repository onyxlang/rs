use std::path::Path;

use crate::program::Program;
use clap::Parser;

#[derive(Parser)]
#[clap(author = "Onyx Contributors <inbox@onyxlang.org>")]
#[clap(version)]
#[clap(about = "The canonical Onyx compiler", long_about = None)]
pub struct Cli {
    #[clap(value_parser)]
    input: String,

    #[clap(long, value_parser, default_value = "./.cache")]
    cache: String,

    #[clap(long, value_parser, default_value = "zig")]
    zig: String,
}

impl Cli {
    /// Compile and run an Onyx file.
    ///
    /// ```sh
    /// $ nx foo.nx
    /// Hello from Onyx!
    /// ```
    pub fn run() {
        let cli = Cli::parse();

        let entry_path = Path::new(&cli.input).to_path_buf();
        let cache_path = Path::new(&cli.cache).to_path_buf();
        let zig_path = Path::new(&cli.zig).to_path_buf();

        let program = Program::new(entry_path, cache_path, zig_path);
        Program::run(program);
    }
}
