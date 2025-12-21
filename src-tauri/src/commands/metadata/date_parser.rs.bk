fn is_date_pattern_with_dots(bytes: &[u8]) -> bool {
    if bytes.len() < 21 {
        return false;
    }

    fn is_digit(b: u8) -> bool {
        b.is_ascii_digit()
    }

    is_digit(bytes[0])
        && is_digit(bytes[1])
        && is_digit(bytes[2])
        && is_digit(bytes[3])
        && bytes[4] == b'.'
        && is_digit(bytes[5])
        && is_digit(bytes[6])
        && bytes[7] == b'.'
        && is_digit(bytes[8])
        && is_digit(bytes[9])
        && bytes[10] == b'_'
        && is_digit(bytes[11])
        && is_digit(bytes[12])
        && is_digit(bytes[13])
        && is_digit(bytes[14])
        && bytes[15] == b'.'
        && is_digit(bytes[16])
        && is_digit(bytes[17])
        && bytes[18] == b'.'
        && is_digit(bytes[19])
        && is_digit(bytes[20])
}

fn is_date_pattern_with_hyphens(bytes: &[u8]) -> bool {
    if bytes.len() < 21 {
        return false;
    }

    fn is_digit(b: u8) -> bool {
        b.is_ascii_digit()
    }

    is_digit(bytes[0])
        && is_digit(bytes[1])
        && is_digit(bytes[2])
        && is_digit(bytes[3])
        && bytes[4] == b'-'
        && is_digit(bytes[5])
        && is_digit(bytes[6])
        && bytes[7] == b'-'
        && is_digit(bytes[8])
        && is_digit(bytes[9])
        && bytes[10] == b'-'
        && is_digit(bytes[11])
        && is_digit(bytes[12])
        && is_digit(bytes[13])
        && is_digit(bytes[14])
        && bytes[15] == b'-'
        && is_digit(bytes[16])
        && is_digit(bytes[17])
        && bytes[18] == b'-'
        && is_digit(bytes[19])
        && is_digit(bytes[20])
}

pub fn extract_dates_from_filename(filename: &str) -> (Option<String>, Option<String>) {
    let name_without_ext = filename.trim_end_matches(".csv");
    let bytes = name_without_ext.as_bytes();

    for i in 0..bytes.len().saturating_sub(20) {
        if is_date_pattern_with_dots(&bytes[i..i + 21]) {
            if let Ok(s) = std::str::from_utf8(&bytes[i..i + 21]) {
                let parts: Vec<&str> = s.split('_').collect();
                if parts.len() == 2 {
                    let start_parts: Vec<&str> = parts[0].split('.').collect();
                    let end_parts: Vec<&str> = parts[1].split('.').collect();

                    if start_parts.len() == 3 && end_parts.len() == 3 {
                        let start_date =
                            format!("{}-{}-{}", start_parts[0], start_parts[1], start_parts[2]);
                        let end_date =
                            format!("{}-{}-{}", end_parts[0], end_parts[1], end_parts[2]);
                        return (Some(start_date), Some(end_date));
                    }
                }
            }
        }
    }

    for i in 0..bytes.len().saturating_sub(20) {
        if is_date_pattern_with_hyphens(&bytes[i..i + 21]) {
            if let Ok(s) = std::str::from_utf8(&bytes[i..i + 21]) {
                let start_date = s[0..10].to_string();
                let end_date = s[11..21].to_string();
                return (Some(start_date), Some(end_date));
            }
        }
    }

    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_dates_format_hyphens() {
        let result = extract_dates_from_filename("UNIUSD_M1_2024-01-01-2025-10-30_20251103.csv");
        assert_eq!(
            result,
            (
                Some("2024-01-01".to_string()),
                Some("2025-10-30".to_string())
            )
        );
    }

    #[test]
    fn test_extract_dates_format_dots() {
        let result = extract_dates_from_filename("ADAUSD_1 Min_Bid_2024.01.01_2025.11.06.csv");
        assert_eq!(
            result,
            (
                Some("2024-01-01".to_string()),
                Some("2025-11-06".to_string())
            )
        );
    }
}
