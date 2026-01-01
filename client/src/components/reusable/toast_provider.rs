use std::rc::Rc;
use yew::prelude::*;
use crate::components::reusable::toast::{Toast, ToastType};
//AI

#[derive(Clone, PartialEq)]
pub struct ToastStore {
    pub message: Option<String>,
    pub toast_type: ToastType,
}

impl Default for ToastStore {
    fn default() -> Self {
        Self {
            toast_type: ToastType::Success,
            message: None,
        }
    }
}

pub enum ToastAction {
    Show {message: String, toast_type: ToastType},
    Hide
}

impl Reducible for ToastStore {
    type Action = ToastAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ToastAction::Show {message, toast_type} => Rc::new(Self {
                message: Some(message),
                toast_type,
            }),
            ToastAction::Hide => Rc::new(Self {
                message: None,
                toast_type: self.toast_type.clone(),
            }),
        }
    }
}

pub type ToastCtx = UseReducerHandle<ToastStore>;

#[derive(Properties, PartialEq)]
pub struct ToastProviderProps {
    #[prop_or_default]
    pub children: Children,
}


#[function_component(ToastProvider)]
pub fn toast_provider(props: &ToastProviderProps) -> Html {
    let state = use_reducer(ToastStore::default);

    let on_close = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(ToastAction::Hide))
    };

    html! {
        <ContextProvider<ToastCtx> context={state.clone()}>
            { for props.children.iter() }

            {if let Some(msg) = &state.message {
                html! {
                    <Toast
                        message={msg.clone()}
                        toast_type={state.toast_type.clone()}
                        on_close={on_close}
                    />
                }
            } else {
                html! {}
            }}
        </ContextProvider<ToastCtx>>
    }
}

#[derive(Clone)]
pub struct ToastManager {
    dispatcher: UseReducerDispatcher<ToastStore>
}

impl ToastManager {
    pub fn success(&self, message: String) {
        self.dispatcher.dispatch(ToastAction::Show {
            message,
            toast_type: ToastType::Success,
        });
    }
    pub fn error(&self, message: String) {
        self.dispatcher.dispatch(ToastAction::Show {
            message,
            toast_type: ToastType::Error,
        });
    }
}

#[yew::hook]
pub fn use_toast() -> ToastManager {
    let ctx = use_context::<ToastCtx>().expect("ToastProvider missing");
    ToastManager {
        dispatcher: ctx.dispatcher(),
    }
}