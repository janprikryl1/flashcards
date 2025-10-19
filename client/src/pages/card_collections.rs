use yew::prelude::*;
use crate::components::collection_edit_modal::CollectionEditModal;
use crate::components::icons::plus_icon::PlusIcon;
use crate::components::my_card_collection::MyCardCollections;
use crate::utils::deck::Deck;


#[function_component(CardCollections)]
pub fn card_collections() -> Html {
    let flashcard_len = use_state(|| 1);
    let is_dialog_open = use_state(|| false);
    let editing_collection = use_state(|| Option::<Deck>::None);
    let decks = use_state(|| vec![
        Deck {
            id: "1".to_string(),
            name: "Základní balíček".to_string(),
            description: "xxx".to_string(),
            color: "FFFFFF".to_string(),
        },
        Deck {
            id: "2".to_string(),
            name: "Pokročilý balíček".to_string(),
            description: "yyy".to_string(),
            color: "000000".to_string(),
        },
    ]);

    let on_new = {
        let is_dialog_open = is_dialog_open.clone();
        let editing_card = editing_collection.clone();
        Callback::from(move |_| {
            editing_card.set(None);
            is_dialog_open.set(true);
        })
    };

    let handle_edit = {
        let editing_collection = editing_collection.clone();
        let is_dialog_open = is_dialog_open.clone();
        Callback::from(move |collection: Deck| {
            editing_collection.set(Some(collection));
            is_dialog_open.set(true);
        })
    };

    let handle_delete = {
//        let on_delete = on_delete_flashcard.clone();
 //       Callback::from(move |id: String| on_delete.emit(id))
        Callback::from(move |id: String| println!("{id}"))
    };

    let close_dialog = {
        let is_dialog_open = is_dialog_open.clone();
        let editing_card = editing_collection.clone();
        Callback::from(move |_| {
            editing_card.set(None);
            is_dialog_open.set(false);
        })
    };

    let on_submit_new = {
        Callback::from(move |new_card: Deck | println!("on submit new") )
    };

    let on_submit_update = {
        //let on_update = props.on_update_flashcard.clone();
        //Callback::from(move |payload: (String, FlashcardPatch)| on_update.emit(payload))
        Callback::from(move |payload: (String, Deck)| println!("{}", "payload"))
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
                        <h1 class="text-3xl mb-2">{"Balíčky karet"}</h1>
                        <p class="text-gray-600">{ format!("Celkem {} balíčků", flashcard_len.to_string()) }</p>
                    </div>

                    <button onclick={open_new_header_btn}
                        class="inline-flex items-center rounded-lg px-4 py-2 font-medium text-white
                               bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 shadow">
                        <div class="mr-2 h-5 w-5">
                            <PlusIcon />
                        </div>
                        {"Nový balíček"}
                    </button>
                </div>

                <MyCardCollections
                    decks={(*decks).clone()}
                    on_update_deck={handle_edit}
                    on_delete_deck={handle_delete}
                />

                { if *is_dialog_open {
                    html!{
                        <CollectionEditModal
                            on_close={close_dialog.clone()}
                            editing_collection={(*editing_collection).clone()}
                            on_submit_new={on_submit_new}
                            on_submit_update={on_submit_update}
                        />
                    }
                } else { html!{} } }
            </div>
        </div>
    }
}