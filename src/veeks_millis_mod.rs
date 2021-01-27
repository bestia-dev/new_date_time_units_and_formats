//! veek_millis_mod

use chrono::{Datelike, NaiveDate};

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
