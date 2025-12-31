use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCredentials;
use crate::utils::functions::api_base;
use crate::auth::{AuthAction, AuthStore, MeResponse};
use yew::prelude::UseReducerHandle;

#[derive(serde::Serialize)]
struct CredentialsBody { email: String, password: String }

pub async fn register(dispatch: UseReducerHandle<AuthStore>, email: String, password: String) -> Result<(), String> {
    dispatch.dispatch(AuthAction::SetLoading(true));
    dispatch.dispatch(AuthAction::SetMessage(None));

    let body = CredentialsBody { email, password };
    let resp = Request::post(&format!("{}/api/register", api_base()))
        .credentials(RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .json(&body).expect("serialize body")
        .send().await;

    let result = match resp {
        Ok(r) if r.status() == 201 => {
            dispatch.dispatch(AuthAction::SetMessage(Some("Registrace proběhla úspěšně".into())));
            Ok(())
        }

        Ok(r) => {
            let err_msg = format!("Registrace selhala: status {}", r.status());
            dispatch.dispatch(AuthAction::SetMessage(Some(err_msg.clone())));
            Err(err_msg)
        }

        Err(e) => {
            let err_msg = format!("Chyba sítě: {e}");
            dispatch.dispatch(AuthAction::SetMessage(Some(err_msg.clone())));
            Err(err_msg)
        }
    };

    dispatch.dispatch(AuthAction::SetLoading(false));
    result
}

pub async fn login(dispatch: UseReducerHandle<AuthStore>, email: String, password: String) -> Result<(), String> {
    dispatch.dispatch(AuthAction::SetLoading(true));
    dispatch.dispatch(AuthAction::SetMessage(None));

    let body = CredentialsBody { email, password };
    let resp = Request::post(&format!("{}/api/login", api_base()))
        .credentials(RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .json(&body)
        .expect("Failed Serialize body")
        .send()
        .await;

    let result = match resp {
        Ok(r) if r.status() == 204 => {
            dispatch.dispatch(AuthAction::SetMessage(Some("Přihlášení proběhlo úspěšně".into())));
            let me = Request::get(&format!("{}/api/me", api_base()))
                .credentials(RequestCredentials::Include)
                .send().await;
            match me {
                Ok(rr) if rr.ok() => match rr.json::<MeResponse>().await {
                    Ok(me) => {
                        dispatch.dispatch(AuthAction::SetMe(Some(me)));
                        Ok(())
                    },
                    Err(_) => {
                        dispatch.dispatch(AuthAction::SetMe(None));
                        Err("Error".into())
                    }
                },
                _ => {
                    dispatch.dispatch(AuthAction::SetMe(None));
                    Err("Error".into())
                }
            }
        }
        Ok(r) => {
            let err_msg = format!("Přihlášení selhalo: {}", r.status());
            dispatch.dispatch(AuthAction::SetMessage(Some(err_msg.clone())));
            Err(err_msg)
        }
        Err(e) => {
            dispatch.dispatch(AuthAction::SetMessage(Some(format!("Chyba: {e}"))));
            Err(format!("Chyba: {e}"))
        }
    };

    dispatch.dispatch(AuthAction::SetLoading(false));
    result
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