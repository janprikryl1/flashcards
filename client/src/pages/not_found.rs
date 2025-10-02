use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yew_router::Routable;
//use crate::Route;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Secure));
    html! {
        <div>
            <h1>{ "404" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}