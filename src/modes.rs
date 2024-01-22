use crate::prints::clock;
use crate::prints::label;
use std::{cell::Cell, thread::sleep, time::Duration};

// General
#[derive(Clone, Copy)]
pub enum Mode {
    Clock,
    Timer(u32),
    Quit,
}

impl Mode {
    pub fn call(self) -> String {
        match self {
            Mode::Clock => clock(),
            Mode::Timer(t) => {
                let hours = t / 60;
                let mins = t - (hours * 60);
                let rt: String;
                if mins < 10 {
                    if hours < 10 {
                        rt = label(format!("0{}:0{}", hours, mins));
                    } else {
                        rt = label(format!("{}:0{}", hours, mins));
                    }
                } else {
                    if hours < 10 {
                        rt = label(format!("0{}:{}", hours, mins));
                    } else {
                        rt = label(format!("{}:{}", hours, mins));
                    }
                }
                rt
            }
            Mode::Quit => "Quit".to_string(),
        }
    }
}

// Timer
pub enum TimerActions {
    Start,
    Stop,
    IncreaseSeconds,
    IncreaseMinuts,
    DecreaseSeconds,
    DecreaseMinuts,
}

pub fn timer(timer_action: TimerActions, time: &Cell<u32>) {
    match timer_action {
        TimerActions::Start => {
            if time.get() > 0 {
                time.set(time.get() - 1);
                sleep(Duration::from_secs(1));
            }
        }
        TimerActions::IncreaseSeconds => time.set(time.get() + 1),
        TimerActions::IncreaseMinuts => time.set(time.get() + 60),
        TimerActions::DecreaseSeconds => {
            if time.get() > 0 {
                time.set(time.get() - 1)
            }
        },
        TimerActions::DecreaseMinuts => {
            if time.get() > 0 {
                time.set(time.get() - 60)
            }
        },
        _ => {}
    }
}
