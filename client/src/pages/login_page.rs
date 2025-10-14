use gloo_net::http::Request;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::RequestCredentials;
use tailyew::atoms::Button;
use tailyew::ButtonType;
use crate::utils::constants::api_base;

#[derive(Serialize)]
struct CredentialsBody {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug, Clone)]
struct MeResponse {
    id: i64,
    email: String,
}


#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let email = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let me = use_state(|| Option::<MeResponse>::None);
    let msg = use_state(|| "".to_string());

    let on_register = {
        let email = email.clone();
        let password = password.clone();
        let msg = msg.clone();
        Callback::from(move |_| {
            let email = (*email).clone();
            let password = (*password).clone();
            let msg = msg.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let body = CredentialsBody { email, password };
                let resp = Request::post(&format!("{}/api/register", api_base()))
                    .credentials(RequestCredentials::Include)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .expect("serialize body")
                    .send().await;

                match resp {
                    Ok(r) if r.status() == 201 => msg.set("Registrace OK".into()),
                    Ok(r) => msg.set(format!("Registrace selhala: {}", r.status())),
                    Err(e) => msg.set(format!("Chyba: {e}")),
                }
            });
        })
    };

    let on_login = {
        let email = email.clone();
        let password = password.clone();
        let msg = msg.clone();
        Callback::from(move |_| {
            let email = (*email).clone();
            let password = (*password).clone();
            let msg = msg.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let body = CredentialsBody { email, password };
                let resp = Request::post(&format!("{}/api/login", api_base()))
                    .credentials(RequestCredentials::Include)
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .expect("serialize body")
                    .send().await;

                match resp {
                    Ok(r) if r.status() == 204 => msg.set("Přihlášení OK".into()),
                    Ok(r) => msg.set(format!("Přihlášení selhalo: {}", r.status())),
                    Err(e) => msg.set(format!("Chyba: {e}")),
                }
            });
        })
    };

    let on_me = {
        let me = me.clone();
        let msg = msg.clone();
        Callback::from(move |_| {
            let me = me.clone();
            let msg = msg.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get(&format!("{}/api/me", api_base()))
                    .credentials(web_sys::RequestCredentials::Include)
                    .send().await;

                match resp {
                    Ok(r) if r.ok() => {
                        let data: MeResponse = r.json().await.unwrap();
                        me.set(Some(data));
                        msg.set("Načteno /api/me".into());
                    }
                    Ok(r) => msg.set(format!("Neautorizováno: {}", r.status())),
                    Err(e) => msg.set(format!("Chyba: {e}")),
                }
            });
        })
    };

    let on_logout = {
        let me = me.clone();
        let msg = msg.clone();
        Callback::from(move |_| {
            let me = me.clone();
            let msg = msg.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post(&format!("{}/api/logout", api_base()))
                    .credentials(RequestCredentials::Include)
                    .send().await;

                match resp {
                    Ok(r) if r.status() == 204 => {
                        me.set(None);
                        msg.set("Odhlášen".into());
                    }
                    Ok(r) => msg.set(format!("Chyba odhlášení: {}", r.status())),
                    Err(e) => msg.set(format!("Chyba: {e}")),
                }
            });
        })
    };

    html! {
        <div style="max-width: 480px; margin: 2rem auto;">
            <h1 class="text-lg font-bold">{ "Přihlášení" }</h1>

            <label>{ "Email" }</label>
            <input
                style="width: 100%; margin-bottom: 0.5rem;"
                value={(*email).clone()}
                oninput={{
                    let email = email.clone();
                    Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        email.set(input.value());
                    })
                }}
            />

            <label>{ "Heslo" }</label>
            <input
                type="password"
                style="width: 100%; margin-bottom: 1rem;"
                value={(*password).clone()}
                oninput={{
                    let password = password.clone();
                    Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        password.set(input.value());
                    })
                }}
            />


                <Button button_type={ButtonType::Ghost} onclick={on_register.clone()}>{ "Registrovat" }</Button>
                <Button button_type={ButtonType::Ghost} onclick={on_login.clone()}>{ "Přihlásit" }</Button>
                <Button button_type={ButtonType::Ghost} onclick={on_me.clone()}>{ "Kdo jsem (/api/me)" }</Button>
                <Button button_type={ButtonType::Ghost} onclick={on_logout.clone()}>{ "Odhlásit" }</Button>


            <p style="margin-top:1rem;">{ (*msg).clone() }</p>

            {
                if let Some(me) = &*me {
                    html!{
                        <div style="margin-top:1rem; padding: .75rem; border: 1px solid #ddd;">
                            <b>{ "Přihlášený uživatel" }</b>
                            <div>{ format!("id: {}", me.id) }</div>
                            <div>{ format!("email: {}", me.email) }</div>
                        </div>
                    }
                } else {
                    html!{}
                }
            }
        </div>
    }
}