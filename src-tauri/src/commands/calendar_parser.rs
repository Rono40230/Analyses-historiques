pub fn parse_record(record: &csv::StringRecord) -> Option<(String, String, String, String)> {
    // 1. Format Forex Factory: Title(0), Country(1), Date(2), Time(3), Impact(4)
    // Ex: "Title", "USD", "12-22-2025", "1:00am", "Low"
    if record.len() >= 5 && record[2].contains('-') && (record[3].contains(':') || record[3].to_lowercase().contains("day")) {
        let title = record[0].trim();
        let currency = record[1].trim();
        let date_str = record[2].trim();
        let time_str = record[3].trim();
        let impact = record[4].trim();

        // Parse Date: MM-DD-YYYY -> YYYY-MM-DD
        let date_parts: Vec<&str> = date_str.split('-').collect();
        if date_parts.len() != 3 { return None; }
        let (month, day, year) = (date_parts[0], date_parts[1], date_parts[2]);

        // Parse Time: HH:MMam/pm -> HH:MM:00
        let (hour, minute) = if time_str.to_lowercase().contains("day") {
            (0, 0) // All Day events -> 00:00
        } else {
            let lower_time = time_str.to_lowercase();
            let is_pm = lower_time.contains("pm");
            let is_am = lower_time.contains("am");
            
            let clean_time = lower_time.replace("am", "").replace("pm", "");
            let time_parts: Vec<&str> = clean_time.split(':').collect();
            
            if time_parts.len() >= 2 {
                let h: u32 = time_parts[0].parse().unwrap_or(0);
                let m: u32 = time_parts[1].parse().unwrap_or(0);
                
                let h_24 = if is_pm && h != 12 {
                    h + 12
                } else if is_am && h == 12 {
                    0
                } else {
                    h
                };
                (h_24, m)
            } else {
                (0, 0)
            }
        };

        let dt = format!("{}-{:0>2}-{:0>2} {:0>2}:{:0>2}:00", year, month, day, hour, minute);
        return Some((dt, currency.to_string(), impact.to_string(), title.to_string()));
    }

    // 2. Format Legacy 1: Date(0) contains '-' (YYYY-MM-DD or MM-DD-YYYY)
    if record[0].contains('-') {
        if record.len() < 5 {
            return None;
        }
        let date = record[0].trim();
        let time = record[1].trim();
        let currency = record[2].trim();
        let event = record[3].trim();
        let impact = record[4].trim();

        let date_parts: Vec<&str> = date.split('-').collect();
        let time_parts: Vec<&str> = time.split(':').collect();

        if date_parts.len() != 3 || time_parts.len() < 2 {
            return None;
        }

        // Détection du format de date (MM-DD-YYYY vs YYYY-MM-DD)
        let (year, month, day) = if date_parts[2].trim().len() == 4 {
            (date_parts[2], date_parts[0], date_parts[1])
        } else {
            (date_parts[0], date_parts[1], date_parts[2])
        };

        let dt = format!(
            "{}-{:0>2}-{:0>2} {:0>2}:{:0>2}:00",
            year.trim(),
            month.trim(),
            day.trim(),
            time_parts[0].trim(),
            time_parts[1].trim()
        );

        Some((
            dt,
            currency.to_string(),
            impact.to_string(),
            event.to_string(),
        ))
    } else {
        // 3. Format Legacy 2: Split columns (Year, Month, Day...)
        if record.len() < 8 {
            return None;
        }

        let year = record[0].trim();
        let month = record[1].trim();
        let day = record[2].trim();
        let hour = record[3].trim();
        let minute = record[4].trim();
        let symbol = record[5].trim();
        let impact = record[6].trim();
        let description = record[7].trim();

        // Basic validation to ensure these are actually numbers
        if year.len() != 4 || !year.chars().all(char::is_numeric) {
            return None;
        }

        let dt = format!(
            "{}-{:0>2}-{:0>2} {:0>2}:{:0>2}:00",
            year, month, day, hour, minute
        );
        Some((
            dt,
            symbol.to_string(),
            impact.to_string(),
            description.to_string(),
        ))
    }
}
