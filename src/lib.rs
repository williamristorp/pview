pub mod human_bytes;
pub mod progress_display;

use progress_display::{ProgressDisplay, ProgressDisplayer};

use std::{
    io::{self, Read, Write},
    time::{Duration, Instant},
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
    pub fn remaining_bytes(&self) -> Option<u128> {
        self.expected_size
            .map(|s| s.saturating_sub(self.bytes_processed))
    }

    pub fn progress_percentage(&self) -> Option<f64> {
        self.expected_size
            .map(|s| self.bytes_processed as f64 / s as f64)
    }

    pub fn transfer_rate(&self) -> f64 {
        let elapsed = self.last_display.elapsed().as_secs_f64();
        (self.bytes_processed_since_last_display as f64) / elapsed
    }

    pub fn average_transfer_rate(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        (self.bytes_processed as f64) / elapsed
    }

    pub fn eta(&self) -> Option<Instant> {
        self.remaining_bytes().map(|remaining| {
            if remaining > 0 && self.transfer_rate() > 0.0 {
                let seconds_left = (remaining as f64 / self.transfer_rate()).max(0.0);
                self.last_display + Duration::from_secs_f64(seconds_left)
            } else {
                self.last_display
            }
        })
    }

    pub fn time_remaining(&self) -> Option<Duration> {
        self.eta().map(|eta| eta.duration_since(Instant::now()))
    }
}

#[derive(Debug, Clone)]
pub struct PipeViewer {
    bytes_processed: u128,
    buffer: Vec<u8>,
    start_time: Instant,
    last_display: Instant,
    bytes_processed_since_last_display: u128,
    expected_size: Option<u128>,
    interval: f64,
    progress_displayer: ProgressDisplayer,
}

impl PipeViewer {
    pub fn new(
        buffer_size: usize,
        expected_size: Option<u128>,
        interval: f64,
        progress_displayer: ProgressDisplayer,
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
            progress_displayer,
        }
    }

    pub fn process(&mut self, input: &mut impl Read, output: &mut impl Write) -> io::Result<()> {
        loop {
            let bytes_read = match input.read(&mut self.buffer)? {
                0 => break,
                n => n,
            };

            self.bytes_processed += bytes_read as u128;
            self.bytes_processed_since_last_display += bytes_read as u128;

            if self.last_display.elapsed().as_secs_f64() > self.interval {
                self.display();
                self.last_display = Instant::now();
                self.bytes_processed_since_last_display = 0;
            }

            output.write_all(&self.buffer[..bytes_read])?;
        }

        Ok(())
    }

    pub fn display(&self) {
        let progress_stats = ProgressStats {
            bytes_processed: self.bytes_processed,
            expected_size: self.expected_size,
            start_time: self.start_time,
            last_display: self.last_display,
            bytes_processed_since_last_display: self.bytes_processed_since_last_display,
        };

        self.progress_displayer.display_progress(progress_stats);
    }
}
