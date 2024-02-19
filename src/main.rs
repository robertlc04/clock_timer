use std::{char, thread::sleep, time::Duration};

use ncurses::*;
use utils::label;
use clock::clock;

use modes::{Modes,TimerData,TimerStatus};

mod clock;
mod utils;
mod modes;

// Colors
static COLOR_BACKGROUND: i16 = 16;
static COLOR_FOREGROUND: i16 = 17;

static COLOR_PAIR_DEFAULT: i16 = 1;

fn main() {

    init_tui();
    let mut actual_mode = Modes::Clock;
    let mut timer_data = TimerData::new();
    let mut timer_status = TimerStatus::Stop;
    let mut timer_count: f32 = 0.0;
    let mut max_stdscr_x = 0;
    let mut max_stdscr_y = 0;

    getmaxyx(stdscr(),&mut max_stdscr_y,&mut max_stdscr_x);

    let main = newwin(5,40,max_stdscr_y/2-5,max_stdscr_x/2-20);
    // let menu_timer = newwin(5,50,0,0);
    
    refresh();

    loop {
        let ch = getch();
        
        if ch == -1 {
            if actual_mode == Modes::Clock {
                timer_status = TimerStatus::stop();
                timer_data.set(0);
                print_mode(main,None);
            } else {
                if timer_status == TimerStatus::Running && timer_data.get() != 0 {
                    timer_count+= 0.1;
                    let timer_count_str = format!("{:.1}",timer_count).chars().next().unwrap().to_string();
                    if timer_count_str.parse::<i32>().unwrap() == 1  {
                        timer_data.decrease(1);
                        timer_count = 0.0;
                    }
                } else {
                    timer_status = TimerStatus::stop()
                }
                print_mode(main,Some(&timer_data));
            }
            refresh();
            sleep(Duration::from_millis(100));
            continue;
        }
        
        let ch = char::from_u32(ch as u32).unwrap();
        match ch {
            'q' => break,
            '\t' => actual_mode.switch_mode(),
            '\n' => timer_status = TimerStatus::start(),
            ' ' => timer_status = TimerStatus::stop(),
            _ => {}
        }
        if actual_mode != Modes::Clock {
            timer_shortkuts(ch, &mut timer_data)
        }

    }

    // mvwin(menu_timer,2,2);
    // print_mode(main);
    

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

fn print_mode(win: WINDOW, timer_data: Option<&TimerData>) {
    wclear(win);
    if timer_data.is_none() {
        waddstr(win,&label(clock()));
    } else {
        waddstr(win,&label(TimerData::format(timer_data.unwrap().get())));
    }
    wrefresh(win);
}

fn timer_shortkuts(ch: char,timer_data: &mut TimerData) {
    match ch {
        's' => timer_data.increase(1),
        'm' => timer_data.increase(60),
        'r' => timer_data.set(0),
        'S' => timer_data.decrease(1),
        'M' => timer_data.decrease(60),
        _ => {}
    }
}
