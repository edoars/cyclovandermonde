use anyhow::{Context, Result};
use cyclovander::tr_h;
use indicatif::ProgressBar;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Opt {
    /// Generate table from file
    Table {
        /// Input file
        #[structopt(parse(from_os_str))]
        input: PathBuf,

        /// Disable spinner
        #[structopt(short = "q", long = "quiet")]
        quiet: bool,

        /// Number of threads
        #[structopt(short, long, default_value = "1")]
        threads: usize,
    },
    /// Output trace for n
    Get {
        /// Unsigned 64 bits integer
        n: u64,
    },
}

fn print_table(reader: BufReader<File>) -> Result<()> {
    writeln!(io::stdout(), "n\tTr(H_n)")?;

    let _ = reader
        .lines()
        .par_bridge()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<u64>().ok())
        .try_for_each(|n| writeln!(io::stdout(), "{0}\t{1}", n, tr_h(n)));

    Ok(())
}

fn print_table_with_spinner(reader: BufReader<File>) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);

    writeln!(io::stdout(), "n\tTr(H_n)")?;

    let _ = reader
        .lines()
        .par_bridge()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<u64>().ok())
        .try_for_each(|n| {
            pb.set_message(format!("computing {}...", n));
            writeln!(io::stdout(), "{0}\t{1}", n, tr_h(n))
        });

    Ok(())
}

fn parse_table(input: PathBuf, mut quiet: bool, threads: usize) -> Result<()> {
    let in_stream = File::open(input).with_context(|| format!("could not read file"))?;
    let reader = BufReader::new(in_stream);

    if atty::is(atty::Stream::Stdout) {
        quiet = true
    }

    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()?;

    match quiet {
        false => print_table_with_spinner(reader),
        true => print_table(reader),
    }
}

fn parse_get(n: u64) -> Result<()> {
    println!("{}", tr_h(n));

    Ok(())
}

fn main() -> Result<()> {
    let matches = Opt::from_args();

    match matches {
        Opt::Table {
            input,
            quiet,
            threads,
        } => parse_table(input, quiet, threads),
        Opt::Get { n } => parse_get(n),
    }
}
