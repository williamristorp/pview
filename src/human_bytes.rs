use std::num::ParseIntError;

pub fn format_bytes(bytes: u128) -> String {
    let mut depth = 0;
    let mut value = bytes as f64;

    loop {
        if value / 1024.0 >= 1.0 {
            depth += 1;
            value /= 1024.0;
        } else {
            break;
        }
    }

    if depth == 0 {
        return format!("{}B", bytes);
    }

    let unit = match depth {
        0 => unreachable!(),
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

    format!("{:.2}{}", value, unit)
}

pub fn format_transfer_rate(transfer_rate: f64) -> String {
    let bytes = format_bytes(transfer_rate as u128);
    format!("{bytes}/s")
}

pub fn parse_bytes(string: &str) -> Result<u128, ParseIntError> {
    let digits_end = string
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(string.len());

    let value = string[..digits_end].parse::<u128>()?;

    let unit = string[digits_end..].trim_start();
    let scalar = match unit.chars().next().map(|c| c.to_ascii_uppercase()) {
        Some('K') => 1024,
        Some('M') => 1024 * 1024,
        Some('G') => 1024 * 1024 * 1024,
        Some('T') => 1024 * 1024 * 1024 * 1024,
        Some('P') => 1024 * 1024 * 1024 * 1024 * 1024,
        Some('E') => 1024 * 1024 * 1024 * 1024 * 1024 * 1024,
        Some('Z') => 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024,
        Some('Y') => 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024,
        _ => 1,
    };

    Ok(value * scalar)
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
