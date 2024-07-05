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
        return format!("{} B", bytes);
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

    format!("{:.2} {}", value, unit)
}

pub fn format_transfer_rate(bytes: u128) -> String {
    let bytes = format_bytes(bytes);
    format!("{bytes}/s")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b100() {
        let actual = format_bytes(100);
        let expected = "100 B";

        assert_eq!(actual, expected);
    }

    #[test]
    fn b1024() {
        let actual = format_bytes(1024);
        let expected = "1.00 KiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn b1400() {
        let actual = format_bytes(1400);
        let expected = "1.37 KiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn kib567() {
        let actual = format_bytes(567 * 1024);
        let expected = "567.00 KiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn kib1024() {
        let actual = format_bytes(1024 * 1024);
        let expected = "1.00 MiB";

        assert_eq!(actual, expected);
    }

    #[test]
    fn mib24() {
        let actual = format_bytes(24 * 1024 * 1024);
        let expected = "24.00 MiB";

        assert_eq!(actual, expected);
    }
}
