use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub progress: f64,
    pub current_card_index: usize,
    pub total_cards: usize,
}

#[function_component(ProgressBar)]
pub fn flashcard_progress(props: &ProgressBarProps) -> Html {

    html! {
        <div class="mb-8">
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm text-gray-600">
                    { format!("Kartička {} z {}", props.current_card_index + 1, props.total_cards) }
                </span>
                <span class="text-sm text-gray-600">
                    { format!("{}%", props.progress.round()) }
                </span>
            </div>
            <div class="h-2 bg-gray-200 rounded">
                <div
                    class="h-2 rounded bg-blue-600"
                    style={format!("width: {:.0}%;", props.progress)}
                />
            </div>
        </div>
    }
}