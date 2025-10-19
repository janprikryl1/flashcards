use yew::prelude::*;

#[function_component(BrainIcon)]
pub fn brain_icon() -> Html {

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M8 9a3 3 0 016 0m-6 0a3 3 0 00-3 3 3 3 0 003 3m6-6a3 3 0 013 3 3 3 0 01-3 3m-6 0a3 3 0 006 0m-3-9v12" />
        </svg>
    }
}