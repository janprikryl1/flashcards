use gloo_net::http::Request;
use web_sys::RequestCredentials;
use yew::prelude::*;
use yew_router::prelude::*;
use tailyew::atoms::Button;
use tailyew::ButtonType;
use crate::utils::routes::Route;
use crate::auth::{use_auth, AuthAction};
use crate::utils::constants::api_base;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let auth = use_auth();
    let navigator = use_navigator().unwrap();

    let on_logout = {
        let auth = auth.clone();
        Callback::from(move |_| {
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post(&format!("{}/api/logout", api_base()))
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await;

                match resp {
                    Ok(r) if r.status() == 204 => {
                        auth.dispatch(AuthAction::SetMe(None));
                        auth.dispatch(AuthAction::SetMessage(Some("Odhlášen".into())));
                    }
                    Ok(r) => {
                        auth.dispatch(AuthAction::SetMessage(Some(format!("Chyba odhlášení: {}", r.status()))));
                    }
                    Err(e) => {
                        auth.dispatch(AuthAction::SetMessage(Some(format!("Chyba: {e}"))));
                    }
                }
            });
        })
    };

    html! {
        <nav class="w-full border-b bg-white">
            <div class="max-w-5xl mx-auto flex items-center justify-between p-3">
                <div class="flex items-center gap-4">
                    <Link<Route> to={Route::Home} classes="font-semibold">{ "MojeApp" }</Link<Route>>
                    <Link<Route> to={Route::Secure} classes="opacity-80 hover:opacity-100">{ "Secure" }</Link<Route>>
                </div>

                <div class="flex items-center gap-3">
                    {
                        if let Some(me) = &auth.me {
                            html! {
                                <>
                                    <span class="text-sm opacity-80">{ format!("Přihlášen: {}", me.email) }</span>
                                    <Button button_type={ButtonType::Ghost} onclick={on_logout.clone()}>{ "Odhlásit" }</Button>
                                </>
                            }
                        } else {
                            html! {
                                <Link<Route> to={Route::Login} classes="opacity-80 hover:opacity-100">{ "Přihlásit" }</Link<Route>>
                            }
                        }
                    }
                </div>
            </div>
        </nav>
    }
}
