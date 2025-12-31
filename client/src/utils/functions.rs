use wasm_bindgen::{JsCast, JsValue};
use crate::utils::types::flashcard::StudyFlashcard;
use chrono::{NaiveDateTime};
//AI

pub fn api_base() -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let protocol = location.protocol().unwrap();
    let hostname = location.hostname().unwrap();
    format!("{}//{}:3000", protocol, hostname)
}

pub fn compare_ignore_case(answer: String, user_answer: String) -> bool {
    answer.trim().eq_ignore_ascii_case(user_answer.trim())
}

pub fn download_file(data: String, filename: String, content_type: String) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let blob = web_sys::Blob::new_with_str_sequence_and_options(
        &js_sys::Array::of1(&JsValue::from_str(&data)),
        web_sys::BlobPropertyBag::new().type_(&content_type),
    ).unwrap();

    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", &filename).unwrap();
    document.body().unwrap().append_child(&a).unwrap();
    a.dyn_ref::<web_sys::HtmlAnchorElement>().unwrap().click();
    document.body().unwrap().remove_child(&a).unwrap();
    web_sys::Url::revoke_object_url(&url).unwrap();
}

pub fn shuffle_cards(cards: &mut [StudyFlashcard]) {
    let len = cards.len();
    if len == 0 {
        return;
    }
    for i in (1..len).rev() {
        let random_idx = (js_sys::Math::random() * (i + 1) as f64).floor() as usize;
        cards.swap(i, random_idx);
    }
}

pub fn format_date(date: &str) -> String {
    let parse_pattern = "%Y-%m-%d %H:%M:%S";

    match NaiveDateTime::parse_from_str(date, parse_pattern) {
        Ok(dt) => {
            dt.format("%d. %m. %Y %H:%M").to_string()
        },
        Err(_) => {
            date.to_string()
        }
    }
}