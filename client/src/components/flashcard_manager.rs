// src/components/flashcard_manager.rs
use yew::prelude::*;
use web_sys::{window, HtmlInputElement, HtmlTextAreaElement, HtmlSelectElement};

#[derive(Clone, PartialEq)]
pub struct Deck {
    pub id: String,
    pub name: String,
    /// Hex barva, např. "#6366F1"
    pub color: String,
}

#[derive(Clone, PartialEq)]
pub struct Flashcard {
    pub id: String,
    pub question: String,
    pub answer: String,
    pub deck_id: String,
    pub created_at: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct NewFlashcard {
    pub question: String,
    pub answer: String,
    pub deck_id: String,
}

#[derive(Clone, PartialEq, Default)]
pub struct FlashcardPatch {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub deck_id: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct FlashcardManagerProps {
    pub flashcards: Vec<Flashcard>,
    pub decks: Vec<Deck>,
    /// on_add: dostane novou kartičku bez id/created_at
    pub on_add_flashcard: Callback<NewFlashcard>,
    /// on_update: (id, patch)
    pub on_update_flashcard: Callback<(String, FlashcardPatch)>,
    pub on_delete_flashcard: Callback<String>,
}

#[function_component(FlashcardManager)]
pub fn flashcard_manager(props: &FlashcardManagerProps) -> Html {
    // ---------- stav formuláře / dialog ----------
    let is_dialog_open = use_state(|| false);
    let editing_card = use_state(|| Option::<Flashcard>::None);
    let question = use_state(|| String::new());
    let answer = use_state(|| String::new());
    let selected_deck = use_state(|| String::new());

    // ---------- filtrování ----------
    let search_query = use_state(|| String::new());
    let filter_deck = use_state(|| "all".to_string());

    // helpers
    let reset_form = {
        let question = question.clone();
        let answer = answer.clone();
        let selected_deck = selected_deck.clone();
        let editing_card = editing_card.clone();
        Callback::from(move |_| {
            question.set(String::new());
            answer.set(String::new());
            selected_deck.set(String::new());
            editing_card.set(None);
        })
    };

    let submit = {
        let question = question.clone();
        let answer = answer.clone();
        let selected_deck = selected_deck.clone();
        let editing_card = editing_card.clone();
        let on_add = props.on_add_flashcard.clone();
        let on_update = props.on_update_flashcard.clone();
        let reset_form2 = reset_form.clone();
        let is_dialog_open = is_dialog_open.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let q = (*question).trim().to_string();
            let a = (*answer).trim().to_string();
            let d = (*selected_deck).trim().to_string();

            if q.is_empty() || a.is_empty() || d.is_empty() {
                if let Some(w) = window() { let _ = w.alert_with_message("Vyplňte prosím všechna pole"); }
                return;
            }

            if let Some(card) = (*editing_card).as_ref() {
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
                on_add.emit(NewFlashcard { question: q, answer: a, deck_id: d });
                if let Some(w) = window() { let _ = w.alert_with_message("Kartička byla vytvořena"); }
            }

            reset_form2.emit(());
            is_dialog_open.set(false);
        })
    };

    let handle_edit = {
        let editing_card = editing_card.clone();
        let is_dialog_open = is_dialog_open.clone();
        let question = question.clone();
        let answer = answer.clone();
        let selected_deck = selected_deck.clone();

        Callback::from(move |card: Flashcard| {
            question.set(card.question.clone());
            answer.set(card.answer.clone());
            selected_deck.set(card.deck_id.clone());
            editing_card.set(Some(card));
            is_dialog_open.set(true);
        })
    };

    let handle_delete = {
        let on_delete = props.on_delete_flashcard.clone();
        Callback::from(move |id: String| {
            if let Some(w) = window() {
                if w.confirm_with_message("Opravdu chcete smazat tuto kartičku?").unwrap_or(false) {
                    on_delete.emit(id);
                    let _ = w.alert_with_message("Kartička byla smazána");
                }
            }
        })
    };

    // odvozené: vyfiltrované kartičky
    let filtered: Vec<Flashcard> = props
        .flashcards
        .iter()
        .cloned()
        .filter(|c| {
            let sq = search_query.to_lowercase();
            let matches_search = c.question.to_lowercase().contains(&sq) || c.answer.to_lowercase().contains(&sq);
            let matches_deck = (*filter_deck == "all") || (c.deck_id == *filter_deck);
            matches_search && matches_deck
        })
        .collect();

