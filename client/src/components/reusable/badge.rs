use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    pub name: String,
    pub color: String,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let style = format!(
        "background-color: {}20; color: {};",
        props.color, props.color
    );

    html! {
        <span class="px-4 py-2 rounded-full text-sm" {style}>
            { &props.name }
        </span>
    }
}