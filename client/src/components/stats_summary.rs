use yew::prelude::*;
use crate::auth::use_auth;
use crate::components::icon::{render_icon, IconKind};

#[function_component(StatsSummary)]
pub fn stats_summary() -> Html {
    let auth = use_auth();

    if let Some(user) = &auth.me {
        let deck_count: usize = 0;
        let card_count: usize = 0;

        //TODO: Fetch actual counts from the user data when available

        html! {
            <div class="grid md:grid-cols-2 gap-6 mb-16">
                <div class="p-6 bg-white/80 backdrop-blur border border-blue-200 rounded-2xl">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-gray-600 mb-1">{"Celkem balíčků"}</p>
                            <p class="text-4xl text-blue-600">{ deck_count }</p>
                        </div>
                        { render_icon(&IconKind::FolderOpen, classes!("h-12","w-12","text-blue-600","opacity-50")) }
                    </div>
                </div>

                <div class="p-6 bg-white/80 backdrop-blur border border-purple-200 rounded-2xl">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-gray-600 mb-1">{"Celkem kartiček"}</p>
                            <p class="text-4xl text-purple-600">{ card_count }</p>
                        </div>
                        { render_icon(&IconKind::BookOpen, classes!("h-12","w-12","text-purple-600","opacity-50")) }
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
