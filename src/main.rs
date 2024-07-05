use std::{
    io::{self, Read, Write},
    time::Instant,
};

use clap::Parser;
use pview::human_bytes::{format_bytes, format_transfer_rate, parse_bytes};

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
        short = 's',
        long,
        value_name = "SIZE",
        help = "Expect the size of the data to be SIZE bytes (supports binary (base 1024) units such as `K`, `M` or `G`)",
        value_parser = parse_bytes,
    )]
    expected_size: Option<u128>,

    #[arg(
        short,
        long,
        value_name = "SIZE",
        help = "Use a buffer size of SIZE bytes (supports binary (base 1024) units such as `K`, `M` or `G`)",
        default_value = "64K",
        value_parser = parse_bytes,
    )]
    buffer_size: u128,
}

fn main() {
    let cli = Cli::parse();

    let start_time = Instant::now();
    let mut last_progress_time = start_time;
    let mut buffer = vec![0; cli.buffer_size as usize];
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

        if last_progress_time.elapsed().as_secs_f64() > cli.interval {
            let elapsed = start_time.elapsed();
            let transfer_rate = format_transfer_rate(total_bytes_read / elapsed.as_secs() as u128);

            if let Some(size) = cli.expected_size {
                eprintln!(
                    "TOTAL: {:>9} / {} ({:.2}%), ELAPSED: {:.2}s, RATE: {}",
                    format_bytes(total_bytes_read),
                    format_bytes(size),
                    total_bytes_read as f64 / size as f64 * 100.0,
                    elapsed.as_secs_f64(),
                    transfer_rate,
                );
            } else {
                eprintln!(
                    "TOTAL: {:>9}, ELAPSED: {:.2}s, RATE: {}",
                    format_bytes(total_bytes_read),
                    elapsed.as_secs_f64(),
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
    let transfer_rate = format_transfer_rate(total_bytes_read / elapsed.as_secs() as u128);

    eprintln!(
        "DONE! TOTAL: {:>9}, ELAPSED: {}s, RATE: {}",
        format_bytes(total_bytes_read),
        elapsed.as_secs_f64(),
        transfer_rate,
    );
}
