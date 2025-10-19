use yew::prelude::*;

#[function_component(BookOpenIcon)]
pub fn book_open_icon() -> Html {

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none"
                 viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M12 6v12M4 19a2 2 0 012-2h5V5H6a2 2 0 00-2 2v12zm16 0a2 2 0 01-2-2h-5V5h5a2 2 0 012 2v12z" />
        </svg>
    }
}