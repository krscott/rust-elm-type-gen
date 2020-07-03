mod cli_file_io;

use cli_file_io::{FileOrStdin, FileOrStdout};
use std::{
    error::Error,
    io::{BufRead, Write},
    path::PathBuf,
    process::exit,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, help = "Silence all log messages")]
    quiet: bool,

    #[structopt(short, long, parse(from_occurrences), help = "Increase log output")]
    verbose: usize,

    #[structopt(parse(from_os_str), default_value = "-", help = "Input file")]
    input: PathBuf,

    #[structopt(
        short,
        long,
        parse(from_os_str),
        default_value = "-",
        help = "Output file"
    )]
    output: PathBuf,
}

fn app(opt: Opt) -> Result<(), Box<dyn Error>> {
    let mut input_file = FileOrStdin::from_path(&opt.input)?;
    let mut output_file = FileOrStdout::from_path(&opt.output)?;

    log::info!(
        "Reading '{}', writing '{}'",
        opt.input.to_string_lossy(),
        opt.output.to_string_lossy()
    );

    let reader = input_file.lock();
    let mut writer = output_file.lock();

    for line in reader.lines() {
        writer.write_all(line?.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose + 1)
        .init()
        .unwrap();

    match app(opt) {
        Ok(()) => {}
        Err(e) => {
            log::error!("Program exited: {}", e);
            exit(1);
        }
    }
}
