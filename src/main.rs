use anyhow::{Context, Result};
use clap::Parser;
use env_logger;
use indicatif::{self};
use log::info;
use std::io::{self, Write};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    //Getting Raw Data from the std:input
    // let pattern = std::env::args().nth(0).expect("no pattern given");
    // let path = std::env::args().nth(1).expect("no Path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    info!("Starting!!");
    info!("Choose Std output");
    let stdout = io::stdout();

    let mut handle = io::BufWriter::new(stdout);

    let args = Cli::parse();
    info!("Parsed arguments successfully");

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let no_of_line = content.lines().count() as u64;
    let pb = indicatif::ProgressBar::new(no_of_line as u64);
    let mut progress_count: u64 = 0;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "At line no: {:?} :  {}", [progress_count], line)?;
        }
        pb.inc(1);
        let progress_in_percent = (progress_count * 100) / no_of_line;
        pb.println(format!("[+] Reading File {}%/100", progress_in_percent));
        progress_count += 1;
    }
    pb.finish_with_message("done");

    Ok(())
}
