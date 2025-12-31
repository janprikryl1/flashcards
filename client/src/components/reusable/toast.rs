use gloo::timers::callback::Timeout;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
}

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub message: String,
    pub toast_type: ToastType,
    pub on_close: Callback<()>,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let on_close = props.on_close.clone();
    use_effect_with((), move |_| {
        let timeout = Timeout::new(3000, move || {
            on_close.emit(());
        });
        timeout.forget();
        || ()
    });

    let bg_color = match props.toast_type {
        ToastType::Success => "bg-green-500",
        ToastType::Error => "bg-red-500",
    };

    let icon = match props.toast_type {
        ToastType::Success => html! {
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
            </svg>
        },
        ToastType::Error => html! {
            <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
        },
    };

    html! {
        <div class={classes!(
            "fixed",
            "bottom-5",
            "right-5",
            "flex",
            "items-center",
            "text-white",
            "px-6",
            "py-4",
            "rounded-lg",
            "shadow-xl",
            "z-[60]",
            bg_color
        )}>
            {icon}
            <span class="font-medium">{ &props.message }</span>
            <button onclick={props.on_close.reform(|_| ())} class="ml-4 hover:text-gray-200">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
            </button>
        </div>
    }
}