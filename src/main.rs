use std::{
    io::{self, Read, Write},
    time::Instant,
};

use clap::Parser;
use pview::human_bytes::{format_bytes, format_transfer_rate};

const BUFFER_SIZE: usize = 65536;

#[derive(Parser)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "SECS",
        help = "Wait SECS seconds between updates (decimals are allowed).",
        default_value = "1.0"
    )]
    interval: f64,
}

fn main() {
    let cli = Cli::parse();

    let start_time = Instant::now();
    let mut last_progress_time = start_time;
    let mut buffer = [0; BUFFER_SIZE];
    let mut total_bytes_read: u128 = 0;

    loop {
        let bytes_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from STDIN: {e}");
                break;
            }
        };

        total_bytes_read += bytes_read as u128;

        let secs_since_last_update = last_progress_time.elapsed().as_secs_f64();

        if secs_since_last_update > cli.interval {
            let transfer_rate =
                format_transfer_rate(total_bytes_read / start_time.elapsed().as_secs() as u128);

            eprintln!(
                "TOTAL: {}, RATE: {}",
                format_bytes(total_bytes_read),
                transfer_rate,
            );

            last_progress_time = Instant::now();
        }

        match io::stdout().write_all(&buffer[..bytes_read]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error writing to STDOUT: {e}");
                break;
            }
        }
    }

    let elapsed = start_time.elapsed();
    let transfer_rate =
        format_transfer_rate(total_bytes_read / start_time.elapsed().as_secs() as u128);

    eprintln!(
        "DONE! TOTAL: {}, ELAPSED: {} s, RATE: {}",
        format_bytes(total_bytes_read),
        elapsed.as_secs_f64(),
        transfer_rate,
    );
}
