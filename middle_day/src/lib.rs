use chrono::{ NaiveDate, Datelike };

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        None
    } else {
        Some(NaiveDate::from_yo_opt(year.try_into().unwrap(), 183).map(|date| date.weekday())?)
    }
}
