use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};
use std::panic::catch_unwind;
use std::thread::sleep;
use std::time::Duration;

use clap::{Parser, Subcommand};
use console::style;
use notify::event::ModifyKind;
use notify::EventKind::Modify;
use notify::{Event, RecursiveMode, Watcher};
use pest::Parser as PestParser;

use crate::resolver::resolve_cst;
use crate::{cst_legacy, render};
use crate::compiler::ast::{AstParser, AstParserError};
use crate::render::RenderError;

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
    Rendering(RenderError),
    CantReadInputFile(std::io::Error),
    AstParsingFailed(AstParserError),
    CstParsingFailed,
}

pub(crate) fn run_cli() {
    let cli = Cli::parse();

    let cli_result: Result<_, CliError> = match cli.command {
        None => { Err(CliError::UnknownCommand) }
        Some(Commands::Watch { source, output }) => run_watch(source, output),
        Some(Commands::Build { source, output }) => run_build(source, output),
    };

    handle_cli_error(cli_result);

    ()
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
            CliError::CstParsingFailed => eprintln!("cst parsing failed"),
            CliError::Rendering(error) => handle_render_error(error),
        }
    }
}

fn handle_render_error(error: RenderError) {
    match error {
        RenderError::UnresolvedAtom { atom } =>
            eprintln!("unresolved atom: {:?}", atom)
    }
}

fn print_error(message: &str, error: Box<dyn Display>) {
    println!("{} {}", style("e:").red().bold(), style(message).red());
    println!("{}", style(error).red());
}

fn run_watch(source: String, output: Option<String>) -> Result<(), CliError> {
    let source_path_clone = source.clone();
    let source_path = source_path_clone.as_ref();

    let mut watcher = notify::recommended_watcher(
        move |event: Result<Event, _>| run_watch_loop(source.clone(), output.clone(), event)
    ).map_err(|err| CliError::CantCreateWatcher(err))?;

    watcher
        .watch(source_path, RecursiveMode::NonRecursive)
        .map_err(|err| CliError::CantStartWatcher(err))?;

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
    let mut source_buff = String::new();
    File::open(source.clone())
        .map_err(|err| CliError::CantReadInputFile(err))?
        .read_to_string(&mut source_buff)
        .map_err(|err| CliError::CantReadInputFile(err))?;

    let ast_parser = AstParser::new();

    let ast = ast_parser.parse(source_buff).unwrap();

    let cst = cst_legacy::parse_cst(ast).map_err(|_| CliError::CstParsingFailed)?;
    let resolved_cst = resolve_cst(cst);

    let output_file_path = output.unwrap_or(format!("{}.bin", source));
    File::create(output_file_path)
        .map_err(|err| CliError::CantCrateOutputFile(err))?
        .write_all(
            &render::render_cst(resolved_cst)
                .map_err(CliError::Rendering)?,
        )
        .map_err(|err| CliError::CantCrateOutputFile(err))
}
