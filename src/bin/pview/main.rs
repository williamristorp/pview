mod cli;

use std::io::{self, IsTerminal};

use clap::Parser;
use cli::{Cli, OutputOption};
use pview::{OutputStyle, PipeViewer};

fn main() {
    let cli = Cli::parse();

    let output_style = match cli.output {
        OutputOption::Auto => {
            if io::stderr().is_terminal() {
                OutputStyle::Interactive
            } else {
                OutputStyle::Silent
            }
        }
        OutputOption::Silent => OutputStyle::Silent,
        OutputOption::Log => OutputStyle::Log,
        OutputOption::Interactive => OutputStyle::Interactive,
    };

    let mut pipe_viewer = PipeViewer::new(
        cli.buffer_size as usize,
        cli.expected_size,
        cli.interval,
        output_style,
    );
    pipe_viewer.process(&mut io::stdin(), &mut io::stdout());
}
