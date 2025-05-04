use std::path::Path;
use std::process;

mod ls_utils;
use crate::ls_utils::Options;

const  CONFIG : Options = Options{
    show_hidden : false,
    long_format : false,
};

fn main() {
    let path = Path::new(".");

    if let Err(e) = ls_utils::ls_(path,CONFIG) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    process::exit(0);
}
