use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::index::Index;
use crate::pages::login_page::LoginPage;
use crate::components::navbar::Navbar;
use crate::pages::card_collections::CardCollections;
use crate::pages::cards::Cards;
use crate::pages::study::Study;
use crate::pages::not_found::NotFound;
use crate::utils::routes::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Index /> },
        Route::Login => html! { <LoginPage /> },
        Route::Study => html! { <Study /> },
        Route::Cards => html! { <Cards /> },
        Route::CardCollections => html! { <CardCollections /> },
        _ => html! { <NotFound /> },
    }
}

#[function_component(MainLayout)]
pub fn main_layout() -> Html {
    html! {
        <>
            <Navbar />
            <main>
                <Switch<Route> render={Callback::from(switch)} />
            </main>
        </>
    }
}