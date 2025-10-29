use std::collections::HashSet;
use yew::{classes, function_component, html, Callback, Html, Properties};
use crate::utils::types::deck::Deck;
use crate::utils::types::flashcard::Flashcard;
use web_sys::{Event};
use crate::utils::functions::download_file;

#[derive(Properties, PartialEq)]
pub struct StudySetupProps {
    pub decks: Vec<Deck>,
    pub available_cards: Vec<Flashcard>,
    pub completed_cards: HashSet<String>,
    pub selected_deck_id: String,
    pub on_select_change: Callback<Event>,
    pub on_start: Callback<()>,
}

#[function_component(StudySetup)]
pub fn study_setup(props: &StudySetupProps) -> Html {
    let available_cards_count = props.available_cards.len();

    let on_start = {
        let on_start_prop = props.on_start.clone();
        Callback::from(move |_| on_start_prop.emit(()))
    };

    let on_export = {
        let available_cards = props.available_cards.clone();
        Callback::from(move |_| {
            let data = serde_json::to_string(&available_cards);
            match data {
                Ok(json) => {
                    download_file(json, "flashcards_export.json".to_string(), "application/json".to_string());
                },
                Err(e) => {
                    web_sys::console::error_1(&format!("Chyba při exportu: {}", e).into());
                }
            }
        })
    };

    html! {
        <div class="container mx-auto px-4 py-8">
            <div class="max-w-2xl mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-3xl mb-2">{"Režim učení"}</h1>
                    <p class="text-gray-600">{"Procvičujte své kartičky v náhodném pořadí"}</p>
                </div>

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
                                    <option selected={true} disabled={true}>{"Vyberte balíček"}</option>
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
                        <button
                            onclick={on_export}
                            class={classes!(
                                "w-full",
                                "bg-gray-300",
                                "py-3",
                                if available_cards_count == 0 { "opacity-50 cursor-not-allowed" } else { "hover:opacity-90" }
                            )}
                        >
                            {"Exportovat otázky do JSON"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}