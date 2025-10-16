use tailyew::{Button, ButtonType};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::utils::routes::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div class="min-h-screen flex flex-col items-center justify-center px-4">
            <h1 class="text-5xl mb-4 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                { "404" }
            </h1>
            <p class="text-xl text-gray-600 mb-6 text-center">
                { "Tato stránka neexistuje" }
            </p>
            <Button {onclick} button_type={ButtonType::Ghost}>{ "Domů" }</Button>
        </div>
    }
}