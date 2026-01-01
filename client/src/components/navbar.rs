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
    let is_menu_open = use_state(|| false);

    let on_logout = {
        let auth = auth.clone();
        Callback::from(move |_| logout(auth.clone()))
    };

    let item_classes_desktop = |is_active: bool| -> Classes {
        let base = classes!("px-4", "py-2", "rounded-xl");
        if is_active {
            classes!(base , "font-semibold", "shadow-sm" ,"bg-gray-800", "text-white")
        } else {
            classes!(base, "font-medium", "text-gray-900", "hover:bg-black/5", "transition")
        }
    };

    let mobile_item_classes = |is_active: bool| -> Classes {
        let base = classes!("block", "px-4", "py-3", "rounded-lg", "text-base", "font-medium", "transition");
        if is_active {
            classes!(base, "bg-gray-100", "text-gray-900", "font-semibold")
        } else {
            classes!(base, "text-gray-600", "hover:bg-gray-50", "hover:text-gray-900")
        }
    };

    let is_active = |r: &Route| -> bool {
        match (route.as_ref(), r) {
            (Some(curr), target) => curr == target,
            _ => false,
        }
    };

    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };

    let close_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(false);
        })
    };

    html! {
        <nav class="w-full border-b bg-white relative">
            <div class="max-w-6xl mx-auto flex items-center justify-between px-4 py-3">
                <div class="flex items-center gap-3">
                    <button
                        onclick={toggle_menu}
                        class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-200"
                    >
                        {
                            if *is_menu_open {
                                html! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                }
                            } else {
                                html! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                    </svg>
                                }
                            }
                        }
                    </button>
                    <Link<Route> to={Route::Home} classes={item_classes_desktop(is_active(&Route::Home))}>
                        { "Domů" }
                    </Link<Route>>
                </div>

                <div class="hidden md:flex items-center gap-2">
                    {
                        if auth.me.is_some() {
                            html! {
                                <>
                                    <Link<Route> to={Route::Cards} classes={item_classes_desktop(is_active(&Route::Cards))}>
                                        { "Kartičky" }
                                    </Link<Route>>
                                    <Link<Route> to={Route::CardCollections} classes={item_classes_desktop(is_active(&Route::CardCollections))}>
                                        { "Balíčky" }
                                    </Link<Route>>
                                    <Link<Route> to={Route::Study} classes={item_classes_desktop(is_active(&Route::Study))}>
                                        { "Studovat" }
                                    </Link<Route>>
                                </>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>

                <div class="flex items-center gap-3 ml-auto">
                    <div class="flex items-center gap-3 min-w-0">
                        <div class="hidden md:block border-l h-6 pr-2 shrink-0"></div>
                            { if let Some(me) = &auth.me {
                                html! {
                                    <>
                                        <div class="hidden md:block flex items-center min-w-0 text-sm text-gray-600" title={me.email.clone()}>
                                            <span class="hidden md:inline whitespace-nowrap text-sm text-gray-600 mr-1">
                                                { format!("Přihlášen: {}", me.email) }
                                            </span>
                                        </div>
                                        <div class="shrink-0">
                                            <Button button_type={ButtonType::Ghost} onclick={on_logout.clone()}>
                                                { "Odhlásit" }
                                            </Button>
                                        </div>
                                    </>
                                }
                            } else {
                                html! {
                                    <Link<Route> to={Route::Login} classes="px-4 py-2 rounded-xl font-medium text-gray-900 hover:bg-black/5 transition">
                                        { "Přihlásit" }
                                    </Link<Route>>
                                }
                            }}
                    </div>
                </div>
            </div>
            if *is_menu_open {
                <div class="md:hidden absolute top-full left-0 w-full bg-white border-b shadow-lg z-50 animate-in slide-in-from-top-2 duration-200">
                    <div class="flex flex-col p-4 space-y-2">
                        <div onclick={close_menu.clone()}>
                            <Link<Route> to={Route::Home} classes={mobile_item_classes(is_active(&Route::Home))}>
                                { "Domů" }
                            </Link<Route>>
                        </div>

                        if auth.me.is_some() {
                           <div onclick={close_menu.clone()} class="space-y-2">
                                <Link<Route> to={Route::Cards} classes={mobile_item_classes(is_active(&Route::Cards))}>
                                    { "Kartičky" }
                                </Link<Route>>
                                <Link<Route> to={Route::CardCollections} classes={mobile_item_classes(is_active(&Route::CardCollections))}>
                                    { "Balíčky" }
                                </Link<Route>>
                                <Link<Route> to={Route::Study} classes={mobile_item_classes(is_active(&Route::Study))}>
                                    { "Studovat" }
                                </Link<Route>>
                           </div>
                            if let Some(me) = &auth.me {
                               <div class="pt-2 mt-2 border-t border-gray-100">
                                   <div class="px-4 py-2 text-sm text-gray-500">
                                        { "Přihlášen jako: " } <span class="font-medium text-gray-900">{ &me.email }</span>
                                   </div>
                               </div>
                           }
                        }
                    </div>
                </div>
            }
        </nav>
    }
}
