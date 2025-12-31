use yew::{function_component, html, Html};
use yew_router::BrowserRouter;
use crate::auth::AuthProvider;
use crate::components::layout::MainLayout;
use crate::components::reusable::toast_provider::ToastProvider;

mod components;
mod utils;
mod auth;
mod pages;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <ToastProvider>
            <AuthProvider>
                <BrowserRouter>
                    <MainLayout />
                </BrowserRouter>
            </AuthProvider>
        </ToastProvider>
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}