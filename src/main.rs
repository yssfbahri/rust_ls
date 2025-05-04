use std::path::Path;
use std::path::PathBuf;
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

    /// sort by time
    #[arg(short,default_value_t=false)]
    t: bool,

    /// reverse order 
    #[arg(short,default_value_t=false)]
    r: bool,

    /// sort by size
    #[arg(short,default_value_t=false)]
    S: bool,


    /// lists author in long format
    #[arg(long,default_value_t=false)]
    author: bool,


    /// Path to list
    #[arg(default_value = ".", value_name = "PATH")]
    path: PathBuf,
}


fn test_function(path:&Path,config:Options,sort_mode:&str){
    //let path = Path::new("/home/yssfbhr/Desktop/test_folder");

    if let Err(e) = ls_utils::ls_(&path,config,sort_mode) {
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
        reverse: false,
    };
    enum SortMode {
        name,
        size,
        time,
    }




    let args = Args::parse();
    
    // sorting options
    let mut sort_mode = "name";
    if args.r {
        conf.reverse = true;
    }
    if args.t {
        sort_mode ="time";
    }
    if args.S {
        sort_mode ="size";
    }

    // printing options
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
    test_function(&args.path,conf,sort_mode);
}
