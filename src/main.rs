use anyhow::{Context, Result};
use indicatif;
use log::{info, warn};
use std::fs::File;
use std::io::{self, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    // #[structopt(parse(from_os_str))]
    path: String,
}

fn main() -> Result<()> {
    env_logger::init();

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path))?;

    info!("file read complete!");
    // let reader = BufReader::new(File::open(&args.path).unwrap());
    grrs::find_matches(&content, &args.pattern, &mut handle);
    Ok(())
}
