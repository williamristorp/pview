use std::{
    io::{Read, Write},
    time::Instant,
};

use human_bytes::{format_bytes, format_transfer_rate};

pub mod human_bytes;

#[derive(Debug, Clone)]
pub enum OutputStyle {
    Silent,
    Log,
    Interactive,
}

#[derive(Debug, Clone)]
pub struct PipeViewer {
    total_bytes_processed: u128,
    buffer: Vec<u8>,
    start_time: Instant,
    last_update_time: Instant,
    expected_size: Option<u128>,
    interval: f64,
    output_style: OutputStyle,
}

impl PipeViewer {
    pub fn new(
        buffer_size: usize,
        expected_size: Option<u128>,
        interval: f64,
        output_style: OutputStyle,
    ) -> Self {
        let total_bytes_processed = 0;
        let buffer = vec![0; buffer_size];
        let start_time = Instant::now();
        let last_update_time = start_time;

        Self {
            total_bytes_processed,
            buffer,
            start_time,
            last_update_time,
            expected_size,
            interval,
            output_style,
        }
    }

    pub fn process(&mut self, input: &mut impl Read, output: &mut impl Write) {
        loop {
            let bytes_read = match input.read(&mut self.buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading from STDIN: {e}");
                    break;
                }
            };

            self.total_bytes_processed += bytes_read as u128;

            if self.last_update_time.elapsed().as_secs_f64() > self.interval {
                self.display_progress();
                self.last_update_time = Instant::now();
            }

            match output.write_all(&self.buffer[..bytes_read]) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error writing to STDOUT: {e}");
                    break;
                }
            }
        }
    }

    fn display_progress(&self) {
        match self.output_style {
            OutputStyle::Silent => return,
            OutputStyle::Log => self.display_log(),
            OutputStyle::Interactive => todo!(),
        }
    }

    fn display_log(&self) {
        let elapsed = self.start_time.elapsed();
        let transfer_rate =
            format_transfer_rate(self.total_bytes_processed / elapsed.as_secs() as u128);

        if let Some(size) = self.expected_size {
            eprintln!(
                "TOTAL: {:>9} / {} ({:.2}%), ELAPSED: {:.2}s, RATE: {}",
                format_bytes(self.total_bytes_processed),
                format_bytes(size),
                self.total_bytes_processed as f64 / size as f64 * 100.0,
                elapsed.as_secs_f64(),
                transfer_rate,
            );
        } else {
            eprintln!(
                "TOTAL: {:>9}, ELAPSED: {:.2}s, RATE: {}",
                format_bytes(self.total_bytes_processed),
                elapsed.as_secs_f64(),
                transfer_rate,
            );
        }
    }
}
