use tailyew::{Button, ButtonType};
use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::utils::routes::Route;


#[function_component(Index)]
pub fn index() -> Html {
    //let navigator = use_navigator().unwrap();


    html! {
        <div class="mt-5 max-w-5xl mx-auto flex justify-between">
            <h1>{"Vítejte na FashCards"}</h1>


        <div>
        <Button button_type={ButtonType::Primary} onclick={Callback::from(|_| console::log_1(&"Primary Click".into()))}>{ "Primary" }</Button>
        <Button button_type={ButtonType::Secondary}>{ "Secondary" }</Button>
        <Button button_type={ButtonType::Danger}>{ "Danger" }</Button>
        <Button button_type={ButtonType::Button} disabled=true>{ "Disabled" }</Button>
        <Button button_type={ButtonType::Ghost}>{ "Ghost" }</Button>

    </div>

        </div>
    }
}