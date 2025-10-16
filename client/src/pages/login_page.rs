use yew::prelude::*;
use tailyew::atoms::Button;
use tailyew::{ButtonType, Input, InputType};
use crate::auth::{use_auth, MeResponse};
use crate::auth::actions::{login, register};
use yew_router::prelude::*;
use crate::utils::routes::Route;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let nav = use_navigator().expect("LoginPage must be under <BrowserRouter>");
    let auth = use_auth();

    let email = use_state(|| String::new());
    let password = use_state(|| String::new());

    let is_logged_in = auth.me.is_some();
    use_effect_with(is_logged_in, move |logged_in| {
        if *logged_in {
            nav.replace(&Route::Dashboard);
        }
        || ()
    });

    let on_email: Callback<String> = {
        let email = email.clone();
        Callback::from(move |val: String| email.set(val))
    };
    let on_pass: Callback<String> = {
        let password = password.clone();
        Callback::from(move |val: String| password.set(val))
    };

    let on_register = {
        let auth = auth.clone();
        let email = email.clone();
        let password = password.clone();
        Callback::from(move |_| {
            register(auth.clone(), (*email).clone(), (*password).clone())
        })
    };

    let on_login = {
        let auth = auth.clone();
        let email = email.clone();
        let password = password.clone();
        Callback::from(move |_| {
            login(auth.clone(), (*email).clone(), (*password).clone())
        })
    };

    html! {
        <div style="max-width: 480px; margin: 2rem auto;">
            <h1 class="text-lg font-bold">{ "Přihlášení" }</h1>

            <label for="email">{ "Email" }</label>
            <Input
                id="email"
                input_type={InputType::Email}
                default_value={(*email).clone()}
                on_change={on_email}
                disabled={auth.loading}
            />

            <label for="password">{ "Heslo" }</label>
            <Input
                id="password"
                input_type={InputType::Password}
                default_value={(*password).clone()}
                on_change={on_pass}
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
