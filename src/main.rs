use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
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
    let reader = file_or_stdin_reader(&opt.input)?;
    let mut writer = file_or_stdout_writer(&opt.output)?;

    log::info!(
        "Reading '{}', writing '{}'",
        opt.input.to_string_lossy(),
        opt.output.to_string_lossy()
    );

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

fn file_or_stdin_reader(path: &PathBuf) -> io::Result<Box<dyn BufRead>> {
    let reader = if path.to_string_lossy() == "-" {
        Box::new(BufReader::new(io::stdin())) as Box<dyn BufRead>
    } else {
        Box::new(BufReader::new(File::open(path)?)) as Box<dyn BufRead>
    };

    Ok(reader)
}

fn file_or_stdout_writer(path: &PathBuf) -> io::Result<BufWriter<Box<dyn Write>>> {
    let writer = if path.to_string_lossy() == "-" {
        Box::new(io::stdout()) as Box<dyn Write>
    } else {
        Box::new(File::create(path)?) as Box<dyn Write>
    };

    Ok(BufWriter::new(writer))
}
