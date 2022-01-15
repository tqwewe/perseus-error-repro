use sycamore::prelude::*;

use crate::breadcrumbs::{BreadcrumbItem, Breadcrumbs};
use crate::sidebar::Sidebar;

#[component(Dashboard<G>)]
pub fn dashboard(children: View<G>) -> View<G> {
    view! {
        div(data-color-mode="light", data-light-theme="light") {
            div(class="Layout Layout--gutter-none") {
                Sidebar()
                div(class="Layout-main") {
                    div(class="Header") {
                        div(class="Header-item") {
                            span { "Platform" }
                        }
                        div(class="Header-item") {
                            input(type="search", class="form-control Header-input")
                        }
                    }
                    div(class="dashboard-content color-bg-subtle px-6 py-4 overflow-y-auto") {
                        Breadcrumbs(vec![
                            BreadcrumbItem { href: "/".to_string(), label: "Home".to_string() },
                            BreadcrumbItem { href: "/about".to_string(), label: "About".to_string() },
                        ])

                        (children)
                    }
                }
            }
        }
    }
}
