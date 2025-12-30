use std::collections::HashSet;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use crate::components::study::study_session::StudySession;
use crate::components::study::study_setup::StudySetup;
use crate::utils::types::deck::Deck;
use crate::utils::types::flashcard::{Flashcard, StudyFlashcard};

fn shuffle_cards(cards: &mut [StudyFlashcard]) {
    let len = cards.len();
    if len == 0 {
        return;
    }
    for i in (1..len).rev() {
        let random_idx = (js_sys::Math::random() * (i + 1) as f64).floor() as usize;
        cards.swap(i, random_idx);
    }
}

#[function_component(Study)]
pub fn study() -> Html {
    let flashcards = use_state(Vec::<Flashcard>::new);
    let decks = use_state(Vec::new);
    let selected_deck_id = use_state(|| -1);
    let is_studying = use_state(|| false);
    let study_cards = use_state(Vec::<StudyFlashcard>::new);
    let completed_cards = use_state(HashSet::<i64>::new);

    {
        let flashcards_handle = flashcards.clone();
        let decks_handle = decks.clone();
        use_effect_with((), move |_| {
            let mock_decks = vec![
                Deck { id: 1, name: "Základy Rustu".to_string(), description: "Klíčové koncepty jazyka Rust.".to_string(), color: "#f59e0b".to_string() },
                Deck { id: 2, name: "Hlavní města".to_string(), description: "Geografický přehled.".to_string(), color: "#3b82f6".to_string() },
                Deck { id: 3, name: "Anglická slovíčka".to_string(), description: "Základní slovní zásoba.".to_string(), color: "#10b981".to_string() },
            ];
            let mock_flashcards = vec![
                Flashcard { id: 101, deck_id: 1, question: "Co je to 'borrowing'?".to_string(), answer: "Půjčování si reference na hodnotu bez převzetí vlastnictví.".to_string(), created_at: Some("2025-10-23T10:00:00Z".to_string()) },
                Flashcard { id: 102, deck_id: 1, question: "Jaký keyword se používá pro proměnlivou proměnnou?".to_string(), answer: "mut".to_string(), created_at: Some("2025-10-23T10:01:00Z".to_string()) },
                Flashcard { id: 201, deck_id: 2, question: "Hlavní město České republiky?".to_string(), answer: "Praha".to_string(), created_at: Some("2025-10-23T10:02:00Z".to_string()) },
            ];
            decks_handle.set(mock_decks);
            flashcards_handle.set(mock_flashcards);
        });
    }

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
        Callback::from(move |_| {
            is_studying.set(false);
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