use std::{
    io::{self, Read, Write},
    process::exit,
    time::Instant,
};

use clap::Parser;
use pview::human_bytes::{format_bytes, format_transfer_rate, parse_bytes};

const BUFFER_SIZE: usize = 65536;

#[derive(Parser)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "SECS",
        help = "Wait SECS seconds between updates",
        default_value = "1.0"
    )]
    interval: f64,

    #[arg(
        short,
        long,
        value_name = "SIZE",
        help = "Assume the size of the input data will be SIZE bytes (supports binary (base 1024) units such as `K`, `M` or `G`)"
    )]
    size: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let expected_size = cli.size.and_then(|s| parse_bytes(&s));
    if let Some(0) = expected_size {
        eprintln!("Size of input data cannot be 0.");
        exit(1);
    }

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

            if let Some(size) = expected_size {
                eprintln!(
                    "TOTAL: {:>9} / {} ({:.2}%), RATE: {}",
                    format_bytes(total_bytes_read),
                    format_bytes(size),
                    total_bytes_read as f64 / size as f64 * 100.0,
                    transfer_rate,
                );
            } else {
                eprintln!(
                    "TOTAL: {:>9}, RATE: {}",
                    format_bytes(total_bytes_read),
                    transfer_rate,
                );
            }

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
        "DONE! TOTAL: {:>9}, ELAPSED: {} s, RATE: {}",
        format_bytes(total_bytes_read),
        elapsed.as_secs_f64(),
        transfer_rate,
    );
}
