use yew::prelude::*;
use tailyew::atoms::Button;
use tailyew::ButtonType;

use crate::auth::{use_auth, MeResponse};
use crate::auth::actions::{login, register};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let auth = use_auth();

    let email = use_state(|| String::new());
    let password = use_state(|| String::new());

    let on_email = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };
    let on_pass = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_register = {
        let auth = auth.clone();
        let email = email.clone();
        let password = password.clone();
        Callback::from(move |_| register(auth.clone(), (*email).clone(), (*password).clone()))
    };

    let on_login = {
        let auth = auth.clone();
        let email = email.clone();
        let password = password.clone();
        Callback::from(move |_| login(auth.clone(), (*email).clone(), (*password).clone()))
    };

    html! {
        <div style="max-width: 480px; margin: 2rem auto;">
            <h1 class="text-lg font-bold">{ "Přihlášení" }</h1>

            <label for="email">{ "Email" }</label>
            <input id="email" type="email"
                style="width: 100%; margin-bottom: .5rem;"
                value={(*email).clone()} oninput={on_email}
                disabled={auth.loading}
            />

            <label for="password">{ "Heslo" }</label>
            <input id="password" type="password"
                style="width: 100%; margin-bottom: 1rem;"
                value={(*password).clone()} oninput={on_pass}
                disabled={auth.loading}
            />

            <div style="display:flex; gap:.5rem;">
                <Button button_type={ButtonType::Ghost} onclick={on_register.clone()} disabled={auth.loading}>{ "Registrovat" }</Button>
                <Button button_type={ButtonType::Ghost} onclick={on_login.clone()} disabled={auth.loading}>{ "Přihlásit" }</Button>
            </div>

            {
                if let Some(msg) = &auth.message {
                    html!{ <p style="margin-top:1rem;">{ msg.clone() }</p> }
                } else { html!{} }
            }

            {
                if let Some(MeResponse { id, email }) = &auth.me {
                    html!{
                        <div style="margin-top:1rem; padding:.75rem; border:1px solid #ddd;">
                            <b>{ "Přihlášený uživatel" }</b>
                            <div>{ format!("id: {id}") }</div>
                            <div>{ format!("email: {email}") }</div>
                        </div>
                    }
                } else { html!{} }
            }
        </div>
    }
}
