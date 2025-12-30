use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::collections::collection_edit_modal::CollectionEditModal;
use crate::components::icons::plus_icon::PlusIcon;
use crate::components::collections::my_card_collection::MyCardCollections;
use crate::utils::functions::api_base;
use crate::utils::types::deck::{Deck, DeckCreate};

#[function_component(CardCollections)]
pub fn card_collections() -> Html {
    let is_dialog_open = use_state(|| false);
    let editing_collection = use_state(|| Option::<Deck>::None);
    let decks = use_state(|| vec![]);
    let flashcard_len = decks.len();

    {
        let decks = decks.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let resp = Request::get(&format!("{}/api/decks", api_base())).send().await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(data) = r.json::<Vec<Deck>>().await {
                            decks.set(data);
                        }
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error fetching decks: status {}", r.status()).into());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                    }
                }
            });
            || ()
        });
    }

    let on_new = {
        let is_dialog_open = is_dialog_open.clone();
        let editing_card = editing_collection.clone();
        Callback::from(move |_| {
            editing_card.set(None);
            is_dialog_open.set(true);
        })
    };

    let handle_edit = {
        let editing_collection = editing_collection.clone();
        let is_dialog_open = is_dialog_open.clone();
        Callback::from(move |collection: Deck| {
            editing_collection.set(Some(collection));
            is_dialog_open.set(true);
        })
    };

    let handle_delete = {
        let decks = decks.clone();
        Callback::from(move |id: i64| {
            let decks = decks.clone();
            spawn_local(async move {
                let resp = Request::delete(&format!("{}/api/deck/{}", api_base(), id))
                    .send()
                    .await;

                if resp.is_ok() {
                    let mut new_list = (*decks).clone();
                    new_list.retain(|d| d.id != id);
                    decks.set(new_list);
                } else {
                    web_sys::console::error_1(&"Failed to delete deck".into());
                }
            });
        })
    };

    let close_dialog = {
        let is_dialog_open = is_dialog_open.clone();
        let editing_card = editing_collection.clone();
        Callback::from(move |_| {
            editing_card.set(None);
            is_dialog_open.set(false);
        })
    };

    let on_submit_new = {
        let decks = decks.clone();
        let is_dialog_open = is_dialog_open.clone();

        Callback::from(move |new_card: DeckCreate| {
            let decks = decks.clone();
            let is_dialog_open = is_dialog_open.clone();

            spawn_local(async move {
                let resp = Request::post(&format!("{}/api/deck", api_base()))
                    .json(&new_card).expect("Failed to serialize new card")
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(created_deck) = r.json::<Deck>().await {
                            let mut current_decks = (*decks).clone();
                            current_decks.push(created_deck);
                            decks.set(current_decks);
                            is_dialog_open.set(false);
                        }
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error saving deck: status {}", r.status()).into());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                    }
                }
            });
        })
    };

    let on_submit_update = {
        let decks = decks.clone();
        let is_dialog_open = is_dialog_open.clone();

        Callback::from(move |(id, updated_data): (i64, Deck)| {
            let decks = decks.clone();
            let is_dialog_open = is_dialog_open.clone();

            spawn_local(async move {
                let resp = Request::put(&format!("{}/api/deck/{}", api_base(), id))
                    .json(&updated_data).expect("Failed to serialize updated data")
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(updated_deck) = r.json::<Deck>().await {
                            let mut current_decks = (*decks).clone();
                            if let Some(index) = current_decks.iter().position(|d| d.id == id) {
                                current_decks[index] = updated_deck;
                                decks.set(current_decks);
                            }
                            is_dialog_open.set(false);
                        }
                    }
                    Ok(r) => web_sys::console::error_1(&format!("Error updating: {}", r.status()).into()),
                    Err(e) => web_sys::console::error_1(&format!("Net error: {}", e).into()),
                }
            });
        })
    };

    let open_new_header_btn = {
        let on_new = on_new.clone();
        Callback::from(move |_| on_new.emit(()))
    };

    html! {
        <div class="bg-slate-50 min-h-screen">
            <div class="container mx-auto px-4 py-8">
                <div class="flex items-center justify-between mb-8">
                    <div>
                        <h1 class="text-3xl mb-2">{"Balíčky karet"}</h1>
                        <p class="text-gray-600">{ format!("Celkem {} balíčků", flashcard_len.to_string()) }</p>
                    </div>

                    <button onclick={open_new_header_btn}
                        class="inline-flex items-center rounded-lg px-4 py-2 font-medium text-white
                               bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 shadow">
                        <div class="mr-2 h-5 w-5">
                            <PlusIcon />
                        </div>
                        {"Nový balíček"}
                    </button>
                </div>

                <MyCardCollections
                    decks={(*decks).clone()}
                    on_update_deck={handle_edit}
                    on_delete_deck={handle_delete}
                />

                { if *is_dialog_open {
                    html!{
                        <CollectionEditModal
                            on_close={close_dialog.clone()}
                            editing_collection={(*editing_collection).clone()}
                            on_submit_new={on_submit_new}
                            on_submit_update={on_submit_update}
                        />
                    }
                } else { html!{} } }
            </div>
        </div>
    }
}