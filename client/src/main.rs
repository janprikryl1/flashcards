use yew::{function_component, html, Html};
use yew_router::BrowserRouter;
use crate::auth::AuthProvider;
use crate::components::layout::MainLayout;

mod components;
mod utils;
mod auth;
mod pages;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <AuthProvider>
            <BrowserRouter>
                <MainLayout />
            </BrowserRouter>
        </AuthProvider>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}