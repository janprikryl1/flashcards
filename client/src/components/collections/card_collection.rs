use yew::prelude::*;
use crate::components::icons::pencil_icon::PencilIcon;
use crate::components::icons::trash_icon::TrashIcon;
use crate::utils::types::deck::Deck;

#[derive(Properties, PartialEq, Clone)]
pub struct MyCardCollectionProps {
    pub deck: Deck,
    pub on_update_deck: Callback<Deck>,
    pub on_delete_deck: Callback<i64>,
}

#[function_component(MyCardCollection)]
pub fn my_card_collection(props: &MyCardCollectionProps) -> Html {
    let deck = props.deck.clone();

    let edit_click = {
        let on_update = props.on_update_deck.clone();
        let deck = deck.clone();
        Callback::from(move |_| on_update.emit(deck.clone()))
    };

    let delete_click = {
        let on_delete = props.on_delete_deck.clone();
        Callback::from(move |_| on_delete.emit(deck.id))
    };

    let count = 1;
    let color = if deck.color.is_empty() { "#3B82F6".into() } else { format!("#{}", deck.color.trim_start_matches('#')) };
    let top_bar_style = format!("height:12px;background-color:{color}");

    html! {
        <div class="overflow-hidden rounded-xl border border-gray-200 bg-white hover:shadow-lg transition-shadow">
            <div style={top_bar_style}></div>
            <div class="p-6">
                <div class="flex items-start justify-between mb-4">
                    <div class="flex-1">
                        <h3 class="text-lg font-semibold mb-2">{ deck.name.clone() }</h3>
                        if !deck.description.is_empty() {
                            <p class="text-sm text-gray-600 mb-3">{ deck.description.clone() }</p>
                        }
                        <p class="text-sm text-gray-500">{ format!("Počet balíčků: {}", count) }</p>
                    </div>
                </div>

                <div class="flex gap-2">
                    <button
                        class="flex-1 inline-flex items-center justify-center h-9 rounded-md border px-3 text-sm transition-colors hover:bg-gray-50"
                        onclick={edit_click}
                    >
                        <PencilIcon />
                        <span class="ml-2">{ "Upravit" }</span>
                    </button>
                    <button
                        class="inline-flex items-center justify-center h-9 w-9 rounded-md border px-2 text-sm transition-colors hover:bg-red-50"
                        onclick={delete_click}
                        aria-label="Smazat"
                    >
                        <TrashIcon />
                    </button>
                </div>
            </div>
        </div>
    }
}