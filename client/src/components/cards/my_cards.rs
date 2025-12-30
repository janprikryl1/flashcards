use yew::prelude::*;
use web_sys::{window, HtmlInputElement, HtmlSelectElement};
use crate::components::icons::pencil_icon::PencilIcon;
use crate::components::icons::trash_icon::TrashIcon;
use crate::utils::types::deck::Deck;
use crate::utils::types::flashcard::Flashcard;

#[derive(Properties, PartialEq)]
pub struct MyCardsProps {
    pub flashcards: Vec<Flashcard>,
    pub decks: Vec<Deck>,
    pub on_new: Callback<()>,
    pub on_edit: Callback<Flashcard>,
    pub on_delete: Callback<i64>,
}

#[function_component(MyCards)]
pub fn my_cards(props: &MyCardsProps) -> Html {
    let search_query = use_state(|| String::new());
    let filter_deck = use_state(|| "all".to_string());

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

    let open_new_empty_btn = {
        let on_new = props.on_new.clone();
        Callback::from(move |_| on_new.emit(()))
    };

    let get_deck = |id: i64| -> Option<Deck> {
        props.decks.iter().find(|d| d.id == id).cloned()
    };

    let filtered: Vec<Flashcard> = props
        .flashcards
        .iter()
        .cloned()
        .filter(|c| {
            let sq = (*search_query).to_lowercase();
            let matches_search = c.question.to_lowercase().contains(&sq)
                || c.answer.to_lowercase().contains(&sq);
            let matches_deck = (*filter_deck == "all") || (c.deck_id == filter_deck.parse::<i64>().unwrap_or(-1));
            matches_search && matches_deck
        })
        .collect();

    html! {
        <>
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
                        <option value={d.id.to_string()}>{ d.name.clone() }</option>
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
                                let deck = get_deck(card.deck_id);

                                let edit = {
                                    let on_edit = props.on_edit.clone();
                                    let c = card.clone();
                                    Callback::from(move |_| on_edit.emit(c.clone()))
                                };

                                let delete = {
                                    let on_delete = props.on_delete.clone();
                                    Callback::from(move |_| {
                                        if let Some(w) = window() {
                                            if w.confirm_with_message("Opravdu chcete smazat tuto kartičku?").unwrap_or(false) {
                                                on_delete.emit(card.id);
                                                let _ = w.alert_with_message("Kartička byla smazána");
                                            }
                                        }
                                    })
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
                                                    <PencilIcon />
                                                </button>
                                                <button onclick={delete}
                                                    class="inline-flex items-center justify-center rounded-lg px-3 py-2
                                                           border border-gray-300 hover:bg-gray-50">
                                                    <TrashIcon />
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
        </>
    }
}
