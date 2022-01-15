use sycamore::prelude::*;

#[component(Sidebar<G>)]
pub fn sidebar() -> View<G> {
    view! {
        div(class="Layout-sidebar color-bg-overlay overflow-y-auto") {
            a(href="#", class="Header-link f4 d-flex flex-items-center") {
                span { "Thalo" } // remove this line and you get a fatal error of assertion failed: `(left == right)
            }
            span { "logo here" }
            ul {
                span {
                }
            }
        }
    }
}
