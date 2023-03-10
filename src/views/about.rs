{% if use_serde and use_reqwest %}
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew_hooks::prelude::*;
{% endif %}
use yew::prelude::*;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    {% if use_serde and use_reqwest %}
    let state =
        use_async(async move { 
            fetch_repo(("yanglinshu/create-yew".to_string()).clone()).await             
        });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };
    {% endif %}

    html! {
        <div class="app">
            <header class="app-header">
                <p>
                    <a
                        class="app-link"
                        href="https://github.com/yanglinshu/create-yew"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        { "Create Yew" }
                    </a>
                    { ", Set up a modern yew web app by running one command." }
                </p>
                {% if use_serde and use_reqwest %}
                <p>
                    <button {onclick}>{ "Load info of this repo" }</button>
                </p>
                <p>
                    {
                        if state.loading {
                            html! { "Loading, wait a sec..." }
                        } else {
                            html! {}
                        }
                    }
                </p>
                {
                    if let Some(repo) = &state.data {
                        html! {
                            <>
                                <p>{ "Repo name: " }<b>{ &repo.name }</b></p>
                                <p>{ "Repo full name: " }<b>{ &repo.full_name }</b></p>
                                <p>{ "Repo description: " }<b>{ &repo.description }</b></p>
                            </>
                            }
                    } else {
                        html! {}
                    }
                }
                <p>
                    {
                        if let Some(error) = &state.error {
                            match error {
                                Error::DeserializeError => html! { "DeserializeError" },
                                Error::RequestError => html! { "RequestError" },
                            }
                        } else {
                            html! {}
                        }
                    }
                </p>
                <p>
                    { "Edit " } <code>{ "src/views/about.rs" }</code> { " and save to reload." }
                </p>
                {% endif %}
            </header>
        </div>
    }
}

{% if use_serde and use_reqwest %}
async fn fetch_repo(repo: String) -> Result<Repo, Error> {
    fetch::<Repo>(format!("https://api.github.com/repos/{}", repo)).await
}

/// You can use reqwest or other crates to fetch your api.
async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        if let Ok(repo) = data.json::<T>().await {
            Ok(repo)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Repo {
    id: i32,
    name: String,
    full_name: String,
    description: String,
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}
{% endif %}

{% if use_wasm_bindgen_test %}
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::About;
    use yew::start_app;

    #[wasm_bindgen_test]
    fn about_page_has_an_app_link() {
        start_app::<About>();

        let app_links = gloo_utils::document().get_elements_by_class_name("app-link");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "Create Yew");
    }
}
{% endif %}