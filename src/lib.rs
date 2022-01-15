mod breadcrumbs;
mod dashboard;
mod sidebar;
mod templates;

use perseus::{define_app, ErrorPages};
use sycamore::view;

define_app! {
    templates: [
        crate::templates::index::get_template::<G>()
    ],
    error_pages: ErrorPages::new(|url, status, err, _| {
        view! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    }),
    static_aliases: {
        "/styles.css" => "static/styles.css",
        "/test.txt" => "static/test.txt"
    }
}
