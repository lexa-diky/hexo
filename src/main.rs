#![feature(test)]


use crate::cli::Cli;

mod cli;
mod compiler;
mod util;

fn main() {
    Cli::run();
}

#[cfg(test)]
mod test {
    #[cfg(test)]
    mod integration {
        use std::env::temp_dir;
        use std::fs::{create_dir_all, File};
        use crate::cli::{Cli};
        use std::io::Read;
        use crate::util::logger::{HexoLogger, LogLevel};

        macro_rules! integration_test_case {
            ($case:ident) => {
                #[test]
                fn $case() {
                    HexoLogger::set_level(LogLevel::None);
                    let case_name = stringify!($case);

                    let input_file_path = format!("samples/{case_name}/input.hexo");
                    let expected_file_path = format!("samples/{case_name}/output.bin").to_string();
                    let actual_file_path = temp_dir().join(format!("samples/{case_name}/output.bin"));
                    create_dir_all(actual_file_path.parent().unwrap()).unwrap();

                    Cli::build(
                        input_file_path,
                        Some(actual_file_path.to_string_lossy().to_string()),
                    ).unwrap();

                    let mut expected_file = File::open(expected_file_path).unwrap();
                    let mut expected_content = vec![];
                    expected_file.read_to_end(&mut expected_content).unwrap();

                    let mut actual_file = File::open(actual_file_path).unwrap();
                    let mut actual_content = vec![];
                    actual_file.read_to_end(&mut actual_content).unwrap();

                    assert_eq!(expected_content, actual_content)
                }
            };
        }

        integration_test_case!(java_object);
        integration_test_case!(len);
        integration_test_case!(pad_left);
        integration_test_case!(pad_right);
    }
}
