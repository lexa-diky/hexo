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

    #[arg(short, long, default_value_t = false)]
    safe: bool,
}

#[derive(Copy, Clone, Debug)]
struct CliCompilerArguments {
    safe_mode: bool,
}

impl Cli {
    pub(crate) fn run() {
        let build_started = Instant::now();

        let cli = Cli::parse();

        logger::HexoLogger::set_level(&cli.log_level);

        cli.log_debug_interface_arguments();

        let compiler_arguments = cli.cli_compiler_arguments();
        let cli_result: Result<_, Error> = match cli.command {
            None => Err(Error::UnknownCommand),
            Some(Commands::Watch { source, output }) => Self::watch(source, output, compiler_arguments),
            Some(Commands::Build { source, output }) => Self::build(source, output, compiler_arguments),
        };

        Self::handle_cli_error_if_required(cli_result, build_started);
    }

    fn cli_compiler_arguments(&self) -> CliCompilerArguments {
        CliCompilerArguments {
            safe_mode: self.safe
        }
    }

    fn log_debug_interface_arguments(&self) {
        logger::debug!("initialized cli interface with arguments:\
            \n  --log-level = {}\
            \n  --safe = {}",
            &self.log_level,
            &self.safe
        );
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

    fn watch(source: String, output: Option<String>, compiler_arguments: CliCompilerArguments) -> Result<(), Error> {
        let source_path_clone = source.clone();
        let source_path = source_path_clone.as_ref();

        let mut watcher = notify::recommended_watcher(move |event: Result<Event, _>| {
            Self::watch_loop(source.clone(), output.clone(), compiler_arguments, event)
        })
            .map_err(Error::FileWatcher)?;

        watcher
            .watch(source_path, RecursiveMode::NonRecursive)
            .map_err(Error::FileWatcher)?;

        logger::debug!("watcher started");

        sleep(Duration::MAX);

        Ok(())
    }

    fn watch_loop(
        source: String,
        output: Option<String>,
        compiler_arguments: CliCompilerArguments,
        event: Result<Event, notify::Error>,
    ) {
        match event {
            Ok(e) => {
                if let Modify(ModifyKind::Data(_)) = e.kind {
                    logger::debug!("rebuilding...");
                    let _ = catch_unwind(|| Self::build(source.clone(), output.clone(), compiler_arguments));
                    logger::debug!(" done!");
                }
            }
            Err(e) => {
                Self::print_error(e.into());
            }
        }
    }

    pub(crate) fn build(
        source: String,
        output: Option<String>,
        compiler_arguments: CliCompilerArguments
    ) -> Result<(), Error> {
        defer!(logger::debug!("BUILDING, done"));
        logger::debug!("BUILDING, source: {}, output: {:?}", source, output);

        let context = HexoCompilerContext::new(
            compiler_arguments.safe_mode
        );
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