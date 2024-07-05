use pview::human_bytes::parse_bytes;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
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
    pub expected_size: Option<u128>,

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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputOption {
    Auto,
    Silent,
    Log,
    Interactive,
}
