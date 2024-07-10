use std::fs::File;
use std::io::Write;
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, Instant};

use clap::{Parser, Subcommand, ValueEnum};
use clap::builder::PossibleValue;
use console::style;
use notify::event::ModifyKind;
use notify::EventKind::Modify;
use notify::{Event, RecursiveMode, Watcher};

pub(crate) use error::Error;

use crate::compiler::{FileCompilerSource, HexoCompiler, HexoCompilerContext};

mod error;
use crate::util::{defer, logger};
use crate::util::logger::{LogLevel};

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

    #[arg(short, long, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}

impl Cli {
    pub(crate) fn run() {
        let build_started = Instant::now();

        let cli = Cli::parse();

        logger::HexoLogger::set_level(cli.log_level);
        let cli_result: Result<_, Error> = match cli.command {
            None => Err(Error::UnknownCommand),
            Some(Commands::Watch { source, output }) => Self::watch(source, output),
            Some(Commands::Build { source, output }) => Self::build(source, output),
        };

        Self::handle_cli_error_if_required(cli_result, build_started);
    }

    fn handle_cli_error_if_required(
        cli_result: Result<(), Error>,
        build_started: Instant,
    ) {
        if let Err(e) = cli_result {
            Self::print_error(e.into());
        } else {
            let build_duration = Instant::now() - build_started;

            logger::output!(
                "{} {:?}",
                style("hexo compilation finished in:").green(),
                build_duration
            );
        }
    }

    fn print_error(error: Box<dyn std::error::Error>) {
        logger::error!("{error}");
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

        logger::debug!("watcher started");

        sleep(Duration::MAX);

        Ok(())
    }

    fn watch_loop(source: String, output: Option<String>, event: Result<Event, notify::Error>) {
        if let Ok(e) = event {
            if let Modify(ModifyKind::Data(_)) = e.kind {
                logger::debug!("rebuilding...");
                let _ = catch_unwind(|| Self::build(source.clone(), output.clone()));
                logger::debug!(" done!");
            }
        } else {
            Self::print_error(event.unwrap_err().into());
        }
    }

    pub(crate) fn build(source: String, output: Option<String>) -> Result<(), Error> {
        defer!(logger::debug!("BUILDING, done"));
        logger::debug!("BUILDING, source: {}, output: {:?}", source, output);

        let context = HexoCompilerContext::new();
        let compiler = HexoCompiler::new(context);

        let source_path = Path::new(&source);
        let compiler_source = FileCompilerSource::new(&source_path);

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

impl ValueEnum for LogLevel {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Debug, Self::Info, Self::Warn, Self::Error, Self::None]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let name = match self {
            LogLevel::Debug => { "debug" }
            LogLevel::Info => { "info" }
            LogLevel::Warn => { "warn" }
            LogLevel::Error => { "error" }
            LogLevel::None => { "none" }
        };

        Some(
            PossibleValue::new(name)
        )
    }
}