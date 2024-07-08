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
    fn init_display(&self, progress_stats: ProgressStats) {
        match self {
            ProgressDisplayer::Silent => (),
            ProgressDisplayer::Log(ld) => ld.init_display(progress_stats),
            ProgressDisplayer::Interactive(id) => id.init_display(progress_stats),
        }
    }

    fn display_progress(&self, progress_stats: ProgressStats) {
        match self {
            ProgressDisplayer::Silent => (),
            ProgressDisplayer::Log(ld) => ld.display_progress(progress_stats),
            ProgressDisplayer::Interactive(id) => id.display_progress(progress_stats),
        }
    }

    fn exit_display(&self, progress_stats: ProgressStats) {
        match self {
            ProgressDisplayer::Silent => (),
            ProgressDisplayer::Log(ld) => ld.exit_display(progress_stats),
            ProgressDisplayer::Interactive(id) => id.exit_display(progress_stats),
        }
    }
}

pub trait ProgressDisplay {
    fn init_display(&self, progress_stats: ProgressStats);
    fn display_progress(&self, progress_stats: ProgressStats);
    fn exit_display(&self, progress_stats: ProgressStats);
}

#[derive(Debug, Clone)]
pub struct LogDisplay;

impl ProgressDisplay for LogDisplay {
    fn init_display(&self, _progress_stats: ProgressStats) {}

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

    fn exit_display(&self, _progress_stats: ProgressStats) {}
}

#[derive(Debug, Clone)]
pub struct InteractiveDisplay {
    width: Option<usize>,
}

impl InteractiveDisplay {
    pub fn new(width: Option<usize>) -> Self {
        Self { width }
    }
}

impl InteractiveDisplay {
    fn display(&self, progress_stats: ProgressStats) {
        let term_width = self
            .width
            .unwrap_or(term_size::dimensions_stderr().map(|(x, _)| x).unwrap_or(80));

        if let Some(expected_size) = progress_stats.expected_size {
            let percent = progress_stats.progress_percentage().unwrap();
            let pre_bar = format!(
                "{:>10} @{:<12} ",
                format_bytes(progress_stats.bytes_processed),
                format_transfer_rate(progress_stats.transfer_rate()),
            );
            let post_bar = format!("{}", format_bytes(expected_size));

            let bar_width = term_width - pre_bar.len() - post_bar.len() - 4;
            let num_filled = ((percent * bar_width as f64) as usize).min(bar_width);

            if progress_stats.bytes_processed == 0 {
                eprintln!(
                    "{pre_bar}\u{251D}\u{252D}{}\u{2524} {post_bar}",
                    "\u{254C}".repeat(bar_width.saturating_sub(num_filled + 1)),
                );
            } else if percent < 1.0 {
                eprintln!(
                    "{pre_bar}\u{251D}{}\u{252D}{}\u{2524} {post_bar}",
                    "\u{2501}".repeat(num_filled.saturating_sub(1)),
                    "\u{254C}".repeat(bar_width.saturating_sub(num_filled)),
                );
            } else {
                eprintln!(
                    "{pre_bar}\u{251D}{}\u{2525} {post_bar}",
                    "\u{2501}".repeat(bar_width)
                );
            }

            let sub_bar = format!(
                "{:.1}%",
                progress_stats.progress_percentage().unwrap() * 100.0
            );
            eprintln!(
                "{}{sub_bar}",
                " ".repeat(pre_bar.len() + num_filled - sub_bar.len() / 2)
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

            eprintln!();

            eprintln!(
                "{}",
                format_transfer_rate(progress_stats.average_transfer_rate()),
            );
        }
    }
}

impl ProgressDisplay for InteractiveDisplay {
    fn init_display(&self, progress_stats: ProgressStats) {
        self.display(progress_stats);
    }

    fn display_progress(&self, progress_stats: ProgressStats) {
        eprint!("\x1B[1A\x1B[2K");
        eprint!("\x1B[1A\x1B[2K");
        eprint!("\x1B[1A\x1B[2K");

        self.display(progress_stats);
    }

    fn exit_display(&self, progress_stats: ProgressStats) {
        self.display_progress(progress_stats);
    }
}
