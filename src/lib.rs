// region: lmake_md_to_doc_comments include README.md A //!

// endregion: lmake_md_to_doc_comments include README.md A //!

// use unwrap::unwrap;
use chrono::{Datelike, NaiveDate};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

mod fullweek_millis_mod;
mod web_sys_mod;

use web_sys_mod::*;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    debug_write(&format!(
        "new_date_time_units_and_formats v{}",
        env!("CARGO_PKG_VERSION")
    ));
    set_event_handlers();
    // init values
    cnv_on_click("cnv_yf");
    div_today_on_click();

    // return
    Ok(())
}

/// write  to screen
pub fn write_to_screen() {
    // TODO: i need a globally accessible storage for a few data:
    // what conversion is chosen
    // what is the user typing
    /*
        // local time and date
        let now = js_sys::Date::new_0();

        let now_time = format!("{:02}:{:02}", now.get_hours(), now.get_minutes(),);
        let now_date = format!(
            "{:02}. {:02}. {:04}",
            now.get_date(),
            now.month() + 1,
            now.year(),
        );
        let now_day = format!("{}", VEC_DAY_NAME[now.get_day() as usize],);
        // just for fun show seconds in binary
        let now_seconds = format!(
            "seconds: {:02} in binary: {:08b}",
            now.get_seconds(),
            now.get_seconds(),
        );
    */
}

pub fn convert() {
    let value_orig = get_text("div_input");
    let conversion = get_text("div_toolbar");
    let conversion = conversion.as_ref();

    match conversion {
        "yyyy-mm-dd ---> fw" => match NaiveDate::parse_from_str(&value_orig, "%Y-%m-%d") {
            Ok(naive_date) => set_text("div_output", &naive_date_to_fullweek(naive_date)),
            Err(_err) => set_text("div_output", "unrecognized format"),
        },
        "fw ---> yyyy-mm-dd" => {
            let nd = fullweek_to_naive_date(&value_orig);
            match nd {
                Some(naive_date) => set_text(
                    "div_output",
                    naive_date.format("%Y-%m-%d").to_string().as_ref(),
                ),
                None => set_text("div_output", "unrecognized format"),
            }
        }
        "dd.mm.yyyy ---> fw" => match NaiveDate::parse_from_str(&value_orig, "%d.%m.%Y") {
            Ok(naive_date) => set_text("div_output", &naive_date_to_fullweek(naive_date)),
            Err(_err) => set_text("div_output", "unrecognized format"),
        },
        "fw ---> dd.mm.yyyy" => {
            let nd = fullweek_to_naive_date(&value_orig);
            match nd {
                Some(naive_date) => set_text(
                    "div_output",
                    naive_date.format("%d.%m.%Y").to_string().as_ref(),
                ),
                None => set_text("div_output", "unrecognized format"),
            }
        }
        "mm/dd/yyyy ---> fw" => match NaiveDate::parse_from_str(&value_orig, "%m/%d/%Y") {
            Ok(naive_date) => set_text("div_output", &naive_date_to_fullweek(naive_date)),
            Err(_err) => set_text("div_output", "unrecognized format"),
        },
        "fw ---> mm/dd/yyyy" => {
            let nd = fullweek_to_naive_date(&value_orig);
            match nd {
                Some(naive_date) => set_text(
                    "div_output",
                    naive_date.format("%m/%d/%Y").to_string().as_ref(),
                ),
                None => set_text("div_output", "unrecognized format"),
            }
        }
        "HH:MM 24---> md" => set_text("div_output", &value_orig),
        "md ---> HH:MM 24" => set_text("div_output", &value_orig),
        "HH:MM 12 ---> md" => set_text("div_output", &value_orig),
        "md ---> HH:MM 12" => set_text("div_output", &value_orig),
        "seconds ---> μd" => set_text("div_output", &value_orig),
        "μd ---> seconds" => set_text("div_output", &value_orig),
        _ => set_text("div_output", &format!("?? {}", conversion)),
    }
}

pub fn naive_date_to_fullweek(nd: NaiveDate) -> String {
    // return
    format!(
        r#"{}c {:02}fw {}d"#,
        nd.year(),
        ((nd.ordinal() as f64 - 0.1) / 7.0).floor() as u32 + 1,
        ((nd.ordinal() as f64 - 0.1) % 7.0).round() as u32
    )
}

pub fn fullweek_to_naive_date(s: &str) -> Option<NaiveDate> {
    // the format is fixed with space after c and fw
    use regex::Regex;
    let re = Regex::new(r"^\d{4}c [0-5][0-9]fw [1-7]d$").unwrap();
    use std::str::FromStr;
    if re.is_match(s) {
        NaiveDate::from_yo_opt(
            i32::from_str(&s[0..4]).unwrap_or(0),
            (u32::from_str(&s[6..8]).unwrap_or(0) - 1) * 7 + u32::from_str(&s[11..12]).unwrap_or(0),
        )
    } else {
        return None;
    }
}

