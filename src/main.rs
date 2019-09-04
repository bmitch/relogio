extern crate chrono;
extern crate pancurses;

mod time_calculations;
mod progress_bars;
mod main_window;
use pancurses::*;
use std::time::Duration;
use std::thread::sleep;

// https://en.wikipedia.org/wiki/Geometric_Shapes
// https://en.wikipedia.org/wiki/Box_Drawing_(Unicode_block)
fn main() {
    let window = main_window::setup_main_window();
    loop {
        let time_progress_window = newwin(6, window.get_max_x() - 3, 2, 1);
        let progress_bars = vec!(
            progress_bars::TimeProgressBar::new(time_calculations::get_percentage_minute_left(), String::from(" M ")),
            progress_bars::TimeProgressBar::new(time_calculations::get_percentage_hour_left(),  String::from(" H ")),
            progress_bars::TimeProgressBar::new(time_calculations::get_percentage_day_left(), String::from(" D ")),
            progress_bars::TimeProgressBar::new(time_calculations::get_percentage_month_left(), String::from(" M ")),
            progress_bars::TimeProgressBar::new(time_calculations::get_percentage_year_left(), String::from(" Y ")),
        );
        let progress_bar_window = progress_bars::TimeProgressBarWindow::new(time_progress_window, progress_bars);

        main_window::draw_frame(&window);
        progress_bar_window.draw();
        sleep(Duration::from_millis(75));
   }
  endwin();
}