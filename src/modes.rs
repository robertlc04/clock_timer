
#[derive(Clone, PartialEq)]
pub enum Modes {
    Clock,
    Timer(TimerData)
}

impl Modes {
    pub fn switch_mode(&mut self) {
        *self = match self {
            Modes::Clock => Modes::Timer(TimerData::new()),
            Modes::Timer(_) => Modes::Clock
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TimerData {
    data: u32
} 

impl TimerData {
    pub fn new() -> Self {
        Self {
            data: 0
        }
    }
    pub fn format(value: u32) -> String {
        let hours = value / 60;
        let mins = value - (hours * 60);
        let rt: String;
        if mins < 10 {
            if hours < 10 {
                rt = format!("0{}:0{}", hours, mins);
            } else {
                rt = format!("{}:0{}", hours, mins);
            }
        } else {
            if hours < 10 {
                rt = format!("0{}:{}", hours, mins);
            } else {
                rt = format!("{}:{}", hours, mins);
            }
        }
        rt
    }
}

impl TimerData {
    pub fn increase(&mut self,sum_num: u32) {
        self.data = self.data + sum_num
    }

    pub fn decrease(&mut self,dec_num: u32) {
        if dec_num > self.data {
            self.data = 0
        } else {
            self.data = self.data - dec_num
        }
    }

    pub fn get(&self) -> u32 {
        self.data
    }

    pub fn set(&mut self, num: u32) {
        self.data = num
    }
}

#[derive(PartialEq)]
pub enum TimerStatus {
    Running,
    Stop
}

impl TimerStatus {
    pub fn start() -> TimerStatus {
        TimerStatus::Running
    }
    pub fn stop() -> TimerStatus {
        TimerStatus::Stop
    }
}
