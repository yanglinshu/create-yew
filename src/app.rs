use yew::prelude::*;

{% if use_yew_router %}
use yew_router::prelude::*;
{% endif %}

{% if use_yew_router %}
use crate::components::nav::Nav;
use crate::views::{switch, AppRoute};
{% else %}
use crate::views::home::Home;
{% endif %}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        {% if use_yew_router %}
        <BrowserRouter>
            <Nav />
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
        {% else %}
        <Home />
        {% endif %}
    }
}
