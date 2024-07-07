pub mod human_bytes;
pub mod progress_display;

use progress_display::ProgressDisplay;

use std::{
    io::{Read, Write},
    time::Instant,
};

#[derive(Debug, Clone)]
pub struct ProgressStats {
    pub bytes_processed: u128,
    pub expected_size: Option<u128>,
    pub start_time: Instant,
    pub last_display: Instant,
    pub bytes_processed_since_last_display: u128,
}

impl ProgressStats {
    pub fn transfer_rate(&self) -> u128 {
        let elapsed = self.last_display.elapsed().as_secs_f64();
        ((self.bytes_processed_since_last_display as f64) / elapsed) as u128
    }

    pub fn average_transfer_rate(&self) -> u128 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        ((self.bytes_processed as f64) / elapsed) as u128
    }
}

#[derive(Debug, Clone)]
pub struct PipeViewer<T: ProgressDisplay> {
    bytes_processed: u128,
    buffer: Vec<u8>,
    start_time: Instant,
    last_display: Instant,
    bytes_processed_since_last_display: u128,
    expected_size: Option<u128>,
    interval: f64,
    progress_display: T,
}

impl<T: ProgressDisplay> PipeViewer<T> {
    pub fn new(
        buffer_size: usize,
        expected_size: Option<u128>,
        interval: f64,
        progress_display: T,
    ) -> Self {
        let bytes_processed = 0;
        let bytes_processed_since_last_display = 0;
        let buffer = vec![0; buffer_size];
        let start_time = Instant::now();
        let last_display = start_time;

        Self {
            bytes_processed,
            buffer,
            start_time,
            last_display,
            bytes_processed_since_last_display,
            expected_size,
            interval,
            progress_display,
        }
    }

    pub fn process(&mut self, input: &mut impl Read, output: &mut impl Write) {
        self.display();

        loop {
            let bytes_read = match input.read(&mut self.buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading from STDIN: {e}");
                    break;
                }
            };

            self.bytes_processed += bytes_read as u128;
            self.bytes_processed_since_last_display += bytes_read as u128;

            if self.last_display.elapsed().as_secs_f64() > self.interval {
                self.display();
                self.last_display = Instant::now();
                self.bytes_processed_since_last_display = 0;
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

    fn display(&self) {
        let progress_stats = ProgressStats {
            bytes_processed: self.bytes_processed,
            expected_size: self.expected_size,
            start_time: self.start_time,
            last_display: self.last_display,
            bytes_processed_since_last_display: self.bytes_processed_since_last_display,
        };

        self.progress_display.display_progress(progress_stats);
    }
}
