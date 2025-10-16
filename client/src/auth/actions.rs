use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCredentials;
use crate::utils::constants::api_base;
use crate::auth::{AuthAction, AuthStore, MeResponse};
use yew::prelude::UseReducerHandle;

#[derive(serde::Serialize)]
struct CredentialsBody { email: String, password: String }

pub fn register(dispatch: UseReducerHandle<AuthStore>, email: String, password: String) {
    spawn_local(async move {
        dispatch.dispatch(AuthAction::SetLoading(true));
        dispatch.dispatch(AuthAction::SetMessage(None));

        let body = CredentialsBody { email, password };
        let resp = Request::post(&format!("{}/api/register", api_base()))
            .credentials(RequestCredentials::Include)
            .header("Content-Type", "application/json")
            .json(&body).expect("serialize body")
            .send().await;

        match resp {
            Ok(r) if r.status() == 201 =>
                dispatch.dispatch(AuthAction::SetMessage(Some("Registrace proběhla úspěšně".into()))),
            Ok(r) =>
                dispatch.dispatch(AuthAction::SetMessage(Some(format!("Registrace selhala: {}", r.status())))),
            Err(e) =>
                dispatch.dispatch(AuthAction::SetMessage(Some(format!("Chyba: {e}")))),
        }

        dispatch.dispatch(AuthAction::SetLoading(false));
    });
}

pub fn login(dispatch: UseReducerHandle<AuthStore>, email: String, password: String) {
    spawn_local(async move {
        dispatch.dispatch(AuthAction::SetLoading(true));
        dispatch.dispatch(AuthAction::SetMessage(None));

        let body = CredentialsBody { email, password };
        let resp = Request::post(&format!("{}/api/login", api_base()))
            .credentials(RequestCredentials::Include)
            .header("Content-Type", "application/json")
            .json(&body).expect("serialize body")
            .send().await;

        match resp {
            Ok(r) if r.status() == 204 => {
                dispatch.dispatch(AuthAction::SetMessage(Some("Přihlášení proběhlo úspěšně".into())));
                let me = Request::get(&format!("{}/api/me", api_base()))
                    .credentials(RequestCredentials::Include)
                    .send().await;

                match me {
                    Ok(rr) if rr.ok() => match rr.json::<MeResponse>().await {
                        Ok(me) => dispatch.dispatch(AuthAction::SetMe(Some(me))),
                        Err(_) => dispatch.dispatch(AuthAction::SetMe(None)),
                    },
                    _ => dispatch.dispatch(AuthAction::SetMe(None)),
                }
            }
            Ok(r) =>
                dispatch.dispatch(AuthAction::SetMessage(Some(format!("Přihlášení selhalo: {}", r.status())))),
            Err(e) =>
                dispatch.dispatch(AuthAction::SetMessage(Some(format!("Chyba: {e}")))),
        }

        dispatch.dispatch(AuthAction::SetLoading(false));
    });
}

pub fn logout(dispatch: UseReducerHandle<AuthStore>) {
    spawn_local(async move {
        dispatch.dispatch(AuthAction::SetLoading(true));
        dispatch.dispatch(AuthAction::SetMessage(None));

        let resp = Request::post(&format!("{}/api/logout", api_base()))
            .credentials(RequestCredentials::Include)
            .send().await;

        match resp {
            Ok(r) if r.status() == 204 => {
                dispatch.dispatch(AuthAction::SetMe(None));
                dispatch.dispatch(AuthAction::SetMessage(Some("Odhlášen".into())));
            }
            Ok(r) =>
                dispatch.dispatch(AuthAction::SetMessage(Some(format!("Chyba odhlášení: {}", r.status())))),
            Err(e) =>
                dispatch.dispatch(AuthAction::SetMessage(Some(format!("Chyba: {e}")))),
        }

        dispatch.dispatch(AuthAction::SetLoading(false));
    });
}