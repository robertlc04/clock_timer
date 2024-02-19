use chrono::*;

pub fn clock() -> String {
    let actual = Local::now().time().to_string();
    let actual = actual.split_at(5).0;
    actual.to_string()
}

