use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use crate::utils::functions::api_base;
use crate::utils::types::flashcard::Flashcard;

#[hook]
pub fn use_cards() -> UseStateHandle<Vec<Flashcard>> {
    let cards = use_state(Vec::new);

    {
        let cards = cards.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = Request::get(&format!("{}/api/cards", api_base()))
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(data) = r.json::<Vec<Flashcard>>().await {
                            cards.set(data);
                        }
                    }
                    Ok(r) => web_sys::console::error_1(&format!("Error loading cards: {}", r.status()).into()),
                    Err(e) => web_sys::console::error_1(&format!("Network error: {}", e).into()),
                }
            });
            || ()
        });
    }

    cards
}