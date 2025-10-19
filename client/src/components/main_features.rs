use yew::prelude::*;
use crate::components::feature_card::FeatureCard;
use crate::components::icons::brain_icon::BrainIcon;
use crate::components::icons::download_icon::DownloadIcon;
use crate::components::icons::folder_open_icon::FolderOpenIcon;
use crate::components::icons::plus_icon::PlusIcon;

#[function_component(MainFeatures)]
pub fn main_features() -> Html {
    html! {
        <div class="mb-16">
            <h2 class="text-3xl text-center mb-12 text-gray-800">{ "Hlavní funkce" }</h2>

            <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
                <FeatureCard
                    title="Vytváření kartiček"
                    description="Snadno vytvářejte otázky a odpovědi pro vaše studium"
                    route="cards"
                    icon_bg="bg-blue-100"
                    icon_tone="text-blue-600"
                    icon={html!(<PlusIcon />)}
                />

                <FeatureCard
                    title="Organizace do balíčků"
                    description="Tříďte kartičky do tematických balíčků pro lepší přehled"
                    route="decks"
                    icon_bg="bg-purple-100"
                    icon_tone="text-purple-600"
                    icon={html!(<FolderOpenIcon />)}
                />

                <FeatureCard
                    title="Režim učení"
                    description="Procvičujte kartičky v náhodném pořadí pro efektivní zapamatování"
                    route="study"
                    icon_bg="bg-green-100"
                    icon_tone="text-green-600"
                    icon={html!(<BrainIcon />)}
                />

                <FeatureCard
                    title="Export &amp; Import"
                    description="Zálohujte a sdílejte své kartičky ve formátu JSON nebo CSV"
                    route="import-export"
                    icon_bg="bg-orange-100"
                    icon_tone="text-orange-600"
                    icon={html!(<DownloadIcon />)}
                />
            </div>
        </div>
    }
}
