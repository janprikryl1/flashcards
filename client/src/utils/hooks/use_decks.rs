use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use crate::utils::functions::api_base;
use crate::utils::types::deck::Deck;

#[hook]
pub fn use_decks() -> UseStateHandle<Vec<Deck>> {
    let decks = use_state(Vec::new);

    {
        let decks = decks.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = Request::get(&format!("{}/api/decks", api_base()))
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(data) = r.json::<Vec<Deck>>().await {
                            decks.set(data);
                        }
                    }
                    Ok(r) => web_sys::console::error_1(&format!("Error loading decks: {}", r.status()).into()),
                    Err(e) => web_sys::console::error_1(&format!("Network error: {}", e).into()),
                }
            });
            || ()
        });
    }

    decks
}