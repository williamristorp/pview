use pview::{
    human_bytes::parse_bytes,
    progress_display::{InteractiveDisplay, LogDisplay, ProgressDisplayer},
    PipeViewer,
};

use std::{
    fs,
    io::{self, IsTerminal},
    path::PathBuf,
};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[arg(
        value_name = "FILE",
        help = "Copy each FILE to standard output in sequence. Use `-` for standard input.",
        default_value = "-"
    )]
    pub files: Vec<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "SECS",
        help = "Wait SECS seconds between updates.",
        default_value = "1.0"
    )]
    pub interval: f64,

    #[arg(
        short = 's',
        long,
        value_name = "SIZE",
        help = "Expect the size of the data to be SIZE bytes (supports binary (base 1024) units such as `K`, `M` or `G`).",
        value_parser = parse_bytes,
    )]
    pub size: Option<u128>,

    #[arg(
        short,
        long,
        value_name = "SIZE",
        help = "Use a buffer size of SIZE bytes (supports binary (base 1024) units such as `K`, `M` or `G`).",
        default_value = "64K",
        value_parser = parse_bytes,
    )]
    pub buffer_size: u128,

    #[arg(
        short,
        long,
        value_name = "OUTPUT",
        help = "Set output format (`auto`, `silent`, `log` or `interactive`). `auto` is `interactive` if standard error is a TTY, otherwise it is `quiet`.",
        default_value = "auto"
    )]
    pub output: OutputOption,

    #[arg(
        short,
        long,
        value_name = "WIDTH",
        help = "Assume terminal is WIDTH characters wide. If unspecified, `pview` will try to determine the width of the terminal or fallback to 80."
    )]
    pub width: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputOption {
    Auto,
    Silent,
    Log,
    Interactive,
}

fn main() {
    let cli = Cli::parse();

    let progress_displayer = match cli.output {
        OutputOption::Interactive | OutputOption::Auto if io::stderr().is_terminal() => {
            ProgressDisplayer::Interactive(InteractiveDisplay::new(cli.width))
        }
        OutputOption::Log => ProgressDisplayer::Log(LogDisplay {}),
        _ => ProgressDisplayer::Silent,
    };

    let expected_size = match cli.size.unwrap_or_else(|| {
        cli.files
            .iter()
            .filter(|f| f.to_str() != Some("-"))
            .filter_map(|f| fs::metadata(f).ok())
            .filter(|md| md.is_file())
            .map(|md| md.len() as u128)
            .sum()
    }) {
        0 => None,
        n => Some(n),
    };

    for file in &cli.files {
        if file.to_str() == Some("-") {
            continue;
        }
    }

    let mut pipe_viewer = PipeViewer::new(
        cli.buffer_size as usize,
        expected_size,
        cli.interval,
        progress_displayer,
    );

    pipe_viewer.init_display();

    for file in &cli.files {
        if file.to_str() == Some("-") {
            pipe_viewer
                .process(&mut io::stdin(), &mut io::stdout())
                .unwrap();
        } else {
            let mut input_file = fs::File::open(file).unwrap();
            pipe_viewer
                .process(&mut input_file, &mut io::stdout())
                .unwrap();
        }
    }

    pipe_viewer.exit_display();
}
