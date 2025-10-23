use yew::prelude::*;
use crate::components::cards::card_edit_modal::CardEditModal;
use crate::components::cards::my_cards::MyCards;
use crate::components::icons::plus_icon::PlusIcon;
use crate::utils::functions::generate_id;
use crate::utils::deck::Deck;
use crate::utils::flashcard::{Flashcard, FlashcardPatch, NewFlashcard};

#[function_component(Cards)]
pub fn cards() -> Html {
    let is_dialog_open = use_state(|| false);
    let editing_card = use_state(|| Option::<Flashcard>::None);

    // --- demo data ---
    let decks = use_state(|| vec![
        Deck { id: "basic".into(), name: "Základní balíček".into(), description: "popis...".to_string(), color: "#6366F1".into() },
    ]);

    let flashcards = use_state(|| vec![
        Flashcard {
            id: "c1".into(),
            question: "Kolik je 1+1".into(),
            answer: "2".into(),
            deck_id: "basic".into(),
            created_at: None,
        }
    ]);

    let on_add = {
        let flashcards = flashcards.clone();
        Callback::from(move |new_card: NewFlashcard| {
            let mut list = (*flashcards).clone();

            list.push(Flashcard {
                id: generate_id(),
                question: new_card.question,
                answer: new_card.answer,
                deck_id: new_card.deck_id,
                created_at: None,
            });
            flashcards.set(list);
        })
    };

    let on_update = {
        let flashcards = flashcards.clone();
        Callback::from(move |(id, patch): (String, FlashcardPatch)| {
            let mut list = (*flashcards).clone();
            if let Some(card) = list.iter_mut().find(|c| c.id == id) {
                if let Some(q) = patch.question { card.question = q; }
                if let Some(a) = patch.answer { card.answer = a; }
                if let Some(d) = patch.deck_id { card.deck_id = d; }
            }
            flashcards.set(list);
        })
    };

    let on_delete = {
        let flashcards = flashcards.clone();
        Callback::from(move |id: String| {
            let mut list = (*flashcards).clone();
            list.retain(|c| c.id != id);
            flashcards.set(list);
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
