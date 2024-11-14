use std::{
    env::current_dir,
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use clap::Parser;
use colored::Colorize;

mod styles;
use styles::STYLES;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Counts lines",
    long_about = None,
    styles = STYLES
)]
struct Args {
    #[arg(short, long, help = "Display more info during execution")]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let cwd = current_dir()?;

    let mut global_line_count: usize = 0;
    for path in fs::read_dir(&cwd)? {
        if let Ok(valid_path) = path {
            let full_path = &cwd.join(valid_path.path());
            if !full_path.is_file() {
                continue;
            }
            if let Ok(file) = File::open(full_path) {
                if args.verbose {
                    println!("[Info] Name: {}", full_path.display());
                }
                let reader = BufReader::new(file);
                global_line_count += reader.lines().count();
            } else if args.verbose {
                println!("[Error] In: {}", full_path.display());
            }
        }
    }
    // let x = cwd.parent().unwrap().display().to_string();
    let dir_name = cwd.file_name().unwrap().to_str().unwrap();
    println!("{dir_name}");

    // println!("In directory {}", cwd.components().last());
    println!("Lines: {}", global_line_count.to_string().magenta());

    Ok(())
}
