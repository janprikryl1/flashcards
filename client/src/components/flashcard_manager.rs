use yew::prelude::*;
use crate::components::card_edit_modal::CardEditModal;
use crate::components::my_cards::MyCards;
use crate::utils::deck::Deck;
use crate::utils::flashcard::{Flashcard, FlashcardPatch, NewFlashcard};

#[derive(Properties, PartialEq)]
pub struct FlashcardManagerProps {
    pub flashcards: Vec<Flashcard>,
    pub decks: Vec<Deck>,
    pub on_add_flashcard: Callback<NewFlashcard>,
    pub on_update_flashcard: Callback<(String, FlashcardPatch)>,
    pub on_delete_flashcard: Callback<String>,
}

#[function_component(FlashcardManager)]
pub fn flashcard_manager(props: &FlashcardManagerProps) -> Html {
    let is_dialog_open = use_state(|| false);
    let editing_card = use_state(|| Option::<Flashcard>::None);

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

    let handle_delete = {
        let on_delete = props.on_delete_flashcard.clone();
        Callback::from(move |id: String| on_delete.emit(id))
    };

    let close_dialog = {
        let is_dialog_open = is_dialog_open.clone();
        let editing_card = editing_card.clone();
        Callback::from(move |_| {
            editing_card.set(None);
            is_dialog_open.set(false);
        })
    };

    let on_submit_new = {
        let on_add = props.on_add_flashcard.clone();
        Callback::from(move |new_card: NewFlashcard| on_add.emit(new_card))
    };

    let on_submit_update = {
        let on_update = props.on_update_flashcard.clone();
        Callback::from(move |payload: (String, FlashcardPatch)| on_update.emit(payload))
    };

    let open_new_header_btn = {
        let on_new = on_new.clone();
        Callback::from(move |_| on_new.emit(()))
    };

    html! {
        <div class="container mx-auto px-4 py-8">
            <div class="flex items-center justify-between mb-8">
                <div>
                    <h1 class="text-3xl mb-2">{"Správa kartiček"}</h1>
                    <p class="text-gray-600">{ format!("Celkem {} kartiček", props.flashcards.len()) }</p>
                </div>

                <button onclick={open_new_header_btn}
                    class="inline-flex items-center rounded-lg px-4 py-2 font-medium text-white
                           bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 shadow">
                    <svg class="mr-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none"
                        viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M12 4v16m8-8H4"/>
                    </svg>
                    {"Nová kartička"}
                </button>
            </div>

            <MyCards
                flashcards={props.flashcards.clone()}
                decks={props.decks.clone()}
                on_new={on_new.clone()}
                on_edit={handle_edit.clone()}
                on_delete={handle_delete.clone()}
            />

            { if *is_dialog_open {
                html!{
                    <CardEditModal
                        on_close={close_dialog.clone()}
                        editing_card={(*editing_card).clone()}
                        decks={props.decks.clone()}
                        on_submit_new={on_submit_new.clone()}
                        on_submit_update={on_submit_update.clone()}
                    />
                }
            } else { html!{} } }
        </div>
    }
}
