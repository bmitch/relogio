use pancurses::*;

pub struct TimeProgressBar {
    percentage: f64,
    prefix: String
}

pub struct TimeProgressBarWindow {
    window: Window,
    progress_bars: Vec<TimeProgressBar>
}

impl TimeProgressBar {
    pub fn new(percentage: f64, prefix: String) -> TimeProgressBar {
        TimeProgressBar { percentage: percentage, prefix: prefix }
    }
}

impl TimeProgressBarWindow {

    pub fn new(window: Window, progress_bars: Vec<TimeProgressBar>) -> TimeProgressBarWindow {
        let new_window = TimeProgressBarWindow { window: window, progress_bars: progress_bars };
        new_window.window.color_set(3);
        new_window.window.mv(0, 0);
        new_window
    }

    pub fn draw(&self) {
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