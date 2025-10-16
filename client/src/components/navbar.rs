use yew::prelude::*;
use yew_router::prelude::*;
use tailyew::atoms::Button;
use tailyew::ButtonType;

use crate::utils::routes::Route;
use crate::auth::use_auth;
use crate::auth::actions::logout;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let auth = use_auth();
    let route = use_route::<Route>();

    let on_logout = {
        let auth = auth.clone();
        Callback::from(move |_| logout(auth.clone()))
    };

    let item_classes = |is_active: bool| -> Classes {
        if is_active {
            classes!(
                "px-4", "py-2", "rounded-xl", "font-semibold", "shadow-sm" ,"bg-gray-800", "text-white"
            )
        } else {
            classes!(
                "px-4", "py-2", "rounded-xl", "font-medium", "text-gray-900", "hover:bg-black/5", "transition"
            )
        }
    };

    let is_active = |r: &Route| -> bool {
        match (route.as_ref(), r) {
            (Some(curr), target) => curr == target,
            _ => false,
        }
    };

    html! {
        <nav class="w-full border-b bg-white">
            <div class="max-w-6xl mx-auto flex items-center justify-between px-4 py-3">
                <Link<Route> to={Route::Home} classes="flex items-center gap-3 select-none">
                    <span class="text-lg font-semibold tracking-tight">{ "Flashcards" }</span>
                </Link<Route>>

                <div class="flex items-center gap-2">
                    <Link<Route> to={Route::Home} classes={item_classes(is_active(&Route::Home))}>
                        { "Domů" }
                    </Link<Route>>

                    {
                        if auth.me.is_some() {
                            html! {
                                <>
                                    <Link<Route> to={Route::Cards} classes={item_classes(is_active(&Route::Cards))}>
                                        { "Kartičky" }
                                    </Link<Route>>
                                    <Link<Route> to={Route::CardCollections} classes={item_classes(is_active(&Route::CardCollections))}>
                                        { "Balíčky" }
                                    </Link<Route>>
                                    <Link<Route> to={Route::Study} classes={item_classes(is_active(&Route::Study))}>
                                        { "Studovat" }
                                    </Link<Route>>
                                </>
                            }
                        } else {
                            html! {}
                        }
                    }

                    <div class="ml-1 flex items-center gap-3">
                        <div class="border-l h-6 pr-2"></div>
                        {
                            if let Some(me) = &auth.me {
                                html! {
                                    <>
                                        <span class="text-sm text-gray-600">{ format!("Přihlášen: {}", me.email) }</span>
                                        <Button button_type={ButtonType::Ghost} onclick={on_logout.clone()}>
                                            { "Odhlásit" }
                                        </Button>
                                    </>
                                }
                            } else {
                                html! {
                                    <Link<Route> to={Route::Login} classes="px-4 py-2 rounded-xl font-medium text-gray-900 hover:bg-black/5 transition">
                                        { "Přihlásit" }
                                    </Link<Route>>
                                }
                            }
                        }
                    </div>
                </div>
            </div>
        </nav>
    }
}
