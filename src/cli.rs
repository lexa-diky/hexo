use std::fs::File;
use std::io::{Read, Write};
use std::panic::catch_unwind;
use std::thread::sleep;
use std::time::Duration;

use clap::{Parser, Subcommand};
use notify::{Event, RecursiveMode, Watcher};
use notify::event::ModifyKind;
use notify::EventKind::Modify;
use pest::Parser as PestParser;

use crate::{ast, cst, render};
use crate::ast::{HexoParser, Rule};

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

pub(crate) fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        None => {}
        Some(Commands::Watch { source, output }) => run_watch(source, output),
        Some(Commands::Build { source, output }) => run_build(source, output),
    }
}

fn run_watch(source: String, output: String) {
    let source_path_clone = source.clone();
    let source_path =  source_path_clone.as_ref();

    let mut watcher = notify::recommended_watcher(move |event: Result<Event, _>| {
        match event {
            Ok(e) => {
                if let Modify(ModifyKind::Data(_)) = e.kind {
                    print!("rebuilding...");
                    let _ = catch_unwind(|| {
                        run_build(source.clone(), output.clone())
                    });
                    println!(" done!");
                }

            }
            Err(e) => { println!("watch error: {:?}", e) }
        }
    }).expect("Can't create watcher");

    watcher.watch(source_path, RecursiveMode::NonRecursive)
        .expect("Can't start watcher");

    println!("watcher started");

    sleep(Duration::MAX);
}

fn run_build(source: String, output: String) {
    let mut source_buff = String::new();
    File::open(source)
        .expect("Can't find source file")
        .read_to_string(&mut source_buff)
        .expect("Failed to read source file");

    let pairs = match HexoParser::parse(Rule::file, source_buff.as_str()) {
        Ok(pairs) => pairs,
        Err(err) => panic!("Can't parse source file\n{}", err)
    };


    let ast = ast::parse_ast(String::from("java_file"), pairs);
    let cst = cst::parse_cst(ast);

    File::create(output)
        .unwrap()
        .write_all(&render::render_cst(cst))
        .unwrap();
}
