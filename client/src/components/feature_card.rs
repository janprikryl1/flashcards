use yew::{classes, function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct FeatureCardProps {
    pub title: AttrValue,
    pub description: AttrValue,
    pub route: &'static str,
    pub icon_bg: &'static str,
    pub icon_tone: &'static str,
    pub icon: Html,
}

#[function_component(FeatureCard)]
pub fn feature_card(props: &FeatureCardProps) -> Html {
    html! {
        <div
            class="p-6 bg-white/80 backdrop-blur hover:shadow-lg transition-shadow cursor-pointer rounded-xl"
            role="button"
        >
            <div class={classes!("h-12","w-12", props.icon_bg, "rounded-lg","flex","items-center","justify-center","mb-4")}>
                <span class={classes!("h-6","w-6", props.icon_tone)}>
                    { props.icon.clone() }
                </span>
            </div>

            <h3 class="mb-2 text-gray-800">{ props.title.clone() }</h3>
            <p class="text-sm text-gray-600">{ props.description.clone() }</p>
        </div>
    }
}