use crate::ast::{HexoParser, Rule};
use crate::{ast, cst, render};
use clap::{Parser, Subcommand};
use pest::Parser as PestParser;
use std::fs::File;
use std::io::{Read, Write};
use pest::error::Error;
use pest::iterators::Pairs;

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
}

pub(crate) fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        None => {}
        Some(Commands::Build { source, output }) => run_build(source, output),
    }
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
