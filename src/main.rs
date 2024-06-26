use crate::cli::{Cli, Error};
use std::env::temp_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod cli;
mod compiler;

fn main() {
    Cli::run();
}

#[test]
fn run_sample() -> Result<(), Error> {
    Cli::build("sample.hexo".to_string(), None)
}

// list files in directory test cases
// for each file, run the test
#[test]
fn run_test_cases() {
    fn read_file(filename: &PathBuf) -> Vec<u8> {
        let a = hexo_io::byte_buffer::ByteBuffer::new();

        let mut f = File::open(filename).expect("no file found");
        let metadata = std::fs::metadata(filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }

    let test_cases_dir = std::fs::read_dir("test_cases").unwrap();
    for entry in test_cases_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        let input_file = path.join("input.hexo");
        let output_file = temp_dir().join(&path).join("output.bin");
        let expected_output = path.join("output.bin");

        std::fs::create_dir_all(temp_dir().join(&path)).unwrap();

        Cli::build(
            input_file.to_str().unwrap().to_string(),
            Some(output_file.to_str().unwrap().to_string()),
        )
        .unwrap();

        let output_buf = read_file(&output_file);
        let expected_output_buf = read_file(&expected_output);

        assert_eq!(output_buf, expected_output_buf);
    }
}
