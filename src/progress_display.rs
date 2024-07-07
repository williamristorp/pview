use crate::{
    human_bytes::{format_bytes, format_duration, format_transfer_rate},
    ProgressStats,
};

#[derive(Debug, Clone)]
pub enum ProgressDisplayer {
    Silent,
    Log(LogDisplay),
    Interactive(InteractiveDisplay),
}

impl ProgressDisplay for ProgressDisplayer {
    fn display_progress(&self, progress_stats: ProgressStats) {
        match self {
            ProgressDisplayer::Silent => (),
            ProgressDisplayer::Log(ld) => ld.display_progress(progress_stats),
            ProgressDisplayer::Interactive(id) => id.display_progress(progress_stats),
        }
    }
}

pub trait ProgressDisplay {
    fn display_progress(&self, progress_stats: ProgressStats);
}

#[derive(Debug, Clone)]
pub struct LogDisplay;

impl ProgressDisplay for LogDisplay {
    fn display_progress(&self, progress_stats: ProgressStats) {
        let elapsed = progress_stats.start_time.elapsed();
        let transfer_rate = format_transfer_rate(progress_stats.transfer_rate());

        if let Some(size) = progress_stats.expected_size {
            eprintln!(
                "TOTAL: {:>10} / {} ({:.2}%), ELAPSED: {:.2}s, RATE: {}",
                format_bytes(progress_stats.bytes_processed),
                format_bytes(size),
                progress_stats.bytes_processed as f64 / size as f64 * 100.0,
                elapsed.as_secs_f64(),
                transfer_rate,
            );
        } else {
            eprintln!(
                "TOTAL: {:>10}, ELAPSED: {:.2}s, RATE: {}",
                format_bytes(progress_stats.bytes_processed),
                elapsed.as_secs_f64(),
                transfer_rate,
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct InteractiveDisplay;

impl ProgressDisplay for InteractiveDisplay {
    fn display_progress(&self, progress_stats: ProgressStats) {
        eprint!("\x1B[1A\x1B[2K");
        eprint!("\x1B[1A\x1B[2K");

        let term_width = term_size::dimensions_stderr().map(|(x, _)| x).unwrap_or(80);

        if let Some(size) = progress_stats.expected_size {
            let percent = progress_stats.bytes_processed as f64 / size as f64;
            let stats = format!(
                "{} @{} ({:.2}%)",
                format_bytes(progress_stats.bytes_processed),
                format_transfer_rate(progress_stats.transfer_rate()),
                progress_stats.progress_percentage().unwrap()
            );

            let bar_width = term_width - stats.len() - 3;
            let num_filled = ((percent * bar_width as f64) as usize).min(bar_width);
            eprintln!(
                "{stats} [{}{}]",
                "=".repeat(num_filled),
                " ".repeat(bar_width.saturating_sub(num_filled)),
            );

            eprintln!(
                "{} ETA {}",
                format_transfer_rate(progress_stats.average_transfer_rate()),
                format_duration(progress_stats.time_remaining().unwrap()),
            );
        } else {
            eprintln!(
                "{} @{}",
                format_bytes(progress_stats.bytes_processed),
                format_transfer_rate(progress_stats.transfer_rate()),
            );

            eprintln!(
                "{}",
                format_transfer_rate(progress_stats.average_transfer_rate()),
            );
        }
    }
}
