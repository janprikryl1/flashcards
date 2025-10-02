use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::Routable;
use crate::routes::Route;

#[function_component(Index)]
pub fn index() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{"Home"}</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}