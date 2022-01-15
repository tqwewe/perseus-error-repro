use perseus::{
    http::header::{HeaderMap, HeaderName},
    Html, RenderFnResultWithCause, SsrNode, Template,
};
use serde::{Deserialize, Serialize};
use sycamore::prelude::{cloned, component, create_memo, view, Signal, View};

use crate::dashboard::Dashboard;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPageProps {
    pub greeting: String,
}

#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page(_props: IndexPageProps) -> View<G> {
    let name = Signal::new("world".to_string());
    let msg = create_memo(cloned!(name => move || {
        let name = name.get();
        if name.trim().is_empty() {
            "Hello, world!!".to_string()
        } else {
            format!("Hello, {}!!", name)
        }
    }));

    let v = view! {
        p {(msg.get())}
        input(bind:value=name, class="form-control", type="text")
        a(href = "about", id = "about-link") { "About!" }
    };

    view! { Dashboard(v) }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_props)
        .template(index_page)
        .head(head)
        .set_headers_fn(set_headers)
}

#[perseus::head]
pub fn head(_props: IndexPageProps) -> View<SsrNode> {
    view! {
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_props(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageProps> {
    Ok(IndexPageProps {
        greeting: "Hello World!".to_string(),
    })
}

#[perseus::autoserde(set_headers)]
pub fn set_headers(props: Option<IndexPageProps>) -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(
        HeaderName::from_lowercase(b"x-greeting").unwrap(),
        props.unwrap().greeting.parse().unwrap(),
    );
    map
}
