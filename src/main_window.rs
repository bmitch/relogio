use crate::time_calculations;

use chrono::prelude::*;
use pancurses::*;

pub fn setup_main_window() -> Window {
    let window = initscr();

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

pub fn draw_frame(window: &Window) {
    let date = Local::now();
    let top_border = date.format("%H:%M:%S - %A %B %d, %Y").to_string();
    let border = format!("{: ^1$}", top_border.to_string(), window.get_max_x() as usize);

    window.color_set(1);
    window.printw(&border);
    window.mv(1, 0);
    window.color_set(2);

    draw_side_borders(&window, 7);     
    window.mv(8, 0);

    draw_bottom_status(&window);

    window.refresh();
    window.clear();
}

fn draw_side_borders(window: &Window, number_of_times: u8) {
    for _x in 0..number_of_times {
        window.printw("┃");
        window.printw(" ".repeat((window.get_max_x() -2) as usize));
        window.printw("┃");
    }
}

fn draw_bottom_status(window: &Window) {
    let day_of_year = time_calculations::get_current_day_of_year();
    let year = time_calculations::get_current_year();
    let status_bar_data = format!(" It is day {} of 365 - {} days remaining in {}", day_of_year, 365-day_of_year, year);
    let status_bar = format!("{: ^1$}", status_bar_data.to_string(), window.get_max_x() as usize);
    window.color_set(1);
    window.printw(&status_bar);
}