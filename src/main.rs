use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path or name of the file
    file: Option<String>,

    /// List all the elements with the complete route
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let current_dir = env::current_dir()?;

    if args.list {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("{}", path.display());
        }
        exit(0);
    }

    if let Some(file) = args.file {
        if !file.is_empty() {
            let full_path = PathBuf::from(&current_dir).join(file);
            println!("{}", full_path.display());
        } else {
            println!("{}", current_dir.display());
        }
    } else {
        println!("{}", current_dir.display());
    }

    Ok(())
}
