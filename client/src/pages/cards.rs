use yew::prelude::*;
use crate::components::flashcard_manager::{FlashcardManager};
use crate::utils::deck::Deck;
use crate::utils::flashcard::{Flashcard, FlashcardPatch, NewFlashcard};

#[function_component(Cards)]
pub fn cards() -> Html {
    // --- demo data ---
    let decks = use_state(|| vec![
        Deck { id: "basic".into(), name: "Základní balíček".into(), color: "#6366F1".into() },
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
                id: js_sys::Date::now().to_string(),
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

    html! {
        <div class="bg-slate-50 min-h-screen">
            <FlashcardManager
                flashcards={(*flashcards).clone()}
                decks={(*decks).clone()}
                on_add_flashcard={on_add}
                on_update_flashcard={on_update}
                on_delete_flashcard={on_delete}
            />
        </div>
    }
}
