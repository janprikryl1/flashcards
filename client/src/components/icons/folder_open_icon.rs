use yew::prelude::*;

#[function_component(FolderOpenIcon)]
pub fn folder_open_icon() -> Html {

    html! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M3 7a2 2 0 012-2h4l2 2h8a2 2 0 012 2v2H6a2 2 0 00-1.94 1.5l-1.6 6A2 2 0 004.4 21h13.2a2 2 0 001.94-1.5L22 11" />
        </svg>
    }
}