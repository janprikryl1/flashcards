use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCredentials;
use crate::components::reusable::toast_provider::use_toast;
use crate::utils::functions::api_base;
use crate::utils::types::study_history::StudyHistory;

#[hook]
pub fn use_study_history() -> UseStateHandle<Vec<StudyHistory>> {
    let toast = use_toast();
    let study_history = use_state(Vec::new);

    {
        let study_history = study_history.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = Request::get(&format!("{}/api/study-history", api_base()))
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(data) = r.json::<Vec<StudyHistory>>().await {
                            study_history.set(data);
                        }
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error loading study history: {}", r.status()).into());
                        toast.error("Chyba při načítání historie".to_string());
                    },
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                        toast.error("Chyba při načítání historie".to_string());
                    },
                }
            });
            || ()
        });
    }

    study_history
}