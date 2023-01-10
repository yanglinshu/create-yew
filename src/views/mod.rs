{% if use_yew_router %}
use yew_router::prelude::*;

pub mod about;
{% endif %}
pub mod home;

{% if use_yew_router %}
use yew::prelude::*;

use about::About;
use home::Home;
{% endif %}

{% if use_yew_router %}
/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
{% endif %}
