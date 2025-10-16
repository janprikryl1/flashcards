use yew::prelude::*;
use crate::components::hero::Hero;
use crate::components::main_features::MainFeatures;
use crate::components::stats_summary::StatsSummary;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <div class="mt-5 container mx-auto px-4">
           <Hero />
           <StatsSummary />
           <MainFeatures />
        </div>
    }
}