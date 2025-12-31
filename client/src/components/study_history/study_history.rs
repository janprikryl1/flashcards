use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCredentials;
use yew::{function_component, html, Callback, Html};
use crate::components::reusable::toast_provider::use_toast;
use crate::utils::functions::{api_base, format_date};
use crate::utils::hooks::use_study_history::use_study_history;

#[function_component(StudyHistory)]
pub fn study_history() -> Html {
    let study_history = use_study_history();
    let toast = use_toast();

    let on_delete = {
        let study_history = study_history.clone();
        let toast = toast.clone();

        Callback::from(move |id_to_delete: i64| {
            let study_history = study_history.clone();
            let toast = toast.clone();

            spawn_local(async move {
                let resp = Request::delete(&format!("{}/api/study-history/{}", api_base(), id_to_delete))
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        let mut updated_history = (*study_history).clone();
                        updated_history.retain(|h| h.id != id_to_delete);
                        study_history.set(updated_history);
                        toast.success("Záznam byl úspěšně smazán".to_string());
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error deleting study history: {}", r.status()).into());
                        toast.error("Chyba při mazání záznamu".to_string());
                    },
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                        toast.error("Chyba při mazání záznamu".to_string());
                    },
                }
            });
        })
    };

    if !study_history.is_empty() {
        html! {
            <div class="p-8 mt-8 border rounded-xl shadow-sm">
                <div class="space-y-6">
                    <div>
                        <h2 class="text-2xl mb-4">{"Historie procvičování"}</h2>
                        {
                            (*study_history).clone().into_iter().map(|history| { //AI
                            let on_click_delete = on_delete.clone();
                            let id = history.id;

                            html! {
                                <div class="mb-4 p-4 border rounded-lg hover:shadow-md transition-shadow bg-gray-50">
                                    <div class="flex justify-between items-start">
                                        <div>
                                            <p class="text-lg font-medium mb-2">{ format!("Balíček: {}", history.deck_name) }</p>
                                            <p class="text-gray-600 mb-1">{ format!("Úspěšnost: {}%", history.accuracy) }</p>
                                            <p class="text-gray-500 text-sm">{ format!("Datum: {}", format_date(&history.filled_at)) }</p>
                                        </div>

                                        <button
                                            onclick={move |_| on_click_delete.emit(id)}
                                            class="p-2 text-gray-400 hover:text-red-500 hover:bg-red-50 rounded-full transition-colors"
                                            title="Smazat záznam"
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                <path d="M3 6h18"/>
                                                <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/>
                                                <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
                                                <line x1="10" x2="10" y1="11" y2="17"/>
                                                <line x1="14" x2="14" y1="11" y2="17"/>
                                            </svg>
                                        </button>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}