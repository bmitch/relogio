use chrono::{Datelike, Timelike, Utc, NaiveDate};
use chrono::prelude::*;

const SECONDS_IN_MINUTE: u32 = 60;
const SECONDS_IN_HOUR: u32 = 3600;
const SECONDS_IN_DAY: u32 = 86400;
const SECONDS_IN_YEAR: u32 = 31536000;

pub fn get_percentage_minute_left() -> f64 {
        let date = Local::now();
        let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
        let seconds = date.second() as f64 + milli_seconds;
        seconds / SECONDS_IN_MINUTE as f64 * 100.00
}

pub fn get_percentage_hour_left() -> f64 {
        let date = Local::now();
        let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
        let seconds = date.second() as f64 + milli_seconds;
        let minutes = (date.minute() as f64 * 60.0) + seconds;

        minutes / SECONDS_IN_HOUR as f64 * 100.00
}

pub fn get_percentage_day_left() -> f64 {
    let date = Local::now();
    let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
    let seconds = date.second() as f64 + milli_seconds;
    let minutes = (date.minute() as f64 * 60.0) + seconds;
    let days = (date.hour() as f64 * 3600.0) + minutes + seconds;
    days / SECONDS_IN_DAY as f64 * 100.00
}

pub fn get_percentage_month_left() -> f64 {
    let date = Local::now();
    let now = Utc::now();
    let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
    let seconds = date.second() as f64 + milli_seconds;
    let minutes = (date.minute() as f64 * 60.0) + seconds;
    let days = (date.hour() as f64 * 3600.0) + minutes + seconds;
    let seconds_in_current_month = seconds_in_month(now.year(), now.month());
    let seconds_elapsed_in_current_month = (date.day() as f64 * 86400.0) + days + minutes + seconds;

    seconds_elapsed_in_current_month / seconds_in_current_month as f64 * 100.00
}

pub fn get_percentage_year_left() -> f64 {
    let now = Utc::now();

    let start_of_year_timestamp = NaiveDate::from_ymd(now.year(), 1, 1).and_hms(0, 0, 0);
    let now_timestamp = now.timestamp();
    let seconds_passed_this_year = now_timestamp - start_of_year_timestamp.timestamp();
    seconds_passed_this_year as f64 / SECONDS_IN_YEAR as f64 * 100.0
}

pub fn seconds_in_month(year: i32, month: u32) -> u32 {
    // the first day of the next month...
    let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let d = NaiveDate::from_ymd(y, m, 1);

    // ...is preceded by the last day of the original month
    d.pred().day() * SECONDS_IN_DAY
}

pub fn get_current_day_of_year() -> u32 {
    let month_length_days = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let today_date: DateTime<Utc> = Utc::now();
    let this_month = today_date.month()-1;
    let this_day = today_date.day();
    let mut month_ticker: usize = 0;
    let mut day_ticker = 0;
    while month_ticker < this_month as usize {
        day_ticker = day_ticker + month_length_days[month_ticker];
        month_ticker+=1;
    }
    day_ticker = day_ticker + this_day;
    day_ticker
}

pub fn get_current_year() -> i32 {
    Utc::now().year()
}
