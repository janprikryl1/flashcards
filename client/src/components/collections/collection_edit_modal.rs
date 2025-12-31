use tailyew::{Button, ButtonType};
use web_sys::{HtmlTextAreaElement};
use yew::prelude::*;
use crate::components::reusable::toast_provider::use_toast;
use crate::utils::constants::COLLECTION_COLORS;
use crate::utils::types::deck::{Deck, DeckCreate};

#[derive(Properties, PartialEq)]
pub struct CollectionEditModalProps {
    pub on_close: Callback<()>,
    pub editing_collection: Option<Deck>,
    pub on_submit_new: Callback<DeckCreate>,
    pub on_submit_update: Callback<(i64, Deck)>,
}

#[function_component(CollectionEditModal)]
pub fn collection_edit_modal(props: &CollectionEditModalProps) -> Html {
    let toast = use_toast();
    let title = use_state(|| String::new());
    let description = use_state(|| String::new());
    let selected_color = use_state(|| COLLECTION_COLORS[0].to_string());

    {
        let title = title.clone();
        let description = description.clone();
        let selected_color = selected_color.clone();

        use_effect_with(props.editing_collection.clone(), move |editing| {
            if let Some(c) = editing {
                title.set(c.name.clone());
                description.set(c.description.clone());
                if !c.color.is_empty() {
                    selected_color.set(c.color.clone());
                } else {
                    selected_color.set(COLLECTION_COLORS[0].to_string());
                }
            } else {
                title.set(String::new());
                description.set(String::new());
                selected_color.set(COLLECTION_COLORS[0].to_string());
            }
            || ()
        });
    }

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let el: HtmlTextAreaElement = e.target_unchecked_into();
            title.set(el.value());
        })
    };

    let on_description_input = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            let el: HtmlTextAreaElement = e.target_unchecked_into();
            description.set(el.value());
        })
    };

    let submit = {
        let title = title.clone();
        let description = description.clone();
        let selected_color = selected_color.clone();
        let editing_collection = props.editing_collection.clone();
        let on_new = props.on_submit_new.clone();
        let on_update = props.on_submit_update.clone();
        let on_close = props.on_close.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let t = (*title).trim().to_string();
            let d = (*description).trim().to_string();

            if t.is_empty() {
                toast.error("Vyplňte prosím název".to_string());
                return;
            }

            if let Some(deck) = &editing_collection {
                on_update.emit((
                    deck.id,
                    Deck {
                        id: deck.id,
                        name: t,
                        description: d,
                        color: (*selected_color).clone(),
                    },
                ));
            } else {
                on_new.emit(DeckCreate {
                    name: t,
                    description: d,
                    color: (*selected_color).clone(),
                });
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
                            { if props.editing_collection.is_some() { "Upravit balíček" } else { "Vytvořit balíček" } }
                        </h2>
                    </div>

                    <form onsubmit={submit} class="space-y-4">
                        <div>
                            <label class="block text-sm mb-1" for="title">{"Název"}</label>
                            <textarea
                                id="title"
                                value={(*title).clone()}
                                oninput={on_title_input}
                                rows={1}
                                placeholder="Zadejte název..."
                                class="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none focus:border-gray-400"
                            />
                        </div>

                        <div>
                            <label class="block text-sm mb-1" for="description">{"Popis (volitelné)"}</label>
                            <textarea
                                id="description"
                                value={(*description).clone()}
                                oninput={on_description_input}
                                rows={3}
                                placeholder="Zadejte popis..."
                                class="w-full rounded-lg border border-gray-300 px-3 py-2 outline-none focus:border-gray-400"
                            />
                        </div>

                        <div>
                            <label class="block text-sm mb-2">{"Barva"}</label>
                            <div class="flex flex-wrap gap-3">
                                {
                                    for COLLECTION_COLORS.iter().map(|c| {
                                        let c_str = c.to_string();
                                        let is_selected = *selected_color == c_str;
                                        let selected_color = selected_color.clone();
                                        html!{
                                            <button
                                                type="button"
                                                class={classes!(
                                                    "h-10", "w-10", "rounded-full", "border", "transition", "focus:outline-none",
                                                    if is_selected { Some("ring-2") } else { None },
                                                    if is_selected { Some("ring-offset-2") } else { None },
                                                    if is_selected { Some("ring-black") } else { None },
                                                )}
                                                style={format!("background-color: {};", c)}
                                                title={c_str.clone()}
                                                aria-label={format!("Vybrat barvu {}", c)}
                                                onclick={Callback::from(move |_| {
                                                    selected_color.set(c_str.clone());
                                                })}
                                            />
                                        }
                                    })
                                }
                            </div>
                        </div>

                        <div class="flex gap-2">
                            <Button
                                button_type={ButtonType::Submit}
                                class="flex-1 inline-flex items-center justify-center rounded-lg px-4 py-2 font-medium bg-success-500"
                            >
                                { if props.editing_collection.is_some() { "Uložit změny" } else { "Vytvořit" } }
                            </Button>
                            <Button
                                button_type={ButtonType::Button}
                                onclick={close_click}
                                class="inline-flex items-center justify-center rounded-lg px-4 py-2 bg-gray-500"
                            >
                                {"Zrušit"}
                            </Button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
