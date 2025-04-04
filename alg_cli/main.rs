use std::io::{read_to_string, stdin, stdout};
use std::process::exit;
use std::str::FromStr;

use clap::{Args, Command, CommandFactory, Parser, Subcommand};
use clap_complete::generator::generate;
use clap_complete::{Generator, Shell};
use cubing::alg::Alg;

/// Alg tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(name = "alg")]
pub struct AlgCLIArgs {
    #[command(subcommand)]
    pub command: AlgCLICommand,
}

#[derive(Subcommand, Debug)]
pub enum AlgCLICommand {
    /// Parse the provided alg (i.e. validate syntax)
    Parse(AlgSource),

    /// Invert the provided alg
    Invert(AlgSource),

    /// Print completions for the given shell.
    Completions(CompletionsArgs),
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct AlgSource {
    #[clap(group = "alg_source")]
    alg: Option<String>,

    #[clap(long, group = "alg_source")]
    stdin: bool,
}

#[derive(Args, Debug)]
pub struct CompletionsArgs {
    /// Print completions for the given shell.
    /// These can be loaded/stored permanently (e.g. when using Homebrew), but they can also be sourced directly, e.g.:
    ///
    ///  twsearch completions fish | source # fish
    ///  source <(twsearch completions zsh) # zsh
    #[clap(verbatim_doc_comment, id = "SHELL")]
    shell: Shell,
}

fn completions_for_shell(cmd: &mut Command, generator: impl Generator) {
    generate(generator, cmd, "alg", &mut stdout());
}

pub fn get_options() -> AlgCLIArgs {
    let mut command = AlgCLIArgs::command();

    let args = AlgCLIArgs::parse();
    if let AlgCLICommand::Completions(completions_args) = args.command {
        completions_for_shell(&mut command, completions_args.shell);
        exit(0);
    };

    args
}

fn main() {
    let args = get_options();

    match args.command {
        AlgCLICommand::Parse(parse_args) => {
            let alg = match parse_args.alg {
                Some(alg) => alg,
                None => {
                    assert!(parse_args.stdin);
                    read_to_string(stdin()).unwrap()
                }
            };
            let exit_code = match Alg::from_str(&alg) {
                Ok(_alg) => {
                    eprintln!("Alg parsed successfully.");
                    0
                }
                Err(e) => {
                    eprintln!("Invalid alg: {}", e);
                    1
                }
            };
            exit(exit_code)
        }
        AlgCLICommand::Invert(invert_args) => {
            let alg = match invert_args.alg {
                Some(alg) => alg,
                None => {
                    assert!(invert_args.stdin);
                    read_to_string(stdin()).unwrap()
                }
            };
            let alg = match Alg::from_str(&alg) {
                Ok(alg) => alg,
                Err(e) => {
                    eprintln!("Invalid alg: {}", e);
                    exit(1);
                }
            };
            println!("{}", alg.invert())
        }
        AlgCLICommand::Completions(_completions_args) => {
            panic!("Completions should have been printed during options parsing, followed by program exit.");
        }
    }
}
