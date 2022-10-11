use std::path::{Path, PathBuf};

use crate::program::Program;
use clap::{command, Parser};

// I want the default command to be `run`.

#[derive(Parser)]
#[clap(author = "Onyx Contributors <inbox@onyxlang.org>")]
#[clap(about = "The canonical Onyx compiler", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    action: Action,

    #[clap(
        long,
        value_parser,
        default_value = ".cache",
        help = "Path to the cache directory"
    )]
    cache: String,
}

#[derive(clap::Subcommand)]
enum Action {
    #[clap(about = "Run an Onyx program", alias = "r")]
    Run {
        #[clap(value_parser, help = "Input file path")]
        input: String,

        #[clap(
            long,
            value_parser,
            default_value = "zig",
            help = "Zig executable path"
        )]
        zig: String,
    },

    #[clap(about = "Compile an Onyx program", alias = "c")]
    Compile {
        #[clap(value_parser, help = "Input file path")]
        input: String,

        #[clap(short, long, value_parser, help = "Output executable path")]
        output: Option<String>,

        #[clap(
            long,
            value_parser,
            default_value = "zig",
            help = "Zig executable path"
        )]
        zig: String,
    },
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        let result = match cli.action {
            Action::Run { input, zig } => {
                let program = Program::new(Path::new(&input).into(), Path::new(&cli.cache).into());
                Program::run(program, Path::new(&zig).into())
            }
            Action::Compile { input, output, zig } => {
                let program = Program::new(Path::new(&input).into(), Path::new(&cli.cache).into());

                let output_path = match output {
                    Some(output) => Path::new(&output).into(),
                    None => {
                        let mut path = PathBuf::from(&input);
                        path.set_extension("");
                        path
                    }
                };

                Program::compile(program, output_path, Path::new(&zig).into())
            }
        };

        if let Err(panic) = result {
            print!("{}", panic);
            std::process::exit(1);
        }
    }
}
