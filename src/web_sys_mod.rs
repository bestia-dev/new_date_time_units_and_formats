// web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! local_storage, session_storage,...

// region: use
use unwrap::unwrap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console;
// endregion: use

/// Simple macro to set listener of on_click events to an element_id.
/// no args: on_click!(element_1_id, function_ident)
/// no args: on_click!("example_email",example_email)

#[macro_export]
macro_rules! on_click {
    ($element_1_id: expr, $function_ident: ident) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident();
        }) as Box<dyn FnMut()>);

        let html_element = get_html_element_by_id($element_1_id);
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
    ($element_1_id: expr, $function_ident: ident, $arg_1: expr) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident($arg_1);
        }) as Box<dyn FnMut()>);

        let html_element = get_html_element_by_id($element_1_id);
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

pub fn get_text(element_id: &str) -> String {
    let div = get_html_element_by_id(element_id);
    div.inner_text()
}

pub fn set_text(element_id: &str, text: &str) {
    let div = get_html_element_by_id(element_id);
    div.set_inner_text(text);
}
