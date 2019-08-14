use std::time::{Duration, Instant};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Progress {
    value: u64,
    max: u64, // Max of zero means just to display the count.

    last_line_time: Option<Instant>,
    last_line_len: usize,
}

impl Progress {
    pub fn new(max: u64) -> Progress {
        Progress {
            max,
            value: 0,
            last_line_len: 0,
            last_line_time: None,
        }
    }

    pub fn inc(&mut self) {
        self.value += 1;
        self.check_update();
    }

    pub fn set(&mut self, val: u64) {
        self.value = val;
        self.check_update();
    }

    pub fn check_update(&mut self) {
        if self.last_line_time.is_none() {
            self.update()
        } else {
            self.last_line_time.map(|last| {
                if last.elapsed() > Duration::from_millis(500) {
                    self.update();
                    io::stderr().flush().unwrap();
                }
            });
        }
    }

    pub fn force_update(&mut self) {
        self.update();
    }

    fn update(&mut self) {
        self.last_line_time = Some(Instant::now());
        let line = format!("{}/{}", self.value, self.max);
        self.last_line_len = line.len();
        eprint!("\r{}", line)
    }

    pub fn finish_and_clear(&mut self) {
        if self.last_line_time.is_some() {
            let spaces = " ".repeat(self.last_line_len);
            eprint!("\r{}\r", spaces);
        }
    }
}