    // pomocník: najít deck
    let get_deck = |id: &str| -> Option<Deck> { props.decks.iter().find(|d| d.id == id).cloned() };

    // ---------- vstupy / handlery ----------
    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let on_filter_deck = {
        let filter_deck = filter_deck.clone();
        Callback::from(move |e: Event| {
            let el: HtmlSelectElement = e.target_unchecked_into();
            filter_deck.set(el.value());
        })
    };

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

    // otevření / zavření dialogu
    let open_new = {
        let is_dialog_open = is_dialog_open.clone();
        Callback::from(move |_| is_dialog_open.set(true))
    };
    let open_new_header_btn = open_new.clone();
    let open_new_empty_btn  = open_new.clone();
    let close_dialog = {
        let is_dialog_open = is_dialog_open.clone();
        let reset_form = reset_form.clone();
        Callback::from(move |_| {
            reset_form.emit(());
            is_dialog_open.set(false);
        })
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

            <div class="grid md:grid-cols-2 gap-4 mb-6">
                <div class="relative">
                    <svg class="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-gray-400"
                        xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M21 21l-4.35-4.35M10 18a8 8 0 100-16 8 8 0 000 16z"/>
                    </svg>
                    <input
                        value={(*search_query).clone()}
                        oninput={on_search}
                        placeholder="Hledat v kartičkách..."
                        class="w-full rounded-lg border border-gray-200 bg-gray-50 pl-10 pr-3 py-2 outline-none
                               focus:bg-white focus:border-gray-300"
                        />
                </div>

                <select onchange={on_filter_deck}
                        value={(*filter_deck).clone()}
                        class="w-full rounded-lg border border-gray-200 bg-gray-50 px-3 py-2 outline-none
                               focus:bg-white focus:border-gray-300">
                    <option value="all">{"Všechny balíčky"}</option>
                    { for props.decks.iter().map(|d| html!{
                        <option value={d.id.clone()}>{ d.name.clone() }</option>
                    }) }
                </select>
            </div>

            {
                if filtered.is_empty() {
                    html! {
                        <div class="p-12 text-center rounded-xl border border-gray-200 bg-white/80 backdrop-blur">
                            <p class="text-gray-500 mb-4">
                                {
                                    if !search_query.is_empty() || *filter_deck != "all" {
                                        "Žádné kartičky neodpovídají filtrům"
                                    } else {
                                        "Zatím nemáte žádné kartičky"
                                    }
                                }
                            </p>
                            {
                                if search_query.is_empty() && *filter_deck == "all" {
                                    html!{
                                        <button onclick={open_new_empty_btn}
                                            class="inline-flex items-center rounded-lg px-4 py-2 font-medium
                                                   border border-gray-300 hover:bg-gray-50">
                                            <svg class="mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none"
                                                viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                    d="M12 4v16m8-8H4"/>
                                            </svg>
                                            {"Vytvořit první kartičku"}
                                        </button>
                                    }
                                } else { html!{} }
                            }
                        </div>
                    }
                } else {
                    html!{
                        <div class="grid gap-4">
                            { for filtered.into_iter().map(|card| {
                                let deck = get_deck(&card.deck_id);
                                let edit = {
                                    let handle_edit = handle_edit.clone();
                                    let c = card.clone();
                                    Callback::from(move |_| handle_edit.emit(c.clone()))
                                };
                                let delete = {
                                    let handle_delete = handle_delete.clone();
                                    let id = card.id.clone();
                                    Callback::from(move |_| handle_delete.emit(id.clone()))
                                };

                                html!{
                                    <div class="p-6 rounded-2xl border border-gray-200 bg-white/80 backdrop-blur">
                                        <div class="flex items-start justify-between gap-4">
                                            <div class="flex-1">
                                                <div class="flex items-center gap-2 mb-3">
                                                    {
                                                        if let Some(d) = deck.clone() {
                                                            let bg = format!("{}20", d.color);
                                                            let style_str = format!("background-color:{};color:{};", bg, d.color);
                                                            html!{
                                                                <span class="px-3 py-1 rounded-full text-sm" style={style_str}>
                                                                    { d.name }
                                                                </span>
                                                            }
                                                        } else { html!{} }
                                                    }
                                                </div>

                                                <div class="mb-3">
                                                    <p class="text-sm text-gray-500 mb-1">{"Otázka:"}</p>
                                                    <p class="text-gray-900">{ card.question.clone() }</p>
                                                </div>

                                                <div>
                                                    <p class="text-sm text-gray-500 mb-1">{"Odpověď:"}</p>
                                                    <p class="text-gray-700">{ card.answer.clone() }</p>
                                                </div>
                                            </div>

                                            <div class="flex gap-2">
                                                <button onclick={edit}
                                                    class="inline-flex items-center justify-center rounded-lg px-3 py-2
                                                           border border-gray-300 hover:bg-gray-50">
                                                    <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none"
                                                        viewBox="0 0 24 24" stroke="currentColor">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                            d="M15.232 5.232l3.536 3.536M4 20h4l10.5-10.5a2.5 2.5 0 00-3.536-3.536L4 16v4z"/>
                                                    </svg>
                                                </button>
                                                <button onclick={delete}
                                                    class="inline-flex items-center justify-center rounded-lg px-3 py-2
                                                           border border-gray-300 hover:bg-gray-50">
                                                    <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none"
                                                        viewBox="0 0 24 24" stroke="currentColor">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                                            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5-3h4m-4 0a1 1 0 00-1 1v1h6V5a1 1 0 00-1-1m-4 0h4"/>
                                                    </svg>
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }) }
                        </div>
                    }
                }
            }

            {
                if *is_dialog_open {
                    html!{
                        <div class="fixed inset-0 z-50">
                            <div class="absolute inset-0 bg-black/40" onclick={close_dialog.clone()}></div>
                            <div class="absolute inset-0 flex items-center justify-center p-4">
                                <div class="w-full max-w-lg rounded-2xl bg-white p-6 shadow-2xl">
                                    <div class="mb-4">
                                        <h2 class="text-lg font-semibold">
                                            {
                                                if editing_card.is_some() { "Upravit kartičku" } else { "Nová kartička" }
                                            }
                                        </h2>
                                    </div>

                                    <form onsubmit={submit.clone()} class="space-y-4">
                                        <div>
                                            <label class="block text-sm mb-1">{"Balíček"}</label>
                                            <select onchange={on_sel_deck.clone()}
                                                    value={(*selected_deck).clone()}
                                                    class="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none
                                                           focus:border-gray-400">
                                                <option value="" disabled={true}>{"Vyberte balíček"}</option>
                                                { for props.decks.iter().map(|d| html!{
                                                    <option value={d.id.clone()}>{ d.name.clone() }</option>
                                                }) }
                                            </select>
                                        </div>

                                        <div>
                                            <label class="block text-sm mb-1" for="q">{"Otázka"}</label>
                                            <textarea id="q"
                                                value={(*question).clone()}
                                                oninput={on_q_input.clone()}
                                                rows={3}
                                                placeholder="Zadejte otázku..."
                                                class="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none
                                                       focus:border-gray-400" />
                                        </div>

                                        <div>
                                            <label class="block text-sm mb-1" for="a">{"Odpověď"}</label>
                                            <textarea id="a"
                                                value={(*answer).clone()}
                                                oninput={on_a_input.clone()}
                                                rows={3}
                                                placeholder="Zadejte odpověď..."
                                                class="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none
                                                       focus:border-gray-400" />
                                        </div>

                                        <div class="flex gap-2">
                                            <button type="submit"
                                                class="flex-1 inline-flex items-center justify-center rounded-lg px-4 py-2
                                                       bg-neutral-900 text-white font-medium hover:bg-black">
                                                {
                                                    if editing_card.is_some() { "Uložit změny" } else { "Vytvořit" }
                                                }
                                            </button>
                                            <button type="button" onclick={close_dialog}
                                                class="inline-flex items-center justify-center rounded-lg px-4 py-2
                                                       border border-gray-300 hover:bg-gray-50">
                                                {"Zrušit"}
                                            </button>
                                        </div>
                                    </form>
                                </div>
                            </div>
                        </div>
                    }
                } else { html!{} }
            }
        </div>
    }
}
