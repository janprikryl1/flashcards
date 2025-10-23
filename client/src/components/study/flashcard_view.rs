use yew::{function_component, html, Callback, Html, Properties};
use crate::components::reusable::badge::Badge;
use crate::utils::deck::Deck;
use crate::utils::flashcard::StudyFlashcard;

#[derive(Properties, PartialEq)]
pub struct FlashcardViewProps {
    pub card: StudyFlashcard,
    pub deck: Option<Deck>,
    pub show_answer: bool,
    pub on_reveal: Callback<()>,
}

#[function_component(FlashcardView)]
pub fn flashcard_view(props: &FlashcardViewProps) -> Html {
    let onclick = {
        let on_reveal = props.on_reveal.clone();
        let show_answer = props.show_answer;
        Callback::from(move |_| {
            if !show_answer {
                on_reveal.emit(());
            }
        })
    };

    html! {
        <div class="p-12 min-h-[400px] flex flex-col justify-center items-center cursor-pointer transition-all hover:shadow-lg border rounded-xl" {onclick}>
            <div class="mb-6">
                {
                    if let Some(d) = &props.deck {
                        html!{ <Badge name={d.name.clone()} color={d.color.clone()} /> }
                    } else {
                        html!{}
                    }
                }
            </div>
            <div class="text-center mb-8">
                <p class="text-sm text-gray-500 mb-3">{ if props.show_answer { "Otázka:" } else { "" } }</p>
                <p class="text-2xl text-gray-900 mb-8">{ props.card.flashcard.question.clone() }</p>
                {
                    if props.show_answer {
                        html!{
                            <div class="pt-8 border-t border-gray-200">
                                <p class="text-sm text-gray-500 mb-3">{"Odpověď:"}</p>
                                <p class="text-xl text-blue-600">
                                    { props.card.flashcard.answer.clone() }
                                </p>
                            </div>
                        }
                    } else {
                        html!{
                            <p class="text-sm text-gray-500 italic">{"Klikněte pro zobrazení odpovědi"}</p>
                        }
                    }
                }
            </div>
        </div>
    }
}