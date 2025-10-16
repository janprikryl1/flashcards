use yew::prelude::*;
use yew_router::prelude::*;
use client::pages::index::Index;
use client::pages::login_page::LoginPage;
use client::pages::not_found::NotFound;
use crate::utils::routes::Route;
use crate::components::navbar::Navbar;

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div class="p-4">
            <h1 class="text-xl font-semibold">{ "Secure" }</h1>
            <button {onclick} class="mt-2 underline">{ "Go Home" }</button>
        </div>
    }
}


#[function_component(RouterOutlet)]
fn router_outlet() -> Html {
    let route = use_route::<Route>();

    match route {
        Some(Route::Home)   => html! { <Index /> },
        Some(Route::Login)  => html! { <LoginPage /> },
        Some(Route::Secure) => html! { <Secure /> },
        _ => html! { <NotFound /> },
    }
}


#[function_component(AppRoutes)]
pub fn app_routes() -> Html {
    html! {
        <>
            <Navbar />
            <RouterOutlet />
        </>
    }
}