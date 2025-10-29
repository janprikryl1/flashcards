use yew::prelude::*;
use crate::components::collections::card_collection::MyCardCollection;
use crate::utils::types::deck::Deck;

#[derive(Properties, PartialEq)]
pub struct MyCardCollectionProps {
    pub decks: Vec<Deck>,
    pub on_update_deck: Callback<Deck>,
    pub on_delete_deck: Callback<String>,
}

#[function_component(MyCardCollections)]
pub fn card_collections(props: &MyCardCollectionProps) -> Html {

    html! {
        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            { for props.decks.iter().map(|d| html!{
                <MyCardCollection
                    deck={d.clone()}
                    on_update_deck={props.on_update_deck.clone()}
                    on_delete_deck={props.on_delete_deck.clone()}
                />
            }) }
        </div>
    }
}