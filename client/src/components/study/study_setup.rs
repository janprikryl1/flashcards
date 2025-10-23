use std::collections::HashSet;
use yew::{classes, function_component, html, Callback, Html, Properties};
use crate::utils::deck::Deck;
use crate::utils::flashcard::Flashcard;
use web_sys::{Event};

#[derive(Properties, PartialEq)]
pub struct StudySetupProps {
    pub decks: Vec<Deck>,
    pub available_cards: Vec<Flashcard>,
    pub completed_cards: HashSet<String>,
    pub selected_deck_id: String,
    pub on_select_change: Callback<Event>,
    pub on_start: Callback<()>,
    pub on_restart: Callback<()>,
}

fn pluralize_cards(n: usize) -> &'static str {
    if n == 1 { "kartičku" } else if n < 5 { "kartičky" } else { "kartiček" }
}

#[function_component(StudySetup)]
pub fn study_setup(props: &StudySetupProps) -> Html {
    let available_cards_count = props.available_cards.len();
    let completed_count = props.completed_cards.len();

    let on_start = {
        let on_start_prop = props.on_start.clone();
        Callback::from(move |_| on_start_prop.emit(()))
    };

    let on_restart = {
        let on_restart_prop = props.on_restart.clone();
        Callback::from(move |_| on_restart_prop.emit(()))
    };

    html! {
        <div class="container mx-auto px-4 py-8">
            <div class="max-w-2xl mx-auto">
                <div class="text-center mb-8">
                    <div class="h-16 w-16 mx-auto mb-4 text-purple-600 text-5xl">{"🧠"}</div>
                    <h1 class="text-3xl mb-2">{"Režim učení"}</h1>
                    <p class="text-gray-600">{"Procvičujte své kartičky v náhodném pořadí"}</p>
                </div>

                { if completed_count > 0 {
                    html!{
                        <div class="p-8 mb-6 bg-gradient-to-r from-green-50 to-emerald-50 border border-green-200 rounded-xl">
                            <div class="text-center">
                                <div class="h-16 w-16 mx-auto mb-4 text-green-600 text-5xl">{"✅"}</div>
                                <h2 class="text-2xl mb-2 text-green-900">{"Výborně!"}</h2>
                                <p class="text-green-700 mb-6">
                                    { format!("Procvičili jste {} {}", completed_count, pluralize_cards(completed_count)) }
                                </p>
                                <button
                                    onclick={on_restart}
                                    class="inline-flex items-center gap-2 px-4 py-2 rounded-md bg-green-600 hover:bg-green-700 text-white"
                                >
                                    <span>{"Studovat znovu"}</span>
                                </button>
                            </div>
                        </div>
                    }
                } else { html!{} }}

                <div class="p-8 border rounded-xl shadow-sm">
                    <div class="space-y-6">
                        <div>
                            <label class="block mb-2">{"Vyberte balíček"}</label>
                            <div class="relative">
                                <select
                                    class="w-full rounded-md border px-3 py-2"
                                    value={props.selected_deck_id.clone()}
                                    onchange={props.on_select_change.clone()}
                                >
                                    <option value="all">{"Všechny balíčky"}</option>
                                    { props.decks.iter().map(|deck|
                                        html!{
                                            <option value={deck.id.clone()}>
                                                { deck.name.clone() }
                                            </option> }
                                        ).collect::<Html>()
                                    }
                                </select>
                            </div>
                        </div>
                        <div class="p-4 bg-gray-50 rounded-lg">
                            <p class="text-sm text-gray-600">
                                {"K dispozici: "}
                                <span class="text-gray-900">{ available_cards_count }</span>
                                { " " }
                                { pluralize_cards(available_cards_count) }
                            </p>
                        </div>
                        <button
                            onclick={on_start}
                            disabled={available_cards_count == 0}
                            class={classes!(
                                "w-full",
                                "flex",
                                "items-center",
                                "justify-center",
                                "gap-2",
                                "px-4",
                                "py-3",
                                "rounded-md",
                                "bg-purple-600",
                                "text-white",
                                "text-lg",
                                if available_cards_count == 0 { "opacity-50 cursor-not-allowed" } else { "hover:opacity-90" })}
                        >
                            <span>{"Začít studovat"}</span>
                        </button>
                        <button class="w-full bg-gray-300 py-3">{"Exportovat otázky do JSON"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}