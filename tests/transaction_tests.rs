use aurumtabula::app::helpers::date::gregorian_to_jalali;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_gregorian_to_jalali() {
        let date = Utc.with_ymd_and_hms(2025, 11, 6, 4, 9, 3).unwrap();

        let jalali = gregorian_to_jalali(date);
        dbg!(&jalali);
        assert_eq!(jalali, "1404-08-15-073903");
    }
}
