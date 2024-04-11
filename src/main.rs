use std::any::Any;
use std::io::{Read, Write};

use crate::cli::run_cli;

mod ast;
mod cli;
mod cst;
mod encoding;
mod render;
mod resovler;

fn main() {
    run_cli()
}
