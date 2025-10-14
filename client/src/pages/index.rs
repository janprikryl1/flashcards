use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;


#[function_component(Index)]
pub fn index() -> Html {
    //let navigator = use_navigator().unwrap();


    html! {
        <div>
            <h1>{"Home page"}</h1>
             <Link<Route> to={Route::Home} classes={classes!("btn")}>
                {"Go Home"}
            </Link<Route>>

            <Link<Route> to={Route::Login} classes={classes!("btn", "btn-primary")}>
                {"Login"}
            </Link<Route>>
        </div>
    }
}