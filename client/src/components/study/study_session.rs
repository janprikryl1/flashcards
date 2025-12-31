use yew::{function_component, html, use_state, Callback, Html, Properties};
use crate::components::study::flashcard_view::FlashcardView;
use crate::components::study::flashcard_progress::ProgressBar;
use crate::utils::functions::compare_ignore_case;
use crate::utils::types::deck::Deck;
use crate::utils::types::flashcard::StudyFlashcard;

#[derive(Properties, PartialEq)]
pub struct StudySessionProps {
    pub study_cards: Vec<StudyFlashcard>,
    pub decks: Vec<Deck>,
    pub on_next: Callback<i64>,
    pub on_restart: Callback<()>,
    pub on_finish: Callback<u8>,
}

#[function_component(StudySession)]
pub fn study_session(props: &StudySessionProps) -> Html {
    let current_card_index = use_state(|| 0);
    let user_answer = use_state(|| "".to_string());
    let show_answer = use_state(|| false);
    let correct_answer_count = use_state(|| 0);
    let total_answer_count = props.study_cards.len();

    let get_deck_by_id = {
        let decks = props.decks.clone();
        move |id: i64| -> Option<Deck> {
            decks.iter().find(|d| d.id == id).cloned()
        }
    };

    let on_show_answer = {
        let show_answer = show_answer.clone();
        let correct_answer_count = correct_answer_count.clone();
        let user_answer = user_answer.clone();
        let current_card = props.study_cards.get(*current_card_index).cloned();

        Callback::from(move |_| {
            if let Some(card) = &current_card {
                if compare_ignore_case(card.flashcard.answer.clone(), user_answer.to_string()) {
                    correct_answer_count.set(*correct_answer_count + 1);
                }
            }
          show_answer.set(true);
        })
    };

    let on_next_card = {
        let current_card_index = current_card_index.clone();
        let user_answer = user_answer.clone();
        let show_answer = show_answer.clone();
        let study_cards = props.study_cards.clone();
        let on_next_prop = props.on_next.clone();
        let on_finish_prop = props.on_finish.clone();
        let correct_answer_count = correct_answer_count.clone();

        Callback::from(move |_| {
            if let Some(card) = study_cards.get(*current_card_index) {
                on_next_prop.emit(card.flashcard.id);
            }

            if *current_card_index + 1 < study_cards.len() {
                current_card_index.set(*current_card_index + 1);
                user_answer.set("".to_string());
                show_answer.set(false);
            } else {
                let accuracy: u8 = ((*correct_answer_count as f32 / total_answer_count as f32) * 100.0) as u8;
                on_finish_prop.emit(accuracy);
            }
        })
    };

    //AI
    let current_card = (*props.study_cards).get(*current_card_index).cloned();
    let progress = if !props.study_cards.is_empty() {
        ((*current_card_index as f64 + if *show_answer { 1.0 } else { 0.0 }) / props.study_cards.len() as f64) * 100.0
    } else { 0.0 };

    html! {
        <div class="container mx-auto px-4 py-8">
            <div class="max-w-3xl mx-auto">
                <ProgressBar
                    progress={progress}
                    current_card_index={*current_card_index}
                    total_cards={props.study_cards.len()}
                />
                <p>{"Správně: "}{correct_answer_count.to_string()}{"/"}{total_answer_count}</p>
                <div class="mb-8">
                {
                    if let Some(card) = current_card {
                        let deck = get_deck_by_id(card.flashcard.deck_id);
                        html! {
                            <FlashcardView
                                card={card}
                                deck={deck}
                                show_answer={*show_answer}
                                user_answer={user_answer.clone()}
                            />
                        }
                    } else {
                        html!{ <p>{"Chyba: Kartička nenalezena."}</p> }
                    }
                }
                </div>

                <div class="flex gap-4">
                {
                    if !*show_answer {
                        html!{
                            <button
                                onclick={on_show_answer}
                                class="flex-1 px-4 py-3 rounded-md bg-gray-900 text-white text-lg hover:opacity-90"
                            >
                                {"Zkontrolovat"}
                            </button>
                        }
                    } else {
                        html!{
                            <>
                                <button onclick={on_next_card} class="flex-1 px-4 py-3 rounded-md bg-blue-600 text-white text-lg hover:opacity-90">
                                    { if *current_card_index < props.study_cards.len() - 1 {
                                        html!{
                                            <span>{"Další kartička"}</span>
                                        }
                                    }
                                      else { html! {
                                        <span>{"Dokončit"}</span>
                                      } }
                                    }
                                </button>
                            </>
                        }
                    }
                }
                </div>
            </div>
        </div>
    }
}
