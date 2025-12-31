use yew::{function_component, html, Html, Properties};
use crate::utils::functions::compare_ignore_case;
use crate::utils::types::flashcard::StudyFlashcard;

#[derive(Properties, PartialEq)]
pub struct FlashcardViewAnswerProps {
    pub card: StudyFlashcard,
    pub user_answer: yew::UseStateHandle<String>,
}
#[function_component(FlashcardViewAnswer)]
pub fn flashcard_view_answer(props: &FlashcardViewAnswerProps) -> Html {
    let is_correct = compare_ignore_case(props.card.flashcard.answer.to_string(), props.user_answer.to_string());

    html! {
        <>
        <div class="pt-8 border-t border-gray-200">
            <p class="text-sm text-gray-500 mb-3">{"Správná odpověď:"}</p>
            <p class="text-xl text-blue-600">{ props.card.flashcard.answer.to_string() }</p>

            { if !is_correct {
                html!{
                    <>
                        <p class="text-sm text-gray-500 mt-6 mb-3">{"Vaše odpověď:"}</p>
                        <p class="text-xl text-red-600">{ props.user_answer.trim().to_string() }</p>
                    </>
                }
            } else {
                html!{}
            }}
        </div>

        <div>
            {if is_correct {
                html!{
                    <p class="mt-4 text-green-600 font-medium">{"Správně, jen tak dál!"}</p>
                }
            } else {
                html!{
                    <p class="mt-4 text-red-600 font-medium">{"Špatně, je potřeba stále procvičovat!"}</p>
                }
            }}
        </div>
        </>
    }
}