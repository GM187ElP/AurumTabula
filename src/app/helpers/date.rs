use chrono::{DateTime, Utc,  Datelike, Timelike};
use chrono_persian::ToPersian;

pub fn gregorian_to_jalali(date: DateTime<Utc>) -> String {
    let naive = date.naive_utc();
    let persian=naive.to_persian().unwrap();
    format!("{:04}-{:02}-{:02}-{:02}{:02}{:02}",
            persian.year(), persian.month(), persian.day(),
            persian.hour(),persian.minute(),persian.second())
}

