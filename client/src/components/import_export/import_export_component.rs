use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, use_node_ref, Callback, Html, Properties, TargetCast};
use crate::utils::types::deck::Deck;
use crate::utils::types::flashcard::Flashcard;
use web_sys::{Event, HtmlInputElement};
use crate::components::reusable::toast_provider::use_toast;
use crate::utils::functions::{api_base, download_file};

#[derive(Properties, PartialEq)]
pub struct ImportExportProps {
    pub decks: Vec<Deck>,
    pub available_cards: Vec<Flashcard>
}

#[function_component(ImportExportComponent)]
pub fn import_export_component(props: &ImportExportProps) -> Html {
    let toast = use_toast();
    let available_cards_count = props.available_cards.len();
    let file_input_ref = use_node_ref();

    let on_export = {
        let available_cards = props.available_cards.clone();
        let toast = toast.clone();

        Callback::from(move |_| {
            let data = serde_json::to_string(&available_cards);
            match data {
                Ok(json) => {
                    download_file(json, "flashcards_export.json".to_string(), "application/json".to_string());
                },
                Err(e) => {
                    web_sys::console::error_1(&format!("Chyba při exportu: {}", e).into());
                    toast.error("Chyba při exportu".to_string());
                }
            }
        })
    };

    let on_import_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    let on_file_change = {
        let toast = toast.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if let Some(file) = input.files() {
                if let Some(file) = file.get(0) {
                    let toast = toast.clone();

                    spawn_local(async move {
                        match wasm_bindgen_futures::JsFuture::from(file.text()).await {
                            Ok(text_js) => {
                                if let Some(text) = text_js.as_string() {
                                    let result = Request::post(&format!("{}/api/import", api_base()))
                                        .header("Content-Type", "application/json")
                                        .body(text)
                                        .expect("Failed to serialize data")
                                        .send()
                                        .await;

                                    match result {
                                        Ok(resp) if resp.ok() => {
                                            toast.success("Karty úspěšně importovány!".to_string());
                                        },
                                        _ => toast.error("Chyba při nahrávání na server".to_string()),
                                    }
                                }
                            },
                            Err(e) => {
                                web_sys::console::error_1(&format!("Chyba při parsování souboru: {:?}", e).into());
                                toast.error("Chyba při parsování souboru".to_string());
                            }
                        }
                    })
                }
            }
            input.set_value("");
        })
    };

    html! {
        <>
            { if available_cards_count != 0 { html! {
                <button
                    onclick={on_export}
                    class={classes!(
                        "w-full",
                        "bg-gray-300",
                        "py-3",
                        if available_cards_count == 0 { "opacity-50 cursor-not-allowed" } else { "hover:opacity-90" }
                    )}
                >
                    {"Exportovat otázky do JSON"}
                </button>
            }} else {html! {}} }
            <input
                type="file"
                ref={file_input_ref}
                class="hidden"
                accept=".json"
                onchange={on_file_change}
            />
            <button
                onclick={on_import_click}
                class="w-full bg-gray-300 py-3"
            >
                {"Importovat otázky ze JSON"}
            </button>
        </>
    }
}