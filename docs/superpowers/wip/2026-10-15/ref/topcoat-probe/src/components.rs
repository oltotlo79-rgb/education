//! Reusable components (not part of the route tree: no #[page] here).

use topcoat::{
    Result,
    view::{Child, View, class, component, view},
};

/// A card with a title prop and child content.
#[component]
pub async fn card(
    title: &str,
    #[default] highlighted: bool,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section class=(class!("card", "card--highlight" if highlighted))>
            <h2>(title)</h2>
            <div class="card-body">(child)</div>
        </section>
    })
}

/// Shows a list of form errors, or nothing.
#[component]
pub async fn error_list(errors: Vec<String>) -> Result<impl View> {
    Ok(view! {
        if !errors.is_empty() {
            <ul class="errors">
                for e in errors {
                    <li>(e)</li>
                }
            </ul>
        }
    })
}
