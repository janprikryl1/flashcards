use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/study")]
    Study,
    #[at("/cards")]
    Cards,
    #[at("/card_collections")]
    CardCollections,
    #[not_found]
    #[at("/404")]
    NotFound,
}