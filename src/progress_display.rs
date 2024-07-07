use crate::{
    human_bytes::{format_bytes, format_transfer_rate},
    ProgressStats,
};

pub trait ProgressDisplay {
    fn display_progress(&self, progress_stats: ProgressStats);
}

pub struct SilentDisplay;

impl ProgressDisplay for SilentDisplay {
    fn display_progress(&self, _progress_stats: ProgressStats) {}
}

pub struct LogDisplay;

impl ProgressDisplay for LogDisplay {
    fn display_progress(&self, progress_stats: ProgressStats) {
        let elapsed = progress_stats.start_time.elapsed();
        let transfer_rate = format_transfer_rate(
            progress_stats
                .bytes_processed
                .checked_div(elapsed.as_secs() as u128)
                .unwrap_or(0),
        );

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

pub struct InteractiveDisplay;

impl ProgressDisplay for InteractiveDisplay {
    fn display_progress(&self, progress_stats: ProgressStats) {
        eprint!("\x1B[1A\x1B[2K");
        eprint!("\x1B[1A\x1B[2K");

        let term_width = term_size::dimensions_stderr().map(|(x, _)| x).unwrap_or(80);
        let elapsed = progress_stats.start_time.elapsed();
        let transfer_rate = format_transfer_rate(
            progress_stats
                .bytes_processed
                .checked_div(elapsed.as_secs() as u128)
                .unwrap_or(0),
        );

        if let Some(size) = progress_stats.expected_size {
            let percent = progress_stats.bytes_processed as f64 / size as f64;
            let bar_width = term_width - 47;
            let num_filled = ((percent * bar_width as f64) as usize).min(bar_width);
            eprintln!(
                "{:>10} / {} ({:6.2}%) [{}{}] {}",
                format_bytes(progress_stats.bytes_processed),
                format_bytes(size),
                (percent * 100.0).min(100.0),
                "=".repeat(num_filled),
                " ".repeat(bar_width.saturating_sub(num_filled)),
                transfer_rate,
            );
        } else {
            eprintln!(
                "{:>10} {:<10}",
                format_bytes(progress_stats.bytes_processed),
                transfer_rate
            );
        }

        eprintln!("{:>12}", transfer_rate);
    }
}
