use chrono::{Datelike, Timelike, Utc, NaiveDate};
use chrono::prelude::*;
use reqwest::get;
use serde::{Deserialize};
use serde::de::{Visitor, Deserializer, Error};
use std::fmt;

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

fn get_moon_data(dt: Date<Utc>) -> MoonData {
    let url = dt.format("https://api.usno.navy.mil/moon/phase?date=%m/%d/%Y&nump=1")
        .to_string();
    get(&url).unwrap().json().unwrap()
}

#[derive(Deserialize, Debug)]
struct MoonData {
    error: bool,
    apiversion: String,
    year: i32,
    month: u32,
    day: u32,
    numphases: usize,
    datechanged: bool,
    phasedata: Vec<PhaseDate>,
}

#[derive(Deserialize, Debug)]
struct PhaseDate {
    phase: MoonPhase,
    date: String,
    time: String,
}

#[derive(Debug, PartialEq)]
enum MoonPhase {
    New,
    FirstQuarter,
    Full,
    LastQuarter,
}

impl<'de> Deserialize<'de> for MoonPhase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>,
    {
        struct MoonPhaseVisitor;

        impl<'de> Visitor<'de> for MoonPhaseVisitor {

            type Value = MoonPhase;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter,"One of `New Moon`, `First Quarter`, `Full Moon`, or `Last Quarter`")
            }

            fn visit_str<E>(self, s: &str) -> Result<MoonPhase, E>
                where E: Error,
            {
                println!("{}", s);
                match s {
                    "New Moon" => Ok(MoonPhase::New),
                    "First Quarter" => Ok(MoonPhase::FirstQuarter),
                    "Full Moon" => Ok(MoonPhase::Full),
                    "Last Quarter" => Ok(MoonPhase::LastQuarter),
                    _ => Err(Error::unknown_variant(s, VARIANTS)),
                }
            }
        }

        const VARIANTS: &'static [&'static str] = &[
            "New Moon",
            "First Quarter",
            "Full Moon",
            "Last Quarter",
        ];

        deserializer.deserialize_str(MoonPhaseVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // this is a really slow test and will break without internet
    fn test_retrieve_moon_data() {
        // march 2019 supermoon
        let md = get_moon_data(Utc.ymd(2019, 03, 17));
        assert_eq!(md.phasedata[0].phase, MoonPhase::Full);
    }
}
