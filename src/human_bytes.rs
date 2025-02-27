use std::{num::ParseFloatError, time::Duration};

pub fn format_bytes(bytes: u128) -> String {
    let value = bytes as f64;
    let depth = (value.log2() / 10.0).floor() as usize;

    let unit = match depth {
        0 => return format!("{}B", bytes),
        1 => "KiB",
        2 => "MiB",
        3 => "GiB",
        4 => "TiB",
        5 => "PiB",
        6 => "EiB",
        7 => "ZiB",
        8 => "YiB",
        _ => panic!("Your data is too big."),
    };

    let value = value / 1024.0_f64.powi(depth as i32);
    format!("{:.2}{}", value, unit)
}

pub fn format_transfer_rate(transfer_rate: f64) -> String {
    let bytes = format_bytes(transfer_rate as u128);
    format!("{bytes}/s")
}

pub fn parse_bytes(string: &str) -> Result<u128, ParseFloatError> {
    let digits_end = string
        .find(|c: char| !c.is_ascii_digit() && c != '.')
        .unwrap_or(string.len());

    let value = string[..digits_end].parse::<f64>()?;

    let unit = string[digits_end..].trim_start();
    let scalar = match unit.chars().next().map(|c| c.to_ascii_uppercase()) {
        Some('K') => 1024.0,
        Some('M') => 1024.0 * 1024.0,
        Some('G') => 1024.0 * 1024.0 * 1024.0,
        Some('T') => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        Some('P') => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        Some('E') => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        Some('Z') => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        Some('Y') => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => 1.0,
    };

    let bytes = (value * scalar) as u128;

    Ok(bytes)
}

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs_f64();
    let hours = seconds / 3600.0;
    let minutes = hours / 60.0;
    let seconds = seconds % 60.0;

    format!("{:0>2.0}:{:0>2.0}:{:0>2.0}", hours, minutes, seconds)
}

pub fn format_percentage(percentage: f64) -> String {
    let scaled = percentage * 100.0;
    if scaled < 10.0 {
        format!("{:.2}%", scaled)
    } else {
        format!("{:.1}%", scaled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_100b() {
        let actual = format_bytes(100);
        let expected = "100B";

        assert_eq!(actual, expected);
    }

    #[test]
    fn format_1024b() {
        let actual = format_bytes(1024);
        let expected = "1.00KiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn format_1400b() {
        let actual = format_bytes(1400);
        let expected = "1.37KiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn format_567kib() {
        let actual = format_bytes(567 * 1024);
        let expected = "567.00KiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn format_1024kib() {
        let actual = format_bytes(1024 * 1024);
        let expected = "1.00MiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn format_24mib() {
        let actual = format_bytes(24 * 1024 * 1024);
        let expected = "24.00MiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_100() {
        let actual = parse_bytes("100");
        let expected = Ok(100);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_100b() {
        let actual = parse_bytes("100B");
        let expected = Ok(100);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_1024b() {
        let actual = parse_bytes("1024B");
        let expected = Ok(1024);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_1400b() {
        let actual = parse_bytes("1400B");
        let expected = Ok(1400);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_567kib() {
        let actual = parse_bytes("567KiB");
        let expected = Ok(567 * 1024);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_1024kib() {
        let actual = parse_bytes("1024KiB");
        let expected = Ok(1024 * 1024);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_24mib() {
        let actual = parse_bytes("24MiB");
        let expected = Ok(24 * 1024 * 1024);

        assert_eq!(actual, expected);
    }
}
