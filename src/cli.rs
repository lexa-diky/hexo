use std::fs::File;
use std::io::{Read, Write};
use std::panic::catch_unwind;
use std::thread::sleep;
use std::time::Duration;

use clap::{Parser, Subcommand};
use notify::event::ModifyKind;
use notify::EventKind::Modify;
use notify::{Event, RecursiveMode, Watcher};
use pest::Parser as PestParser;

use crate::ast::{HexoParser, Rule};
use crate::resolver::resolve_cst;
use crate::{ast, cst, render};

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
        output: String,
    },

    #[command(about = "Watch source and write compiled output on change")]
    Watch {
        #[arg(short, long)]
        source: String,

        #[arg(short, long)]
        output: String,
    },
}

#[derive(Debug)]
pub(crate) enum CliError {
    UnknownCommand,
    CantCreateWatcher(notify::Error),
    CantStartWatcher(notify::Error),
    CantCrateOutputFile(std::io::Error),
    CantReadInputFile(std::io::Error),
    AstParsingFailed,
    SyntaxError(pest::error::Error<Rule>),
}

pub(crate) fn run_cli() {
    let cli = Cli::parse();

    let cli_result: Result<_, CliError> = match cli.command {
        None => { Err(CliError::UnknownCommand) }
        Some(Commands::Watch { source, output }) => run_watch(source, output),
        Some(Commands::Build { source, output }) => run_build(source, output),
    };

    if cli_result.is_err() {
        match cli_result.unwrap_err() {
            CliError::UnknownCommand => println!("unknown command"),
            CliError::CantCreateWatcher(_) => println!("can't create watcher"),
            CliError::CantStartWatcher(_) => println!("can't start watcher"),
            CliError::CantCrateOutputFile(_) => println!("can't create output file"),
            CliError::CantReadInputFile(_) => println!("can't read input file"),
            CliError::AstParsingFailed => println!("ast parsing failed"),
            CliError::SyntaxError(_) => println!("syntax error")
        }
    }

    ()
}

fn run_watch(source: String, output: String) -> Result<(), CliError> {
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

fn run_watch_loop(source: String, output: String, event: Result<Event, notify::Error>) {
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

fn run_build(source: String, output: String) -> Result<(), CliError> {
    let mut source_buff = String::new();
    File::open(source)
        .map_err(|err| CliError::CantReadInputFile(err))?
        .read_to_string(&mut source_buff)
        .map_err(|err| CliError::CantReadInputFile(err))?;

    let pairs = match HexoParser::parse(Rule::file, source_buff.as_str()) {
        Ok(pairs) => pairs,
        Err(err) => return Err(CliError::SyntaxError(err))
    };

    let ast = ast::parse_ast(String::from("java_file"), pairs)
        .map_err(|err| CliError::AstParsingFailed)?;

    let cst = cst::parse_cst(ast);
    let resolved_cst = resolve_cst(cst);

    File::create(output)
        .map_err(|err| CliError::CantCrateOutputFile(err))?
        .write_all(&render::render_cst(resolved_cst))
        .map_err(|err| CliError::CantCrateOutputFile(err))
}
