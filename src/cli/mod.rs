use std::fmt::Debug;
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

pub(crate) use error::Error;

use crate::compiler::{FileCompilerSource, HexoCompiler, HexoCompilerContext};

mod error;

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

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub(crate) fn run() {
        let cli = Cli::parse();

        let cli_result: Result<_, Error> = match cli.command {
            None => Err(Error::UnknownCommand),
            Some(Commands::Watch { source, output }) => Self::watch(source, output),
            Some(Commands::Build { source, output }) => Self::build(source, output),
        };

        Self::handle_cli_error_if_required(cli_result);
    }

    fn handle_cli_error_if_required(cli_result: Result<(), Error>) {
        if let Err(e) = cli_result {
            Self::print_error(e.into());
        }
    }

    fn print_error(error: Box<dyn std::error::Error>) {
        eprintln!("{}", style(error).red());
    }

    fn watch(source: String, output: Option<String>) -> Result<(), Error> {
        let source_path_clone = source.clone();
        let source_path = source_path_clone.as_ref();

        let mut watcher = notify::recommended_watcher(move |event: Result<Event, _>| {
            Self::watch_loop(source.clone(), output.clone(), event)
        })
        .map_err(Error::FileWatcher)?;

        watcher
            .watch(source_path, RecursiveMode::NonRecursive)
            .map_err(Error::FileWatcher)?;

        println!("watcher started");

        sleep(Duration::MAX);

        Ok(())
    }

    fn watch_loop(source: String, output: Option<String>, event: Result<Event, notify::Error>) {
        if let Ok(e) = event {
            if let Modify(ModifyKind::Data(_)) = e.kind {
                println!("rebuilding...");
                let _ = catch_unwind(|| Self::build(source.clone(), output.clone()));
                println!(" done!");
            }
        } else {
            Self::print_error(event.unwrap_err().into());
        }
    }

    pub(crate) fn build(source: String, output: Option<String>) -> Result<(), Error> {
        let context = HexoCompilerContext::new();
        let compiler = HexoCompiler::new(context);

        let compiler_source = FileCompilerSource::new(PathBuf::from(source.clone()));

        let compilation_result = compiler
            .compile(&compiler_source)
            .map_err(Error::Compilation)?;

        let output_file_path = output.unwrap_or(format!("{}.bin", source));

        File::create(output_file_path)
            .map_err(Error::CantCrateOutputFile)?
            .write_all(compilation_result.content.iter().as_slice())
            .map_err(Error::CantCrateOutputFile)
    }
}
