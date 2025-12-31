use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use crate::components::reusable::toast_provider::use_toast;
use crate::utils::functions::api_base;
use crate::utils::types::deck::Deck;

#[hook]
pub fn use_decks() -> UseStateHandle<Vec<Deck>> {
    let toast = use_toast();
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
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error loading decks: {}", r.status()).into());
                        toast.error("Chyba při načítání balíčků".to_string());
                    },
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                        toast.error("Chyba při načítání balíčků".to_string());
                    }
                }
            });
            || ()
        });
    }

    decks
}