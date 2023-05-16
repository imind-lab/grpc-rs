use chrono::{Local, TimeZone};

pub fn get_timestamp() -> i64 {
    Local::now().timestamp_millis()
}

pub fn fmt_timestamp(ts: i64, fmt: &str) -> String {
    let datetime = Local.timestamp_millis_opt(ts).unwrap();
    format!("{}", datetime.format(fmt))
}
