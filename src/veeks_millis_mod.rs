//! veek_millis_mod

use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};

pub fn naive_date_to_veek(nd: NaiveDate) -> String {
    // return
    format!(
        r#"{}c {:02}v {}d"#,
        nd.year(),
        ((nd.ordinal() as f64 - 0.1) / 7.0).floor() as u32 + 1,
        ((nd.ordinal() as f64 - 0.1) % 7.0).round() as u32
    )
}

pub fn veek_to_naive_date(s: &str) -> Option<NaiveDate> {
    // the format is fixed with space after c and v
    use regex::Regex;
    let re = Regex::new(r"^\d{4}c [0-5][0-9]v [1-7]d$").unwrap();
    use std::str::FromStr;
    if re.is_match(s) {
        NaiveDate::from_yo_opt(
            i32::from_str(&s[0..4]).unwrap_or(0),
            (u32::from_str(&s[6..8]).unwrap_or(0) - 1) * 7 + u32::from_str(&s[10..11]).unwrap_or(0),
        )
    } else {
        return None;
    }
}

pub fn naive_time_to_millis(nt: NaiveTime) -> String {
    // return
    format!(
        r#"{}md"#,
        (nt.num_seconds_from_midnight() as f64 / 86.4).round()
    )
}

pub fn millis_from_str_opt(millis: &str) -> Option<f64> {
    // the format is fixed:a decimal number followed by "md" no space
    use regex::Regex;
    let re = Regex::new(r"^\d*(\.\d+)?md$").unwrap();
    use std::str::FromStr;
    if re.is_match(millis) {
        let millis = millis.strip_suffix("md").unwrap();
        let millis = f64::from_str(millis);
        // return
        match millis {
            Ok(millis) => Some(millis),
            Err(_err) => None,
        }
    } else {
        return None;
    }
}

/// rounded to seconds
pub fn millis_to_naive_time(millis: f64) -> Option<NaiveTime> {
    NaiveTime::from_num_seconds_from_midnight_opt((millis * 86.4).round() as u32, 0)
}

/// 24*60*60 = 86_400 seconds per day
/// 1_000_000 μd per day
pub fn seconds_to_micros(seconds: f64) -> f64 {
    seconds / 0.0864
}

pub fn micros_from_str_opt(micros: &str) -> Option<f64> {
    // the string format is fixed:a decimal number followed by "μd". No space.
    use regex::Regex;
    let re = Regex::new(r"^\d*(\.\d+)?μd$").unwrap();
    use std::str::FromStr;
    if re.is_match(micros) {
        let micros = micros.strip_suffix("μd").unwrap();
        let micros = f64::from_str(micros).unwrap_or(0.0);
        return Some(micros);
    } else {
        return None;
    }
}

pub fn micros_to_seconds(micros: f64) -> f64 {
    micros * 0.0864
}
