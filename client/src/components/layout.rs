use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::index::Index;
use crate::pages::login_page::LoginPage;
use crate::components::navbar::Navbar;
use crate::pages::dashboard::Dashboard;
use crate::pages::game::Game;
use crate::pages::not_found::NotFound;
use crate::utils::routes::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Index /> },
        Route::Login => html! { <LoginPage /> },
        Route::Game => html! { <Game /> },
        Route::Dashboard => html! { <Dashboard /> },
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