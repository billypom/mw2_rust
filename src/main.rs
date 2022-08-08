// https://rust-cli.github.io/book/tutorial/impl-draft.html

#![allow(unused)] // what is this???

// i put dependencie in Cargo.toml
use clap::Parser;

// search for a pattern in a file, display the lines that contain pattern
#[derive(Parser)] // what is this???
struct Cli {
    // pattern to look for
    pattern: String,
    // path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    // i print an experiment...
    println!("{}", &args);

    // i print the pattern
    println!("{}", &args.pattern);

    // i print the path
    println!("{}", &args.path.display());
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // println!("Hello, world!");
}
