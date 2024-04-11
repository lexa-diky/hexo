use crate::cli::run_cli;

mod ast;
mod cli;
mod cst;
mod encoding;
mod render;
mod resolver;

fn main() {
    run_cli()
}
