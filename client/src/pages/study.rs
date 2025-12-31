use std::collections::HashSet;
use gloo_net::http::Request;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlSelectElement, RequestCredentials};
use crate::utils::functions::api_base;
use crate::components::reusable::toast_provider::use_toast;
use crate::components::study::study_session::StudySession;
use crate::components::study::study_setup::StudySetup;
use crate::utils::functions::shuffle_cards;
use crate::utils::hooks::use_cards::use_cards;
use crate::utils::hooks::use_decks::use_decks;
use crate::utils::types::flashcard::{Flashcard, StudyFlashcard};
use crate::utils::types::study_history::StudyHistoryCreate;

#[function_component(Study)]
pub fn study() -> Html {
    let toast = use_toast();
    let flashcards = use_cards();
    let decks = use_decks();
    let selected_deck_id = use_state(|| -1);
    let is_studying = use_state(|| false);
    let study_cards = use_state(Vec::<StudyFlashcard>::new);
    let completed_cards = use_state(HashSet::<i64>::new);

    let available_cards: Vec<Flashcard> = {
        let all_cards = (*flashcards).clone();
        all_cards.into_iter().filter(|c| c.deck_id == *selected_deck_id).collect()
    };

    let on_start_study = {
        let study_cards = study_cards.clone();
        let is_studying = is_studying.clone();
        let completed_cards = completed_cards.clone();
        let available_cards = available_cards.clone();

        Callback::from(move |_| {
            if available_cards.is_empty() { return; }
            let mut study_session_cards: Vec<StudyFlashcard> = available_cards.iter().map(|card| {
                StudyFlashcard {
                    flashcard: card.clone(),
                    last_reviewed: None,
                }
            }).collect();
            shuffle_cards(&mut study_session_cards);
            study_cards.set(study_session_cards);
            is_studying.set(true);
            completed_cards.set(HashSet::<i64>::new());
        })
    };

    let on_select_change = {
        let selected_deck_id = selected_deck_id.clone();
        Callback::from(move |e: Event| {
            let target: HtmlSelectElement = e.target().unwrap().unchecked_into();
            selected_deck_id.set(target.value().parse().unwrap_or(-1));
        })
    };

    let on_next_card = {
        let completed_cards = completed_cards.clone();
        Callback::from(move |card_id: i64| {
            let mut set = (*completed_cards).clone();
            set.insert(card_id);
            completed_cards.set(set);
        })
    };

    let on_finish_study = {
        let is_studying = is_studying.clone();
        let selected_deck_id = selected_deck_id.clone();
        let toast = toast.clone();

        Callback::from(move |accuracy: u8| {
            let is_studying = is_studying.clone();
            let selected_deck_id = selected_deck_id.clone();
            let toast = toast.clone();

            //Save study history
            spawn_local(async move {
                let study_history = StudyHistoryCreate {
                    deck_id: *selected_deck_id,
                    accuracy
                };
                let resp = Request::post(&format!("{}/api/study-history", api_base()))
                    .credentials(RequestCredentials::Include)
                    .json(&study_history)
                    .expect("Failed to serialize data")
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.ok() => {
                        is_studying.set(false);
                    }
                    Ok(r) => {
                        web_sys::console::error_1(&format!("Error saving history: status {}", r.status()).into());
                        toast.error("Chyba při ukládání historie".to_string());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Network error: {}", e).into());
                        toast.error("Chyba při ukládání historie".to_string());
                    }
                }
            });
        })
    };


    if *is_studying {
        html! {
            <StudySession
                study_cards={(*study_cards).clone()}
                decks={(*decks).clone()}
                on_next={on_next_card}
                on_restart={on_start_study.clone()}
                on_finish={on_finish_study}
            />
        }
    } else {
        html! {
            <StudySetup
                decks={(*decks).clone()}
                available_cards={available_cards.clone()}
                completed_cards={(*completed_cards).clone()}
                selected_deck_id={*selected_deck_id}
                on_select_change={on_select_change}
                on_start={on_start_study}
            />
        }
    }
}