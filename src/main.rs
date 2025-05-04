use std::path::Path;
use std::process;

mod ls_utils;
use crate::ls_utils::Options;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// do not ignore entries starting with '.'
    #[arg(short,long,default_value_t=false)]
    all: bool,
    /// use a long listing format
    #[arg(short,default_value_t=false)]
    l: bool,

    /// lists author in long format
    #[arg(long,default_value_t=false)]
    author: bool,
}


fn test_function(config:Options){
    let path = Path::new("/home/yssfbhr/Desktop/test_folder");

    if let Err(e) = ls_utils::ls_(path,config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    process::exit(0);
}
fn main() {

    let mut conf : Options = Options {
        all: false,
        long_format: false,
        author: false,
    };

    let args = Args::parse();
    if args.all{
        conf.all = true;
        println!("all {}",args.all)
    }
    if args.l {
        conf.long_format = true;
        println!("long {}",args.l)
    }
    if args.author {
        conf.author = true;
        println!("author {}",args.author);
    }    
    test_function(conf);
}
