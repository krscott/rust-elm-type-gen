use polymorphio::FileOrStdout;
use rust_elm_types::*;
use std::{error::Error, io::Write, path::PathBuf, process::exit};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, help = "Silence all log messages")]
    quiet: bool,

    #[structopt(short, long, parse(from_occurrences), help = "Increase log output")]
    verbose: usize,

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
    let mut output_file = FileOrStdout::from_path(&opt.output)?;

    let mut writer = output_file.lock();

    let t = test_data_spec();

    writer.write_all(serde_yaml::to_string(&t).unwrap().as_bytes())?;
    writer.write_all(b"\n")?;

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

fn test_data_spec() -> ApiSpec {
    ApiSpec {
        module: "test_types".into(),
        types: vec![TypeSpec::Struct {
            name: "TestStruct".into(),
            fields: vec![],
        }],
    }
}
