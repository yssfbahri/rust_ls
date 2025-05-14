use std::env;
use std::path::{Path, PathBuf};
use std::process;

mod ls_utils;
use crate::ls_utils::Options;

fn parse_args() -> (Options, String, PathBuf) {
    let mut conf = Options {
        all: false,
        long_format: false,
        author: false,
        reverse: false,
        human: false,
    };

    let mut sort_mode = "name".to_string();
    let mut path = PathBuf::from(".");

    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--author" => conf.author = true,
                "--help" => {
                    print_help();
                    process::exit(0);
                }
                _ => {
                    eprintln!("Unknown long option: {}", arg);
                    process::exit(1);
                }
            }
        } else if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => conf.all = true,
                    'l' => conf.long_format = true,
                    't' => sort_mode = "time".to_string(),
                    's' => sort_mode = "size".to_string(),
                    'r' => conf.reverse = true,
                    'h' => conf.human = true,
                    _ => {
                        eprintln!("Unknown short option: -{}", ch);
                        process::exit(1);
                    }
                }
            }
        } else {
            // Assume any non-flag arg is the path
            path = PathBuf::from(arg);
        }
    }

    (conf, sort_mode, path)
}

fn print_help() {
    println!(
        "Usage: ls_clone [OPTIONS] [PATH]
        
Options:
  -a            Do not ignore entries starting with '.'
  -l            Use a long listing format
  -t            Sort by time
  -s            Sort by size
  -r            Reverse order
  -h            Human-readable sizes
  --author      Show author in long format
  --help        Show this help message
  PATH          Directory to list (default: .)"
    );
}

fn call_ls_utils(path: &Path, config: Options, sort_mode: &str) {
    if let Err(e) = ls_utils::ls_(path, config, sort_mode) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn main() {
    let (conf, sort_mode, path) = parse_args();
    call_ls_utils(&path, conf, &sort_mode);
}
