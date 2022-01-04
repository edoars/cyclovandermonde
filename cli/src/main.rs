use anyhow::{Context, Result};
use cyclovander::{cond, tr_h};
use indicatif::ProgressBar;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Compute trace instead of condition number
    #[structopt(short, long)]
    trace: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
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

fn get_result(n: u64, trace: bool) -> String {
    if trace {
        tr_h(n).to_string()
    } else {
        cond(n).to_string()
    }
}

fn print_table(reader: BufReader<File>, trace: bool) -> Result<()> {
    let header = if trace { "n\tTr(H_n)" } else { "n\tCond(V_n)" };
    writeln!(io::stdout(), "{}", header)?;

    let _ = reader
        .lines()
        .par_bridge()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<u64>().ok())
        .try_for_each(|n| writeln!(io::stdout(), "{0}\t{1}", n, get_result(n, trace)));

    Ok(())
}

fn print_table_with_spinner(reader: BufReader<File>, trace: bool) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);

    let header = if trace { "n\tTr(H_n)" } else { "n\tCond(V_n)" };
    writeln!(io::stdout(), "{}", header)?;

    let _ = reader
        .lines()
        .par_bridge()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<u64>().ok())
        .try_for_each(|n| {
            pb.set_message(format!("computing {}...", n));
            writeln!(io::stdout(), "{0}\t{1}", n, get_result(n, trace))
        });

    Ok(())
}

fn parse_table(input: PathBuf, trace: bool, mut quiet: bool, threads: usize) -> Result<()> {
    let in_stream = File::open(input).with_context(|| format!("could not read file"))?;
    let reader = BufReader::new(in_stream);

    if atty::is(atty::Stream::Stdout) {
        quiet = true
    }

    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()?;

    match quiet {
        false => print_table_with_spinner(reader, trace),
        true => print_table(reader, trace),
    }
}

fn parse_get(n: u64, trace: bool) -> Result<()> {
    println!("{}", get_result(n, trace));

    Ok(())
}

fn main() -> Result<()> {
    let matches = Opt::from_args();

    match matches.cmd {
        Command::Table {
            input,
            quiet,
            threads,
        } => parse_table(input, matches.trace, quiet, threads),
        Command::Get { n } => parse_get(n, matches.trace),
    }
}
