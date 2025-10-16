pub mod actions;

use std::rc::Rc;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::RequestCredentials;
use yew::prelude::*;
use crate::utils::constants::api_base;

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct MeResponse {
    pub id: i64,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuthStore {
    pub me: Option<MeResponse>,
    pub loading: bool,
    pub message: Option<String>,
}

impl Default for AuthStore {
    fn default() -> Self {
        Self { me: None, loading: false, message: None }
    }
}

#[derive(Clone)]
pub enum AuthAction {
    SetMe(Option<MeResponse>),
    SetLoading(bool),
    SetMessage(Option<String>),
}

impl Reducible for AuthStore {
    type Action = AuthAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::SetMe(me) => Rc::new(Self { me, ..(*self).clone() }),
            AuthAction::SetLoading(loading) => Rc::new(Self { loading, ..(*self).clone() }),
            AuthAction::SetMessage(message) => Rc::new(Self { message, ..(*self).clone() }),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub struct AuthCtx(pub UseReducerHandle<AuthStore>);


#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let state = use_reducer(AuthStore::default);

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                state.dispatch(AuthAction::SetLoading(true));
                let resp = Request::get(&format!("{}/api/me", api_base()))
                    .credentials(RequestCredentials::Include)
                    .send().await;

                if let Ok(r) = resp {
                    if r.ok() {
                        if let Ok(me) = r.json::<MeResponse>().await {
                            state.dispatch(AuthAction::SetMe(Some(me)));
                        } else {
                            state.dispatch(AuthAction::SetMe(None));
                        }
                    } else {
                        state.dispatch(AuthAction::SetMe(None));
                    }
                } else {
                    state.dispatch(AuthAction::SetMe(None));
                }
                state.dispatch(AuthAction::SetLoading(false));
            });
            || ()
        });
    }

    html! {
        <ContextProvider<AuthCtx> context={AuthCtx(state.clone())}>
            { for props.children.iter() }
        </ContextProvider<AuthCtx>>
    }
}

#[yew::hook]
pub fn use_auth() -> UseReducerHandle<AuthStore> {
    use_context::<AuthCtx>().expect("AuthProvider missing").0.clone()
}