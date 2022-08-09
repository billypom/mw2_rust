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

// baby's first rust
fn main1() {
    let args = Cli::parse();
    // i print the pattern
    println!("input string: {}", &args.pattern);
    // i print the path
    println!("input file path: {}", &args.path.display());
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // println!("Hello, world!");
    // // i print an experiment...
    // let mut crap = 1;
    // crap += 1;
    // println!("{}", crap);
}

// read file. if line contains my search string, print the line
fn main2 () {
    let args = Cli::parse();
    // open the file
    // content = standard library :: filestream :: built in method called read_to_string or something
    let my_content = std::fs::read_to_string(&args.path).expect("could not read file");
    // .except is ugly and bad
    for line in my_content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    // try my own implementation
    // let f = File::open(&args.path)?;
    // let mut reader = std::io::BufReader::new(f);
    // let mut line = String::new();
    // if reader.read_line(&mut line).contains(&args.pattern) {
    //     println!("{}", line)
    // }
    // didnt work lol. not reading documentation thanks tho
}

// Error handling 2
fn main3() {
    let args = Cli::parse();
    // A function like read_to_string doesn't return a string. Instead - it resuts a RESULT that contains either a String or an error of some type.
    // In this case -> std::io::Error
    // So how do we know which it is?
    // - Since RESULT is an enum, you can use 'match' to check which variant it is
    // all "arms" of a match block need to return something of the same type.
    // here is an example of how to do that - abort the program on error
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("uh oh: {}", error); }
    }; // We can also use the shortcut method that creates a match block for us, below example:
    let content = std::fs::read_to_string("test.txt").unwrap();
    println!("file content: {}", content);

    // Alternatively, we could use return instead of panic
    // let result_2 = std::fs::read_to_string("test.txt");
    // let content_2 = match result_2 {
        // Ok(content) => { content },
        // Err(error) => { return Err(error.into()); }
    // };

    // However, this changes the return type our function needs... see below
}

fn main4() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    }; // Similar to the panic match block, we can use the ? to create an error match block
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}


// nicer error handling
#[derive(Debug)]
struct CustomError(String);
fn main5() -> Result<(), CustomError> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    println!("file content: {}", content);
    Ok(())
}

// The problem with this one is that we don't store the original error. only the string representation.
// use the anyhow library, use its Context trait to add a description, and keep the original error
// add 'anyhow ="1.0"' to [dependencies] in Cargo.toml
// use anyhow::{Context, Result};
// fn main() -> Result<()> {
//     let path = "test.txt";
//     let content = std::fs::read_to_string(path)
//         .with_context(|| format!("could not read file `{}`", path))?;
//     println!("file content: {}", content);
//     Ok(())
// anyhow doesnt fuckin work. yay 