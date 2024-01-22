use std::cell::Cell;
use std::{sync::mpsc, thread::sleep, time::Duration};

use modes::*;
use ncurses::{getch, *};
use prints::print_actual_time;

mod modes;
mod prints;

// Colors
static COLOR_BACKGROUND: i16 = 16;
static COLOR_FOREGROUND: i16 = 17;

static COLOR_PAIR_DEFAULT: i16 = 1;


fn main() {
    init_tui();


    let (sender, receiver) = mpsc::channel::<Option<Mode>>();
    let time_thread = std::thread::spawn(|| {
        print_actual_time(receiver);
    });

    // let actual: Mode = Action::Clock;
    let mut actual = Mode::Clock;
    let _ = sender.send(Some(actual));
    let mut timer_status = TimerActions::Stop;
    let data_timer: Cell<u32> = Cell::new(0);

    bkgd(COLOR_PAIR(COLOR_PAIR_DEFAULT));


    loop {
        let ch = getch();

        if ch == -1 {
            match actual {
                Mode::Clock => {
                    sleep(Duration::from_millis(100));
                    continue;
                }
                Mode::Timer(_) => match timer_status {
                    TimerActions::Start => {
                        if data_timer.get() == 0 {
                            timer_status = TimerActions::Stop;
                            actual = Mode::Timer(data_timer.get());
                            let _ = sender.send(Some(actual));
                            continue;
                        }
                        timer(TimerActions::Start, &data_timer);
                        actual = Mode::Timer(data_timer.get());
                        let _ = sender.send(Some(actual));
                        continue;
                    }
                    _ => {
                        sleep(Duration::from_millis(100));
                        continue;
                    }
                },
                _ => {}
            }
        }

        match char::from_u32(ch as u32).unwrap() {
            // Select Mode
            'c' => match actual {
                Mode::Timer(_) => {
                    data_timer.set(0);
                    actual = Mode::Clock;
                }
                _ => {}
            },
            't' => match actual {
                Mode::Clock => {
                    actual = Mode::Timer(0);
                    timer_status = TimerActions::Stop;
                }
                _ => {}
            },
            // Timer Keybinds
            's' => match actual {
                Mode::Timer(_) => {
                    timer(TimerActions::IncreaseSeconds, &data_timer);
                    actual = Mode::Timer(data_timer.get());
                }
                _ => {}
            },
            'm' => match actual {
                Mode::Timer(_) => {
                    timer(TimerActions::IncreaseMinuts, &data_timer);
                    actual = Mode::Timer(data_timer.get());
                }
                _ => {}
            },
            'S' => match actual {
                Mode::Timer(_) => {
                    timer(TimerActions::DecreaseSeconds, &data_timer);
                    actual = Mode::Timer(data_timer.get());
                }
                _ => {}
            },
            'M' => match actual {
                Mode::Timer(_) => {
                    timer(TimerActions::DecreaseMinuts, &data_timer);
                    actual = Mode::Timer(data_timer.get());
                }
                _ => {}
            },
            'p' => match actual {
                Mode::Timer(_) => {
                    timer(TimerActions::Stop, &data_timer);
                    actual = Mode::Timer(data_timer.get());
                    timer_status = TimerActions::Stop;
                }
                _ => {}
            },
            '\n' => match actual {
                Mode::Timer(_) => {
                    timer(TimerActions::Start, &data_timer);
                    actual = Mode::Timer(data_timer.get());
                    timer_status = TimerActions::Start;
                }
                _ => {}
            },
            // Quit Keybind
            'q' => match actual {
                _ => {
                    let _ = sender.send(Some(Mode::Quit));
                    break;
                }
            },
            _ => {}
        }

        let _ = sender.send(Some(actual));
    }

    time_thread.join().unwrap();

    endwin();
}

fn init_tui() {
    initscr();
    keypad(stdscr(), true);
    timeout(0);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    init_color(COLOR_BACKGROUND, 0, 43 * 4, 54 * 4);
    init_color(COLOR_FOREGROUND, 142 * 4, 161 * 4, 161 * 4);
    init_pair(COLOR_PAIR_DEFAULT, COLOR_FOREGROUND, COLOR_BACKGROUND);
}
