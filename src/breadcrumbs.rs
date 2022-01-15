use sycamore::prelude::*;

pub struct BreadcrumbItem {
    pub href: String,
    pub label: String,
}

#[component(Breadcrumbs<G>)]
pub fn breadcrumbs(items: Vec<BreadcrumbItem>) -> View<G> {
    let items_len = items.len();
    let item_views = View::<G>::new_fragment(
        items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                if index == items_len - 1 {
                    view! {
                        li(class="breadcrumb-item breadcrumb-item-selected") {
                            a(href=(item.href)) { (item.label) }
                        }
                    }
                } else {
                    view! {
                        li(class="breadcrumb-item") {
                            a(href=(item.href)) { (item.label) }
                        }
                    }
                }
            })
            .collect(),
    );

    view! {
        nav(aria-label="Breadcrumb") {
            ol {
                (item_views)
            }
        }
    }
}
