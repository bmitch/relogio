extern crate chrono;
extern crate pancurses;

use chrono::{Datelike, Timelike, Utc, NaiveDate};
use chrono::prelude::*;
use pancurses::*;
use std::time::Duration;
use std::thread::sleep;

const SECONDS_IN_MINUTE: u32 = 60;
const SECONDS_IN_HOUR: u32 = 3600;
const SECONDS_IN_DAY: u32 = 86400;
const SECONDS_IN_YEAR: u32 = 31536000;

struct TimeProgressBar {
    percentage: f64,
    prefix: String
}

struct TimeProgressBarWindow<'a> {
    window: &'a Window,
    progress_bars: Vec<TimeProgressBar>
}

impl TimeProgressBar {
    pub fn new(percentage: f64, prefix: String) -> TimeProgressBar {
        TimeProgressBar { percentage: percentage, prefix: prefix }
    }
}

impl<'a> TimeProgressBarWindow<'a> {

    pub fn new(window: &'a Window, progress_bars: Vec<TimeProgressBar>) -> TimeProgressBarWindow<'a> {
        let new_window = TimeProgressBarWindow { window: &window, progress_bars: progress_bars };
        new_window.window.color_set(3);
        new_window.window.mv(0, 0);
        new_window
    }

    fn draw(&self) {
        for (i, progress_bar) in self.progress_bars.iter().enumerate() {
            self.window.printw(&progress_bar.prefix);
            let width = self.window.get_max_x() - 3 - 15;
            for n in 1..width {
                if (n as f64 / width as f64 * 100.0) < progress_bar.percentage as f64 {
                    self.window.printw("█");
                    continue;
                } 
                self.window.printw("░");
            }
            let formatted_number = format!("{:0>5.*}", 5, format!("{:.*}", 2, progress_bar.percentage));
            self.window.printw(" ");
            self.window.printw(formatted_number.to_string());
            self.window.mv((i + 1) as i32, 0);
        }
        self.window.refresh();
        self.window.clear();
    }
}

fn main() {
    let window = setup_main_window();
    // https://en.wikipedia.org/wiki/Geometric_Shapes
    // https://en.wikipedia.org/wiki/Box_Drawing_(Unicode_block)
    loop {

        let time_progress_window = newwin(6, window.get_max_x() - 3, 2, 1);
        let progress_bars = vec!(
            TimeProgressBar::new(get_percentage_minute_left(), String::from(" M ")),
            TimeProgressBar::new(get_percentage_hour_left(),  String::from(" H ")),
            TimeProgressBar::new(get_percentage_day_left(), String::from(" D ")),
            TimeProgressBar::new(get_percentage_month_left(), String::from(" M ")),
            TimeProgressBar::new(get_percentage_year_left(), String::from(" Y ")),
        );
        let progress_bar_window = TimeProgressBarWindow::new(&time_progress_window, progress_bars);

        window.refresh();
        window.clear();
        progress_bar_window.draw();

        sleep(Duration::from_millis(50));
   }
  endwin();
}

fn setup_main_window() -> Window {
    let window = initscr();
    window.color_set(1);

    if has_colors() {
        start_color();
        init_pair(0, COLOR_BLUE, COLOR_BLUE);
        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        init_pair(2, COLOR_BLUE, COLOR_BLACK);
        init_pair(3, COLOR_WHITE, COLOR_BLACK);
        init_pair(4, COLOR_WHITE, COLOR_BLACK);
    }
    window
}

fn get_percentage_minute_left() -> f64 {
        let date = Local::now();
        let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
        let seconds = date.second() as f64 + milli_seconds;
        seconds / SECONDS_IN_MINUTE as f64 * 100.00
}

fn get_percentage_hour_left() -> f64 {
        let date = Local::now();
        let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
        let seconds = date.second() as f64 + milli_seconds;
        let minutes = (date.minute() as f64 * 60.0) + seconds;

        let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
        minutes / SECONDS_IN_HOUR as f64 * 100.00
}

fn get_percentage_day_left() -> f64 {
    let date = Local::now();
    let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
    let seconds = date.second() as f64 + milli_seconds;
    let minutes = (date.minute() as f64 * 60.0) + seconds;
    let days = (date.hour() as f64 * 3600.0) + minutes + seconds;
    days / SECONDS_IN_DAY as f64 * 100.00
}

fn get_percentage_month_left() -> f64 {
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

fn get_percentage_year_left() -> f64 {
    let now = Utc::now();

    let start_of_year_timestamp = NaiveDate::from_ymd(now.year(), 1, 1).and_hms(0, 0, 0);
    let now_timestamp = now.timestamp();
    let seconds_passed_this_year = now_timestamp - start_of_year_timestamp.timestamp();
    seconds_passed_this_year as f64 / SECONDS_IN_YEAR as f64 * 100.0
}

fn seconds_in_month(year: i32, month: u32) -> u32 {
    // the first day of the next month...
    let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let d = NaiveDate::from_ymd(y, m, 1);

    // ...is preceded by the last day of the original month
    d.pred().day() * SECONDS_IN_DAY
}