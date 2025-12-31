use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, Html, Properties, TargetCast};
use crate::components::reusable::badge::Badge;
use crate::components::study::flashcard_view_answer::FlashcardViewAnswer;
use crate::utils::types::deck::Deck;
use crate::utils::types::flashcard::StudyFlashcard;

#[derive(Properties, PartialEq)]
pub struct FlashcardViewProps {
    pub card: StudyFlashcard,
    pub deck: Option<Deck>,
    pub show_answer: bool,
    pub user_answer: yew::UseStateHandle<String>,
}

#[function_component(FlashcardView)]
pub fn flashcard_view(props: &FlashcardViewProps) -> Html {
    let on_answer_change = {
        let user_answer = props.user_answer.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input_element) = e.target_dyn_into::<HtmlInputElement>() {
                user_answer.set(input_element.value());
            }
        })
    };

    html! {
        <div class="p-12 min-h-[400px] flex flex-col justify-center items-center transition-all hover:shadow-lg border rounded-xl">
            <div class="mb-6">
                {
                    if let Some(d) = &props.deck {
                        html!{ <Badge name={format!("Balíček {}", d.name.clone())} color={d.color.clone()} /> }
                    } else {
                        html!{}
                    }
                }
            </div>
            <div class="text-center mb-8">
                <p class="text-sm text-gray-500 mb-3">{ if props.show_answer { "Otázka:" } else { "" } }</p>
                <p class="text-2xl text-gray-900 mb-8">{ props.card.flashcard.question.clone() }</p>
                <input
                    id="answer"
                    placeholder="Odpověď"
                    type="text"
                    required=true
                    value={props.user_answer.to_string()}
                    oninput={on_answer_change}
                    disabled={props.show_answer}
                    class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500 text-lg"
                />
                {
                    if props.show_answer {
                        html!{
                            <FlashcardViewAnswer
                                card={props.card.clone()}
                                user_answer={props.user_answer.clone()}
                            />
                        }
                    } else {
                        html!{}
                    }
                }
            </div>
        </div>
    }
}