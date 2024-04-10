use std::any::Any;
use std::io::{Read, Write};

use crate::cli::run_cli;

mod ast;
mod encoding;
mod cst;
mod render;
mod cli;

fn main() {
    run_cli()
}

