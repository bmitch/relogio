extern crate chrono;
extern crate pancurses;

use chrono::Local;
use chrono::{Datelike, Timelike, Utc, NaiveDate};
use chrono::prelude::*;
use pancurses::*;
use std::time::Duration;
use std::thread::sleep;

const SECONDS_IN_MINUTE: u32 = 60;
const SECONDS_IN_HOUR: u32 = 3600;
const SECONDS_IN_DAY: u32 = 86400;
const SECONDS_IN_YEAR: u32 = 31536000;

fn main() {
    let window = initscr();
    let max_x: i32 = window.get_max_x();
    let time_progress_window = newwin(6, max_x - 3, 2, 1);

    if has_colors() {
        start_color();
        init_pair(0, COLOR_BLUE, COLOR_BLUE);
        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        init_pair(2, COLOR_BLUE, COLOR_BLACK);
        init_pair(3, COLOR_WHITE, COLOR_BLACK);
        init_pair(4, COLOR_WHITE, COLOR_BLACK);
    }

    // https://en.wikipedia.org/wiki/Geometric_Shapes
    // https://en.wikipedia.org/wiki/Box_Drawing_(Unicode_block)
    
    loop {
        let date = Local::now();
        let now = Utc::now();

        window.color_set(1);

        let top_border = format!("{}", date.format("%H:%M:%S - %A %B %d, %Y").to_string());
        let horizontal_border = "━".repeat((max_x -2) as usize);
        let top_border = date.format("%H:%M:%S - %A %B %d, %Y").to_string();
        let border = format!("{: ^1$}", top_border.to_string(), max_x as usize);

        let horizontal_border = "━".repeat((max_x -2) as usize);
        window.printw(&border);
        window.mv(1, 0);
        window.color_set(2);

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        time_progress_window.color_set(3);

        // Start days left
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
        let status_bar_data = format!(" It is day {} of 365 - {} days remaining in {}", day_ticker, 365-day_ticker, today_date.year());
        let status_bar = format!("{: ^1$}", status_bar_data.to_string(), max_x as usize);
        window.color_set(1);
        window.printw(&status_bar);

        // Start Minutes
        time_progress_window.mv(0, 0);
        time_progress_window.printw(" M ");
        let progress_width = time_progress_window.get_max_x() - 15;
        let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
        let seconds = date.second() as f64 + milli_seconds;

        let minute_progress_percentage_complete : f64 = seconds / SECONDS_IN_MINUTE as f64 * 100.00;
        for n in 1..progress_width {
            if (n as f64 / progress_width as f64 * 100.0) < minute_progress_percentage_complete as f64 {
                time_progress_window.printw("█");
            } else {
                time_progress_window.printw("░");
            }
        }
        let r1_format = format!("{:0>10}", minute_progress_percentage_complete);
        let formatted_number = format!("{:.*}", 2, r1_format);
        time_progress_window.printw(" ");
        time_progress_window.printw(formatted_number.to_string());

        // Start Hours
        time_progress_window.mv(1, 0);
        time_progress_window.printw(" H ");
        let progress_width = time_progress_window.get_max_x() - 15;
        let minutes = (date.minute() as f64 * 60.0) + seconds;

        let hour_progress_percentage_complete = minutes / SECONDS_IN_HOUR as f64 * 100.00;
        for n in 1..progress_width {
            if (n as f64 / progress_width as f64 * 100.0) < hour_progress_percentage_complete as f64 {
                time_progress_window.printw("█");
            } else {
                time_progress_window.printw("░");
            }
        }
        let formatted_number = format!("{:.*}", 2, hour_progress_percentage_complete);
        time_progress_window.printw(" ");
        time_progress_window.printw(formatted_number.to_string());


        // Start Days
        time_progress_window.mv(2, 0);
        time_progress_window.printw(" D ");
        let progress_width = time_progress_window.get_max_x() - 15;
        let days = (date.hour() as f64 * 3600.0) + minutes + seconds;

        let day_progress_percentage_complete = days / SECONDS_IN_DAY as f64 * 100.00;
        for n in 1..progress_width {
            if (n as f64 / progress_width as f64 * 100.0) < day_progress_percentage_complete as f64 {
                time_progress_window.printw("█");
            } else {
                time_progress_window.printw("░");
            }
        }
        let formatted_number = format!("{:.*}", 2, day_progress_percentage_complete);
        time_progress_window.printw(" ");
        time_progress_window.printw(formatted_number.to_string());

        // Start Months
        time_progress_window.mv(3, 0);
        time_progress_window.printw(" M ");

        let progress_width = time_progress_window.get_max_x() - 15;
        let seconds_in_current_month = seconds_in_month(now.year(), now.month());
        let seconds_elapsed_in_current_month = (date.day() as f64 * 86400.0) + days + minutes + seconds;

        let month_progress_percentage_complete = seconds_elapsed_in_current_month / seconds_in_current_month as f64 * 100.00;
        for n in 1..progress_width {
            if (n as f64 / progress_width as f64 * 100.0) < month_progress_percentage_complete as f64 {
                time_progress_window.printw("█");
            } else {
                time_progress_window.printw("░");
            }
        }
        let formatted_number = format!("{:.*}", 2, month_progress_percentage_complete);
        time_progress_window.printw(" ");
        time_progress_window.printw(formatted_number.to_string());

        // Start Year
        time_progress_window.mv(4, 0);
        time_progress_window.printw(" Y ");

        let progress_width = time_progress_window.get_max_x() - 15;
        let days = (date.hour() as f64 * 3600.0) + minutes + seconds;
        let seconds_elapsed_in_current_year = (day_ticker as f64 * SECONDS_IN_DAY as f64) + days;


        let year_progress_percentage_complete = seconds_elapsed_in_current_year / SECONDS_IN_YEAR as f64 * 100.00;
        for n in 1..progress_width {
            if (n as f64 / progress_width as f64 * 100.0) < year_progress_percentage_complete as f64 {
                time_progress_window.printw("█");
            } else {
                time_progress_window.printw("░");
            }
        }
        let formatted_number = format!("{:.*}", 2, year_progress_percentage_complete);
        time_progress_window.printw(" ");
        time_progress_window.printw(formatted_number.to_string());


        window.refresh();
        window.clear();
        time_progress_window.refresh();
        time_progress_window.clear();

        sleep(Duration::from_millis(50));
   }
  endwin();
}

fn seconds_in_month(year: i32, month: u32) -> u32 {
    // the first day of the next month...
    let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let d = NaiveDate::from_ymd(y, m, 1);

    // ...is preceded by the last day of the original month
    d.pred().day() * SECONDS_IN_DAY
}