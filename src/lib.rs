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
    pub last_update: Instant,
}

#[derive(Debug, Clone)]
pub struct PipeViewer<T: ProgressDisplay> {
    bytes_processed: u128,
    buffer: Vec<u8>,
    start_time: Instant,
    last_update: Instant,
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
        let buffer = vec![0; buffer_size];
        let start_time = Instant::now();
        let last_update = start_time;

        Self {
            bytes_processed,
            buffer,
            start_time,
            last_update,
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

            if self.last_update.elapsed().as_secs_f64() > self.interval {
                self.display();
                self.last_update = Instant::now();
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
            last_update: self.last_update,
        };
        self.progress_display.display_progress(progress_stats);
    }
}