pub fn set_event_handlers() {
    on_click!("div_1", div_cell_on_click, "1");
    on_click!("div_2", div_cell_on_click, "2");
    on_click!("div_3", div_cell_on_click, "3");
    on_click!("div_4", div_cell_on_click, "4");
    on_click!("div_5", div_cell_on_click, "5");
    on_click!("div_6", div_cell_on_click, "6");
    on_click!("div_7", div_cell_on_click, "7");
    on_click!("div_8", div_cell_on_click, "8");
    on_click!("div_9", div_cell_on_click, "9");
    on_click!("div_0", div_cell_on_click, "0");
    on_click!("div_dot", div_cell_on_click, ".");
    on_click!("div_common_era", div_cell_on_click, ":");
    on_click!("div_minus", div_cell_on_click, "-");
    on_click!("div_divide", div_cell_on_click, "/");
    on_click!("div_common_era", div_cell_on_click, "c ");
    on_click!("div_full_week", div_cell_on_click, "fw ");
    on_click!("div_day", div_cell_on_click, "d");

    on_click!("div_B", div_b_on_click);
    on_click!("div_C", div_c_on_click);
    on_click!("div_today", div_today_on_click);

    on_click!("div_toolbar", toolbar_on_click);

    on_click!("cnv_yf", cnv_on_click, "cnv_yf");
    on_click!("cnv_fy", cnv_on_click, "cnv_fy");
    on_click!("cnv_df", cnv_on_click, "cnv_df");
    on_click!("cnv_fd", cnv_on_click, "cnv_fd");
    on_click!("cnv_mf", cnv_on_click, "cnv_mf");
    on_click!("cnv_fm", cnv_on_click, "cnv_fm");
    on_click!("cnv_24m", cnv_on_click, "cnv_24m");
    on_click!("cnv_m24", cnv_on_click, "cnv_m24");
    on_click!("cnv_12m", cnv_on_click, "cnv_12m");
    on_click!("cnv_m12", cnv_on_click, "cnv_m12");
    on_click!("cnv_sm", cnv_on_click, "cnv_sm");
    on_click!("cnv_ms", cnv_on_click, "cnv_ms");

    on_click!("span_menu", menu_on_click);
    on_click!("modal_02_close", modal_02_close_on_click);
}

pub fn div_cell_on_click(text: &str) {
    let input_text = get_text("div_input");
    // the single space after c or fw
    if input_text.ends_with("c") || input_text.ends_with("fw") {
        set_text("div_input", &format!("{} {}", get_text("div_input"), text));
    } else {
        set_text("div_input", &format!("{}{}", get_text("div_input"), text));
    }
    convert();
}

pub fn div_b_on_click() {
    let mut text = get_text("div_input");
    if !text.is_empty() {
        text.pop().unwrap();
        set_text("div_input", &text);
        convert();
    }
}
pub fn div_c_on_click() {
    set_text("div_input", "");
    convert();
}

pub fn div_today_on_click() {
    let now = js_sys::Date::new_0();
    let now = NaiveDate::from_ymd(
        now.get_full_year() as i32,
        now.get_month() + 1,
        now.get_date(),
    );
    let conversion = get_text("div_toolbar");

    if conversion.starts_with("fw") {
        set_text("div_input", &naive_date_to_fullweek(now));
    } else if conversion.starts_with("yyyy-mm-dd") {
        set_text("div_input", now.format("%Y-%m-%d").to_string().as_ref());
    } else if conversion.starts_with("dd.mm.yyyy") {
        set_text("div_input", now.format("%d.%m.%Y").to_string().as_ref());
    } else if conversion.starts_with("mm/dd/yyyy") {
        set_text("div_input", now.format("%m/%d/%Y").to_string().as_ref());
    }

    convert();
}

/// open modal div
pub fn toolbar_on_click() {
    let _x = get_html_element_by_id("modal_01")
        .style()
        .set_property("display", "block");
}

pub fn cnv_on_click(element_id: &str) {
    let conversion = get_text(element_id);
    set_text("div_toolbar", &conversion);
    let _x = get_html_element_by_id("modal_01")
        .style()
        .set_property("display", "none");
    //different formats allows different characters

    match conversion.as_ref() {
        "yyyy-mm-dd ---> fw" => {
            debug_write("cnv_on_click yyyy-mm-dd ---> fw");
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        "fw ---> yyyy-mm-dd" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell disabled_button");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell");
            get_element_by_id("div_full_week").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
        }
        "dd.mm.yyyy ---> fw" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell disabled_button");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        "fw ---> dd.mm.yyyy" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell disabled_button");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell");
            get_element_by_id("div_full_week").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
        }
        "mm/dd/yyyy ---> fw" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell disabled_button");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        "fw ---> mm/dd/yyyy" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell disabled_button");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell");
            get_element_by_id("div_full_week").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
        }
        "HH:MM 24---> md" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        "md ---> HH:MM 24" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        "HH:MM 12 ---> md" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        "md ---> HH:MM 12" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        "seconds ---> μd" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        "μd ---> seconds" => {
            get_element_by_id("div_today").set_class_name("div_cell");
            get_element_by_id("div_minus").set_class_name("div_cell");
            get_element_by_id("div_multiply").set_class_name("div_cell disabled_button");
            get_element_by_id("div_divide").set_class_name("div_cell disabled_button");
            get_element_by_id("div_dot").set_class_name("div_cell disabled_button");
            get_element_by_id("div_common_era").set_class_name("div_cell disabled_button");
            get_element_by_id("div_full_week").set_class_name("div_cell disabled_button");
            get_element_by_id("div_day").set_class_name("div_cell disabled_button");
        }
        _ => set_text("div_output", &format!("?? {}", conversion)),
    }
    div_today_on_click();
    convert();
}

/// open modal div
pub fn menu_on_click() {
    let _x = get_html_element_by_id("modal_02")
        .style()
        .set_property("display", "block");
}

pub fn modal_02_close_on_click() {
    let _x = get_html_element_by_id("modal_02")
        .style()
        .set_property("display", "none");
}
