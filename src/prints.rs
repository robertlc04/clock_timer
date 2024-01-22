use chrono::prelude::*;
use figlet_rs::FIGfont;
use std::{sync::mpsc, thread, time::Duration};

use crate::modes::Mode;
use ncurses::*;

// Printers

pub fn clock() -> String {
    let actual = Local::now().time().to_string();
    let actual = actual.split_at(5).0;
    label(actual.to_string())
}

pub fn label(source: String) -> String {

    let font = FIGfont::standard().unwrap();
    let text = font.convert(&source).unwrap().to_string();
    text
}

pub fn print_actual_time(sender: mpsc::Receiver<Option<Mode>>) {
    let mut actual: Option<Mode> = sender.try_recv().unwrap();
    let mut maxx: i32 = 0;
    let mut maxy: i32 = 0;

    loop {
        let value: Option<Mode> = match sender.try_recv() {
            Ok(v) => v,
            Err(_) => None,
        };

        match actual.unwrap() {
            Mode::Quit => {
                break;
            }
            _ => {}
        }

        getmaxyx(stdscr(), &mut maxy, &mut maxx);

        if value.is_some() {
            actual = value;
        }

        clear();
        for (i, l) in actual.unwrap().call().lines().enumerate() {
            attr_on(A_BOLD());
            mvprintw(maxy / 2 - 3 + i as i32, maxx / 2 - 22 as i32, &l);
            attr_off(A_BOLD());
        }
        refresh();

        thread::sleep(Duration::from_millis(10));
    }
}
