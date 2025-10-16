use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::utils::routes::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "404" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}