// region: lmake_md_to_doc_comments include README.md A //!

// endregion: lmake_md_to_doc_comments include README.md A //!

// use unwrap::unwrap;
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
    write_to_screen();
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
            now.get_month() + 1,
            now.get_full_year(),
        );
        let now_day = format!("{}", VEC_DAY_NAME[now.get_day() as usize],);
        // just for fun show seconds in binary
        let now_seconds = format!(
            "seconds: {:02} in binary: {:08b}",
            now.get_seconds(),
            now.get_seconds(),
        );
    */
    div_today_on_click();
}

pub fn convert() {
    let value_orig = get_text("div_input");
    let conversion = get_text("div_toolbar");
    let conversion = conversion.as_ref();
    if conversion == "yyyy-mm-dd ---> fw" {
        debug_write("equal yyyy-mm-dd ---> fw");
    } else {
        debug_write(conversion);
    }
    match conversion {
        "yyyy-mm-dd ---> fw" => set_text("div_output", &value_orig),
        "fw ---> yyyy-mm-dd" => set_text("div_output", &value_orig),
        "dd.mm.yyyy ---> fw" => set_text("div_output", &value_orig),
        "fw ---> dd.mm.yyyy" => set_text("div_output", &value_orig),
        "mm/dd/yyyy ---> fw" => set_text("div_output", &value_orig),
        "fw ---> mm/dd/yyyy" => set_text("div_output", &value_orig),
        "HH:MM 24---> md" => set_text("div_output", &value_orig),
        "md ---> HH:MM 24" => set_text("div_output", &value_orig),
        "HH:MM 12 ---> md" => set_text("div_output", &value_orig),
        "md ---> HH:MM 12" => set_text("div_output", &value_orig),
        "seconds ---> μd" => set_text("div_output", &value_orig),
        "μd ---> seconds" => set_text("div_output", &value_orig),
        _ => set_text("div_output", &format!("?? {}", conversion)),
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
    on_click!("div_colon", div_cell_on_click, ":");

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
    set_text("div_input", &format!("{}{}", get_text("div_input"), text));
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
    let now_date = format!(
        "{:02}. {:02}. {:04}",
        now.get_date(),
        now.get_month() + 1,
        now.get_full_year(),
    );
    set_text("div_input", &now_date);
    convert();
}

/// open modal div
pub fn toolbar_on_click() {
    let _x = get_html_element_by_id("modal_01")
        .style()
        .set_property("display", "block");
}

pub fn cnv_on_click(element_id: &str) {
    let text = get_text(element_id);
    set_text("div_toolbar", &text);
    let _x = get_html_element_by_id("modal_01")
        .style()
        .set_property("display", "none");
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
