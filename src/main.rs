extern crate chrono;
extern crate pancurses;

use chrono::Local;
use chrono::Timelike;
use pancurses::*;
use std::time::Duration;
use std::thread::sleep;

const SECONDS_IN_MINUTE: u32 = 60;

struct TimeProgressBar {
    width: i32,
    percentage: f64,
    prefix: String
}

struct TimeProgressBarWindow<'a> {
    window: &'a Window,
    progress_bar: TimeProgressBar
}

impl<'a> TimeProgressBarWindow<'a> {

    pub fn new(window: &'a Window, progress_bar: TimeProgressBar) -> TimeProgressBarWindow<'a> {
        let new_window = TimeProgressBarWindow { window: &window, progress_bar: progress_bar };
        new_window.window.color_set(3);
        new_window.window.mv(0, 0);
        new_window
    }

    fn draw(&self) {
        self.window.printw(&self.progress_bar.prefix);
        for n in 1..self.progress_bar.width {
            if (n as f64 / self.progress_bar.width as f64 * 100.0) < self.progress_bar.percentage as f64 {
                self.window.printw("█");
                continue;
            } 
            self.window.printw("░");
        }
        let formatted_number = format!("{:0>5.*}", 5, format!("{:.*}", 2, self.progress_bar.percentage));
        self.window.printw(" ");
        self.window.printw(formatted_number.to_string());
        self.window.refresh();
        self.window.clear();
    }
}

fn main() {
    let window = setup_main_window();
    // https://en.wikipedia.org/wiki/Geometric_Shapes
    // https://en.wikipedia.org/wiki/Box_Drawing_(Unicode_block)
    loop {

        // Start Minutes
        let minutes_bar = TimeProgressBar { 
            width: window.get_max_x() - 3 - 15,
            percentage: get_percentage_minute_left(),
            prefix: String::from(" M ")
        };




        let time_progress_window = newwin(6, window.get_max_x() - 3, 2, 1);
        let progress_bar_window = TimeProgressBarWindow::new(&time_progress_window, minutes_bar);



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