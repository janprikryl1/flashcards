use gloo_net::http::Request;
use web_sys::RequestCredentials;
use yew::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use crate::auth::use_auth;
use crate::components::icons::book_open_icon::BookOpenIcon;
use crate::components::icons::folder_open_icon::FolderOpenIcon;
use crate::components::reusable::toast_provider::use_toast;
use crate::utils::functions::api_base;

#[derive(Clone, PartialEq, Deserialize, Default)]
struct CardCountDto {
    pub cards: i64,
    pub decks: i64,
}

#[function_component(StatsSummary)]
pub fn stats_summary() -> Html {
    let toast = use_toast();
    let auth = use_auth();
    let counts = use_state(|| CardCountDto::default());

    {
        let counts = counts.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = Request::get(&format!("{}/api/card-count", api_base()))
                    .credentials(RequestCredentials::Include)
                    .header("Content-Type", "application/json")
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(data) = r.json::<CardCountDto>().await {
                            counts.set(data);
                        }
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error fetching counts: status {}", r.status()).into());
                        toast.error("Chyba při načítání dat".to_string());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                        toast.error("Chyba při načítání dat".to_string());
                    }
                }
            });
            || ()
        });
    }

    if let Some(_user) = &auth.me {
        html! {
            <div class="grid md:grid-cols-2 gap-6 mb-16">
                <div class="p-6 bg-white/80 backdrop-blur border border-blue-200 rounded-2xl">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-gray-600 mb-1">{"Celkem balíčků"}</p>
                            <p class="text-4xl text-blue-600">{ (*counts).decks }</p>
                        </div>
                        <div class={classes!("h-12","w-12","text-blue-600","opacity-50")}>
                            <FolderOpenIcon />
                        </div>
                    </div>
                </div>

                <div class="p-6 bg-white/80 backdrop-blur border border-purple-200 rounded-2xl">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-gray-600 mb-1">{"Celkem kartiček"}</p>
                            <p class="text-4xl text-purple-600">{ (*counts).cards }</p>
                        </div>
                        <div class={classes!("h-12","w-12","text-blue-600","opacity-50")}>
                            <BookOpenIcon />
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}