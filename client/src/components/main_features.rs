use yew::prelude::*;
use crate::components::icon::{render_icon, IconKind};

#[derive(Properties, PartialEq, Clone)]
struct FeatureCardProps {
    pub title: AttrValue,
    pub description: AttrValue,
    pub route: &'static str,
    pub icon_bg: &'static str,
    pub icon_tone: &'static str,
    pub icon: IconKind,
}

#[function_component(FeatureCard)]
fn feature_card(props: &FeatureCardProps) -> Html {
    html! {
        <div
            class="p-6 bg-white/80 backdrop-blur hover:shadow-lg transition-shadow cursor-pointer rounded-xl"
            role="button"
        >
            <div class={classes!("h-12","w-12", props.icon_bg, "rounded-lg","flex","items-center","justify-center","mb-4")}>
                { render_icon(&props.icon, classes!("h-6", "w-6", props.icon_tone)) }
            </div>

            <h3 class="mb-2 text-gray-800">{ props.title.clone() }</h3>
            <p class="text-sm text-gray-600">{ props.description.clone() }</p>
        </div>
    }
}

#[function_component(MainFeatures)]
pub fn main_features() -> Html {
    html! {
        <div class="mb-16">
            <h2 class="text-3xl text-center mb-12 text-gray-800">{"Hlavní funkce"}</h2>

            <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
                <FeatureCard
                    title="Vytváření kartiček"
                    description="Snadno vytvářejte otázky a odpovědi pro vaše studium"
                    route="cards"
                    icon_bg="bg-blue-100"
                    icon_tone="text-blue-600"
                    icon={IconKind::Plus}
                />

                <FeatureCard
                    title="Organizace do balíčků"
                    description="Tříďte kartičky do tematických balíčků pro lepší přehled"
                    route="decks"
                    icon_bg="bg-purple-100"
                    icon_tone="text-purple-600"
                    icon={IconKind::FolderOpen}
                />

                <FeatureCard
                    title="Režim učení"
                    description="Procvičujte kartičky v náhodném pořadí pro efektivní zapamatování"
                    route="study"
                    icon_bg="bg-green-100"
                    icon_tone="text-green-600"
                    icon={IconKind::Brain}
                />

                <FeatureCard
                    title="Export &amp; Import"
                    description="Zálohujte a sdílejte své kartičky ve formátu JSON nebo CSV"
                    route="import-export"
                    icon_bg="bg-orange-100"
                    icon_tone="text-orange-600"
                    icon={IconKind::Download}
                />
            </div>
        </div>
    }
}
