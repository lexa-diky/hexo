use crate::ast::{HexoParser, Rule};
use crate::{ast, cst, render};
use clap::{Parser, Subcommand};
use pest::Parser as PestParser;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Compile source and write compiled output on change")]
    Compile {
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
        Some(Commands::Compile { source, output }) => run_compile(source, output),
    }
}

fn run_compile(source: String, output: String) {
    let mut source_buff = String::new();
    File::open(source)
        .unwrap()
        .read_to_string(&mut source_buff)
        .unwrap();

    let pairs = HexoParser::parse(Rule::file, source_buff.as_str()).unwrap();

    let ast = ast::parse_ast(String::from("java_file"), pairs);
    let cst = cst::parse_cst(ast);

    File::create(output)
        .unwrap()
        .write_all(&render::render_cst(cst))
        .unwrap();
}
