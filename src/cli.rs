use std::fmt::{Debug};
use std::fs::File;
use std::io::Write;
use std::panic::catch_unwind;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use clap::{Parser, Subcommand};
use console::style;
use notify::event::ModifyKind;
use notify::EventKind::Modify;
use notify::{Event, RecursiveMode, Watcher};

use crate::compiler::{FileCompilerSource, HexoCompiler, HexoCompilerContext};
use crate::compiler::ast::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Compile source and write compiled output on change")]
    Build {
        #[arg(short, long)]
        source: String,

        #[arg(short, long)]
        output: Option<String>,
    },

    #[command(about = "Watch source and write compiled output on change")]
    Watch {
        #[arg(short, long)]
        source: String,

        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Debug)]
pub(crate) enum CliError {
    UnknownCommand,
    CantCreateWatcher(notify::Error),
    CantStartWatcher(notify::Error),
    CantCrateOutputFile(std::io::Error),
    CantReadInputFile(std::io::Error),
    AstParsingFailed(Error),
    CompilationError(crate::compiler::CompilerError),
}

pub(crate) fn run_cli() {
    let cli = Cli::parse();

    let cli_result: Result<_, CliError> = match cli.command {
        None => Err(CliError::UnknownCommand),
        Some(Commands::Watch { source, output }) => run_watch(source, output),
        Some(Commands::Build { source, output }) => run_build(source, output),
    };

    handle_cli_error(cli_result);
}

fn handle_cli_error(cli_result: Result<(), CliError>) {
    if cli_result.is_err() {
        match cli_result.unwrap_err() {
            CliError::UnknownCommand => eprintln!("unknown command"),
            CliError::CantCreateWatcher(_) => eprintln!("can't create watcher"),
            CliError::CantStartWatcher(_) => eprintln!("can't start watcher"),
            CliError::CantCrateOutputFile(_) => eprintln!("can't create output file"),
            CliError::CantReadInputFile(_) => eprintln!("can't read input file"),
            CliError::AstParsingFailed(_) => eprintln!("ast parsing error"),
            CliError::CompilationError(compilation_error) => {
                handle_compilation_error(compilation_error)
            }
        }
    }
}

fn handle_compilation_error(err: crate::compiler::CompilerError) {
    print_error("compilation error", Box::new(err));
}

fn print_error(message: &str, error: Box<dyn Debug>) {
    println!("{} {}", style("e:").red().bold(), style(message).red());
    println!("{:?}", style(error).red());
}

fn run_watch(source: String, output: Option<String>) -> Result<(), CliError> {
    let source_path_clone = source.clone();
    let source_path = source_path_clone.as_ref();

    let mut watcher = notify::recommended_watcher(move |event: Result<Event, _>| {
        run_watch_loop(source.clone(), output.clone(), event)
    })
    .map_err(CliError::CantCreateWatcher)?;

    watcher
        .watch(source_path, RecursiveMode::NonRecursive)
        .map_err(CliError::CantStartWatcher)?;

    println!("watcher started");

    sleep(Duration::MAX);

    Ok(())
}

fn run_watch_loop(source: String, output: Option<String>, event: Result<Event, notify::Error>) {
    match event {
        Ok(e) => {
            if let Modify(ModifyKind::Data(_)) = e.kind {
                print!("rebuilding...");
                let _ = catch_unwind(|| run_build(source.clone(), output.clone()));
                println!(" done!");
            }
        }
        Err(e) => {
            println!("watch error: {:?}", e)
        }
    }
}

pub(crate) fn run_build(source: String, output: Option<String>) -> Result<(), CliError> {
    let context = HexoCompilerContext::new();
    let compiler = HexoCompiler::new(context);

    let compiler_source = FileCompilerSource::new(PathBuf::from(source.clone()));

    let compilation_result = compiler
        .compile(&compiler_source)
        .map_err(CliError::CompilationError)?;

    let output_file_path = output.unwrap_or(format!("{}.bin", source));

    File::create(output_file_path)
        .map_err(CliError::CantCrateOutputFile)?
        .write_all(compilation_result.content.iter().as_slice())
        .map_err(CliError::CantCrateOutputFile)
}
