
#[cfg(test)]
mod tests {
    use crate::commands::calendar_parser::parse_record;
    use csv::StringRecord;

    #[test]
    fn test_forex_factory_format() {
        // Title, Country, Date, Time, Impact
        let record = StringRecord::from(vec!["Test Event", "USD", "01-15-2025", "2:30pm", "High"]);
        let result = parse_record(&record);
        assert!(result.is_some(), "Forex Factory format failed");
        let (dt, curr, imp, desc, act, fc, prev) = result.expect("Should parse Forex Factory format");
        assert_eq!(dt, "2025-01-15 14:30:00");
        assert_eq!(curr, "USD");
        assert_eq!(imp, "High");
        assert_eq!(desc, "Test Event");
        assert_eq!(act, None);
        assert_eq!(fc, None);
        assert_eq!(prev, None);
    }

    #[test]
    fn test_legacy_format_1() {
        // Date, Time, Currency, Event, Impact
        let record = StringRecord::from(vec!["2025-01-15", "14:30", "EUR", "ECB Rate", "HIGH"]);
        let result = parse_record(&record);
        assert!(result.is_some(), "Legacy format 1 failed");
        let (dt, curr, imp, desc, act, fc, prev) = result.expect("Should parse Legacy format 1");
        assert_eq!(dt, "2025-01-15 14:30:00");
        assert_eq!(curr, "EUR");
        assert_eq!(imp, "HIGH");
        assert_eq!(desc, "ECB Rate");
        assert_eq!(act, None);
        assert_eq!(fc, None);
        assert_eq!(prev, None);
    }

    #[test]
    fn test_legacy_format_with_data() {
        // Date, Time, Currency, Event, Impact, Actual, Forecast, Previous
        let record = StringRecord::from(vec!["2025-01-15", "14:30", "EUR", "ECB Rate", "HIGH", "4.5", "4.2", "4.0"]);
        let result = parse_record(&record);
        assert!(result.is_some(), "Legacy format with data failed");
        let (dt, curr, imp, desc, act, fc, prev) = result.expect("Should parse Legacy format with data");
        assert_eq!(dt, "2025-01-15 14:30:00");
        assert_eq!(curr, "EUR");
        assert_eq!(imp, "HIGH");
        assert_eq!(desc, "ECB Rate");
        assert_eq!(act, Some(4.5));
        assert_eq!(fc, Some(4.2));
        assert_eq!(prev, Some(4.0));
    }

    #[test]
    fn test_slash_date_format() {
        // Date, Time, Currency, Event, Impact (with slashes)
        let record = StringRecord::from(vec!["2025/01/15", "14:30", "EUR", "ECB Rate", "HIGH"]);
        let result = parse_record(&record);
        assert!(result.is_some(), "Slash date format failed");
    }
}
