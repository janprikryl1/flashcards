use yew::prelude::*;
use web_sys::{window, HtmlTextAreaElement, HtmlSelectElement};
use crate::utils::deck::Deck;
use crate::utils::flashcard::{Flashcard, FlashcardPatch, NewFlashcard};

#[derive(Properties, PartialEq)]
pub struct CardEditModalProps {
    pub on_close: Callback<()>,
    pub editing_card: Option<Flashcard>,
    pub decks: Vec<Deck>,
    pub on_submit_new: Callback<NewFlashcard>,
    pub on_submit_update: Callback<(String, FlashcardPatch)>,
}

#[function_component(CardEditModal)]
pub fn card_edit_modal(props: &CardEditModalProps) -> Html {
    let question = use_state(|| String::new());
    let answer = use_state(|| String::new());
    let selected_deck = use_state(|| String::new());

    {
        let question = question.clone();
        let answer = answer.clone();
        let selected_deck = selected_deck.clone();

        use_effect_with(props.editing_card.clone(), move |editing| {
            if let Some(c) = editing {
                question.set(c.question.clone());
                answer.set(c.answer.clone());
                selected_deck.set(c.deck_id.clone());
            } else {
                question.set(String::new());
                answer.set(String::new());
                selected_deck.set(String::new());
            }
            || ()
        });
    }

    let on_sel_deck = {
        let selected_deck = selected_deck.clone();
        Callback::from(move |e: Event| {
            let el: HtmlSelectElement = e.target_unchecked_into();
            selected_deck.set(el.value());
        })
    };

    let on_q_input = {
        let question = question.clone();
        Callback::from(move |e: InputEvent| {
            let el: HtmlTextAreaElement = e.target_unchecked_into();
            question.set(el.value());
        })
    };

    let on_a_input = {
        let answer = answer.clone();
        Callback::from(move |e: InputEvent| {
            let el: HtmlTextAreaElement = e.target_unchecked_into();
            answer.set(el.value());
        })
    };

    let submit = {
        let question = question.clone();
        let answer = answer.clone();
        let selected_deck = selected_deck.clone();
        let editing_card = props.editing_card.clone();
        let on_new = props.on_submit_new.clone();
        let on_update = props.on_submit_update.clone();
        let on_close = props.on_close.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let q = (*question).trim().to_string();
            let a = (*answer).trim().to_string();
            let d = (*selected_deck).trim().to_string();

            if q.is_empty() || a.is_empty() || d.is_empty() {
                if let Some(w) = window() { let _ = w.alert_with_message("Vyplňte prosím všechna pole"); }
                return;
            }

            if let Some(card) = &editing_card {
                on_update.emit((
                    card.id.clone(),
                    FlashcardPatch {
                        question: Some(q),
                        answer: Some(a),
                        deck_id: Some(d),
                    },
                ));
                if let Some(w) = window() { let _ = w.alert_with_message("Kartička byla aktualizována"); }
            } else {
                on_new.emit(NewFlashcard { question: q, answer: a, deck_id: d });
                if let Some(w) = window() { let _ = w.alert_with_message("Kartička byla vytvořena"); }
            }

            on_close.emit(());
        })
    };

    let close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    html! {
        <div class="fixed inset-0 z-50">
            <div class="absolute inset-0 bg-black/40" onclick={close_click.clone()}></div>
            <div class="absolute inset-0 flex items-center justify-center p-4">
                <div class="w-full max-w-lg rounded-2xl bg-white p-6 shadow-2xl">
                    <div class="mb-4">
                        <h2 class="text-lg font-semibold">
                            { if props.editing_card.is_some() { "Upravit kartičku" } else { "Nová kartička" } }
                        </h2>
                    </div>

                    <form onsubmit={submit} class="space-y-4">
                        <div>
                            <label class="block text-sm mb-1">{"Balíček"}</label>
                            <select
                                onchange={on_sel_deck}
                                value={(*selected_deck).clone()}
                                class="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none focus:border-gray-400"
                            >
                                <option value="" disabled=true>{"Vyberte balíček"}</option>
                                { for props.decks.iter().map(|d| html!{
                                    <option value={d.id.clone()}>{ d.name.clone() }</option>
                                }) }
                            </select>
                        </div>

                        <div>
                            <label class="block text-sm mb-1" for="q">{"Otázka"}</label>
                            <textarea
                                id="q"
                                value={(*question).clone()}
                                oninput={on_q_input}
                                rows={3}
                                placeholder="Zadejte otázku..."
                                class="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none focus:border-gray-400"
                            />
                        </div>

                        <div>
                            <label class="block text-sm mb-1" for="a">{"Odpověď"}</label>
                            <textarea
                                id="a"
                                value={(*answer).clone()}
                                oninput={on_a_input}
                                rows={3}
                                placeholder="Zadejte odpověď..."
                                class="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none focus:border-gray-400"
                            />
                        </div>

                        <div class="flex gap-2">
                            <button
                                type="submit"
                                class="flex-1 inline-flex items-center justify-center rounded-lg px-4 py-2 bg-neutral-900 text-white font-medium hover:bg-black"
                            >
                                { if props.editing_card.is_some() { "Uložit změny" } else { "Vytvořit" } }
                            </button>
                            <button
                                type="button"
                                onclick={close_click}
                                class="inline-flex items-center justify-center rounded-lg px-4 py-2 border border-gray-300 hover:bg-gray-50"
                            >
                                {"Zrušit"}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
