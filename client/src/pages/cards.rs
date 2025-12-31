use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::components::cards::card_edit_modal::CardEditModal;
use crate::components::cards::my_cards::MyCards;
use crate::components::icons::plus_icon::PlusIcon;
use crate::components::reusable::toast_provider::use_toast;
use crate::utils::functions::api_base;
use crate::utils::hooks::use_cards::use_cards;
use crate::utils::hooks::use_decks::use_decks;
use crate::utils::types::flashcard::{Flashcard, FlashcardPatch, NewFlashcard};

#[function_component(Cards)]
pub fn cards() -> Html {
    let toast = use_toast();
    let is_dialog_open = use_state(|| false);
    let editing_card = use_state(|| Option::<Flashcard>::None);
    let decks = use_decks();
    let flashcards = use_cards();

    let on_add = {
        let flashcards = flashcards.clone();
        let is_dialog_open = is_dialog_open.clone();
        let toast = toast.clone();

        Callback::from(move |new_card: NewFlashcard| {
            let flashcards = flashcards.clone();
            let is_dialog_open = is_dialog_open.clone();
            let toast = toast.clone();

            spawn_local(async move {
                let resp = Request::post(&format!("{}/api/card", api_base()))
                    .json(&new_card)
                    .expect("Failed to serialize new card")
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(created_card) = r.json::<Flashcard>().await {
                            let mut list = (*flashcards).clone();
                            list.push(created_card);
                            flashcards.set(list);
                            is_dialog_open.set(false);
                            toast.success("Kartička úspěšně vytvořena".to_string());
                        }
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error saving card: status {}", r.status()).into());
                        toast.error("Chyba při vytváření kartičky".to_string());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                        toast.error("Chyba při vytváření kartičky".to_string());
                    }
                }
            });
        })
    };

    let on_update = {
        let flashcards = flashcards.clone();
        let toast = toast.clone();

        Callback::from(move |(id, patch): (i64, FlashcardPatch)| {
            let flashcards = flashcards.clone();
            let toast = toast.clone();

            spawn_local(async move {
                let resp = Request::put(&format!("{}/api/card/{}", api_base(), id))
                    .json(&patch)
                    .expect("Failed to serialize card")
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        if let Ok(created_card) = r.json::<Flashcard>().await {
                            let mut list = (*flashcards).clone();
                            if let Some(card) = list.iter_mut().find(|c| c.id == id) {
                                /*if let Some(q) = patch.question { card.question = q; }
                                if let Some(a) = patch.answer { card.answer = a; }
                                if let Some(d) = patch.deck_id { card.deck_id = d; }*/
                                *card = created_card;
                            }
                            flashcards.set(list);
                            toast.success("Kartička upravena".to_string());
                        }
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error saving card: status {}", r.status()).into());
                        toast.error("Chyba při ukládání změn".to_string());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                        toast.error("Chyba při ukládání změn".to_string());
                    }
                }
            });
        })
    };

    let on_delete = {
        let flashcards = flashcards.clone();
        let toast = toast.clone();

        Callback::from(move |id: i64| {
            let flashcards = flashcards.clone();
            let toast = toast.clone();

            spawn_local(async move {
                let resp = Request::delete(&format!("{}/api/card/{}", api_base(), id))
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        let mut current_list = (*flashcards).clone();
                        current_list.retain(|c| c.id != id);
                        flashcards.set(current_list);
                        toast.success("Kartička smazána".to_string());
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error deleting card: status {}", r.status()).into());
                        toast.error("Chyba při mazání kartičky".to_string());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                        toast.error("Chyba při mazání kartičky".to_string());
                    }
                }
            });
        })
    };

    let on_new = {
        let is_dialog_open = is_dialog_open.clone();
        let editing_card = editing_card.clone();
        Callback::from(move |_| {
            editing_card.set(None);
            is_dialog_open.set(true);
        })
    };

    let handle_edit = {
        let editing_card = editing_card.clone();
        let is_dialog_open = is_dialog_open.clone();
        Callback::from(move |card: Flashcard| {
            editing_card.set(Some(card));
            is_dialog_open.set(true);
        })
    };

    let close_dialog = {
        let is_dialog_open = is_dialog_open.clone();
        let editing_card = editing_card.clone();
        Callback::from(move |_| {
            editing_card.set(None);
            is_dialog_open.set(false);
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
                        <h1 class="text-3xl mb-2">{"Správa kartiček"}</h1>
                        <p class="text-gray-600">{ format!("Celkem {} kartiček", flashcards.len()) }</p>
                    </div>

                    <button onclick={open_new_header_btn}
                        class="inline-flex items-center rounded-lg px-4 py-2 font-medium text-white
                               bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 shadow">
                        <div class="mr-2 h-5 w-5">
                            <PlusIcon />
                        </div>
                        {"Nová kartička"}
                    </button>
                </div>

                <MyCards
                    flashcards={(*flashcards).clone()}
                    decks={(*decks).clone()}
                    on_new={on_new.clone()}
                    on_edit={handle_edit.clone()}
                    on_delete={on_delete}
                />

                { if *is_dialog_open {
                    html!{
                        <CardEditModal
                            on_close={close_dialog.clone()}
                            editing_card={(*editing_card).clone()}
                            decks={(*decks).clone()}
                            on_submit_new={on_add.clone()}
                            on_submit_update={on_update}
                        />
                    }
                } else { html!{} } }
            </div>
        </div>
    }
}
