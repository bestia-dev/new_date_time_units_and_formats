// region: lmake_md_to_doc_comments include README.md A //!

// endregion: lmake_md_to_doc_comments include README.md A //!

// use unwrap::unwrap;
use chrono::{NaiveDate, NaiveTime};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

mod veeks_millis_mod;
mod web_sys_mod;

use veeks_millis_mod::*;
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
    div_now_on_click();

    // return
    Ok(())
}

pub fn set_event_handlers() {
    on_click!("div_1", div_cell_multi_value_on_click, "div_1");
    on_click!("div_2", div_cell_multi_value_on_click, "div_2");
    on_click!("div_3", div_cell_multi_value_on_click, "div_3");
    on_click!("div_4", div_cell_multi_value_on_click, "div_4");
    on_click!("div_5", div_cell_multi_value_on_click, "div_5");
    on_click!("div_6", div_cell_multi_value_on_click, "div_6");
    on_click!("div_7", div_cell_multi_value_on_click, "div_7");
    on_click!("div_8", div_cell_multi_value_on_click, "div_8");
    on_click!("div_9", div_cell_multi_value_on_click, "div_9");
    on_click!("div_0", div_cell_multi_value_on_click, "div_0");

    on_click!("div_dot", div_cell_on_click, ".");
    on_click!("div_common_era", div_cell_on_click, ":");
    on_click!("div_hyphen", div_cell_on_click, "-");
    on_click!("div_slash", div_cell_on_click, "/");

    on_click!(
        "div_common_era",
        div_cell_multi_value_on_click,
        "div_common_era"
    );
    on_click!("div_veek", div_cell_multi_value_on_click, "div_veek");
    on_click!("div_day", div_cell_multi_value_on_click, "div_day");

    on_click!("div_backspace", div_backspace_on_click);
    on_click!("div_clear", div_c_on_click);
    on_click!("div_now", div_now_on_click);

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

pub fn convert() {
    let value_orig = get_text("div_input");
    let conversion = get_text("div_toolbar");
    let conversion = conversion.as_ref();

    match conversion {
        "yyyy-mm-dd ---> v" => match NaiveDate::parse_from_str(&value_orig, "%Y-%m-%d") {
            Ok(naive_date) => set_text("div_output", &naive_date_to_veek(naive_date)),
            Err(_err) => set_text("div_output", "unrecognized format"),
        },
        "v ---> yyyy-mm-dd" => {
            let nd = veek_to_naive_date(&value_orig);
            match nd {
                Some(naive_date) => set_text(
                    "div_output",
                    naive_date.format("%Y-%m-%d").to_string().as_ref(),
                ),
                None => set_text("div_output", "unrecognized format"),
            }
        }
        "dd.mm.yyyy ---> v" => match NaiveDate::parse_from_str(&value_orig, "%d.%m.%Y") {
            Ok(naive_date) => set_text("div_output", &naive_date_to_veek(naive_date)),
            Err(_err) => set_text("div_output", "unrecognized format"),
        },
        "v ---> dd.mm.yyyy" => {
            let nd = veek_to_naive_date(&value_orig);
            match nd {
                Some(naive_date) => set_text(
                    "div_output",
                    naive_date.format("%d.%m.%Y").to_string().as_ref(),
                ),
                None => set_text("div_output", "unrecognized format"),
            }
        }
        "mm/dd/yyyy ---> v" => match NaiveDate::parse_from_str(&value_orig, "%m/%d/%Y") {
            Ok(naive_date) => set_text("div_output", &naive_date_to_veek(naive_date)),
            Err(_err) => set_text("div_output", "unrecognized format"),
        },
        "v ---> mm/dd/yyyy" => {
            let nd = veek_to_naive_date(&value_orig);
            match nd {
                Some(naive_date) => set_text(
                    "div_output",
                    naive_date.format("%m/%d/%Y").to_string().as_ref(),
                ),
                None => set_text("div_output", "unrecognized format"),
            }
        }
        "HH:MM 24 ---> md" => match NaiveTime::parse_from_str(&value_orig, "%H:%M") {
            Ok(naive_time) => set_text("div_output", &naive_time_to_millis(naive_time)),
            Err(_err) => set_text("div_output", "unrecognized format"),
        },
        "md ---> HH:MM 24" => {
            let millis = millis_from_str_opt(&value_orig);
            match millis {
                Some(millis) => {
                    let nt = millis_to_naive_time(millis);
                    match nt {
                        Some(naive_time) => set_text(
                            "div_output",
                            naive_time.format("%H:%M").to_string().as_ref(),
                        ),
                        None => set_text("div_output", "unrecognized format"),
                    }
                }
                None => set_text("div_output", "unrecognized format"),
            }
        }
        "HH:MM 12 ---> md" => match NaiveTime::parse_from_str(&value_orig, "%I:%M %p") {
            Ok(naive_time) => set_text("div_output", &naive_time_to_millis(naive_time)),
            Err(_err) => set_text("div_output", "unrecognized format"),
        },
        "md ---> HH:MM 12" => {
            let millis = millis_from_str_opt(&value_orig);
            match millis {
                Some(millis) => {
                    let nt = millis_to_naive_time(millis);
                    match nt {
                        Some(naive_time) => set_text(
                            "div_output",
                            naive_time.format("%I:%M %p").to_string().as_ref(),
                        ),
                        None => set_text("div_output", "unrecognized format"),
                    }
                }
                None => set_text("div_output", "unrecognized format"),
            }
        }
        "seconds ---> μd" => {
            use std::str::FromStr;
            let seconds = f64::from_str(&value_orig);
            match seconds {
                Ok(seconds) => {
                    let micros = seconds_to_micros(seconds);
                    // format to 3 decimal places
                    let micros = format!(r#"{:.3}μd"#, micros);
                    set_text("div_output", &micros);
                }
                Err(_err) => set_text("div_output", "unrecognized format"),
            }
        }
        "μd ---> seconds" => {
            let micros = micros_from_str_opt(&value_orig);
            match micros {
                Some(micros) => {
                    let seconds = micros_to_seconds(micros);
                    // format to 3 decimal places
                    let seconds = format!("{:.3}", seconds);
                    set_text("div_output", &seconds);
                }
                None => set_text("div_output", "unrecognized format"),
            }
        }
        _ => set_text("div_output", &format!("?? {}", conversion)),
    }
}

pub fn div_cell_on_click(text: &str) {
    let input_text = get_text("div_input");
    // the single space after c or v
    if input_text.ends_with("c") || input_text.ends_with("v") {
        set_text("div_input", &format!("{} {}", get_text("div_input"), text));
    } else {
        set_text("div_input", &format!("{}{}", get_text("div_input"), text));
    }
    convert();
}

pub fn div_cell_multi_value_on_click(element_id: &str) {
    let input_text = get_text("div_input");
    let text = get_text(element_id);
    if text == "P" || text == "A" {
        // AM PM has space before
        set_text("div_input", &format!("{} {}", get_text("div_input"), text));
    } else if input_text.ends_with("c") || input_text.ends_with("v") {
        // the single space after c or v
        set_text("div_input", &format!("{} {}", get_text("div_input"), text));
    } else {
        set_text("div_input", &format!("{}{}", get_text("div_input"), text));
    }

    convert();
}

pub fn div_backspace_on_click() {
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

pub fn div_now_on_click() {
    let now_js = js_sys::Date::new_0();
    let now_time = NaiveTime::from_hms(now_js.get_hours(), now_js.get_minutes(), 0);
    let now_date = NaiveDate::from_ymd(
        now_js.get_full_year() as i32,
        now_js.get_month() + 1,
        now_js.get_date(),
    );
    let conversion = get_text("div_toolbar");

    if conversion.starts_with("v") {
        set_text("div_input", &naive_date_to_veek(now_date));
    } else if conversion.starts_with("yyyy-mm-dd") {
        set_text(
            "div_input",
            now_date.format("%Y-%m-%d").to_string().as_ref(),
        );
    } else if conversion.starts_with("dd.mm.yyyy") {
        set_text(
            "div_input",
            now_date.format("%d.%m.%Y").to_string().as_ref(),
        );
    } else if conversion.starts_with("mm/dd/yyyy") {
        set_text(
            "div_input",
            now_date.format("%m/%d/%Y").to_string().as_ref(),
        );
    } else if conversion.starts_with("HH:MM 24") {
        set_text("div_input", now_time.format("%H:%M").to_string().as_ref());
    } else if conversion.starts_with("HH:MM 12") {
        set_text(
            "div_input",
            now_time.format("%I:%M %p").to_string().as_ref(),
        );
    } else if conversion.starts_with("md") {
        set_text("div_input", &naive_time_to_millis(now_time));
    } else if conversion.starts_with("μd") {
        set_text("div_input", "110.880μd");
    } else if conversion.starts_with("seconds") {
        set_text("div_input", "9.58");
    }
    convert();
}

/// open modal div
pub fn toolbar_on_click() {
    modal_open("modal_01");
}

pub fn cnv_on_click(element_id: &str) {
    let conversion = get_text(element_id);
    set_text("div_toolbar", &conversion);
    modal_close("modal_01");
    // reset to the most often value
    get_element_by_id("div_now").set_class_name("div_cell");
    get_element_by_id("div_hyphen").set_class_name("div_cell cell_disabled");
    get_element_by_id("div_colon").set_class_name("div_cell cell_disabled");
    get_element_by_id("div_slash").set_class_name("div_cell cell_disabled");
    get_element_by_id("div_dot").set_class_name("div_cell cell_disabled");
    get_element_by_id("div_common_era").set_class_name("div_cell cell_disabled");
    get_element_by_id("div_veek").set_class_name("div_cell cell_disabled");
    get_element_by_id("div_day").set_class_name("div_cell cell_disabled");
    get_html_element_by_id("div_common_era").set_inner_text("c");
    get_html_element_by_id("div_veek").set_inner_text("v");
    get_html_element_by_id("div_day").set_inner_text("d");

    // different formats allows different characters

    match conversion.as_ref() {
        "yyyy-mm-dd ---> v" => {
            debug_write("cnv_on_click yyyy-mm-dd ---> v");
            get_element_by_id("div_hyphen").set_class_name("div_cell");
        }
        "v ---> yyyy-mm-dd" => {
            get_element_by_id("div_common_era").set_class_name("div_cell");
            get_element_by_id("div_veek").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
        }
        "dd.mm.yyyy ---> v" => {
            get_element_by_id("div_dot").set_class_name("div_cell");
        }
        "v ---> dd.mm.yyyy" => {
            get_element_by_id("div_common_era").set_class_name("div_cell");
            get_element_by_id("div_veek").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
        }
        "mm/dd/yyyy ---> v" => {
            get_element_by_id("div_slash").set_class_name("div_cell");
        }
        "v ---> mm/dd/yyyy" => {
            get_element_by_id("div_common_era").set_class_name("div_cell");
            get_element_by_id("div_veek").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
        }
        "HH:MM 24 ---> md" => {
            get_element_by_id("div_colon").set_class_name("div_cell");
        }
        "md ---> HH:MM 24" => {
            get_element_by_id("div_veek").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
            get_html_element_by_id("div_veek").set_inner_text("m");
            get_html_element_by_id("div_day").set_inner_text("d");
        }
        "HH:MM 12 ---> md" => {
            get_element_by_id("div_colon").set_class_name("div_cell");
            get_element_by_id("div_common_era").set_class_name("div_cell");
            get_element_by_id("div_veek").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
            get_html_element_by_id("div_common_era").set_inner_text("A");
            get_html_element_by_id("div_veek").set_inner_text("P");
            get_html_element_by_id("div_day").set_inner_text("M");
        }
        "md ---> HH:MM 12" => {
            get_element_by_id("div_veek").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
            get_html_element_by_id("div_veek").set_inner_text("m");
            get_html_element_by_id("div_day").set_inner_text("d");
        }
        "seconds ---> μd" => {
            get_element_by_id("div_now").set_class_name("div_cell cell_disabled");
            get_element_by_id("div_dot").set_class_name("div_cell");
        }
        "μd ---> seconds" => {
            get_element_by_id("div_now").set_class_name("div_cell cell_disabled");
            get_element_by_id("div_dot").set_class_name("div_cell");
            get_element_by_id("div_veek").set_class_name("div_cell");
            get_element_by_id("div_day").set_class_name("div_cell");
            get_html_element_by_id("div_veek").set_inner_text("μ");
            get_html_element_by_id("div_day").set_inner_text("d");
        }
        _ => set_text("div_output", &format!("?? {}", conversion)),
    }
    div_now_on_click();
    convert();
}

/// open modal div
pub fn menu_on_click() {
    modal_open("modal_02");
}

pub fn modal_02_close_on_click() {
    modal_close("modal_02");
}
