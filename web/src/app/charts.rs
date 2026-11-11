use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, page},
    view::{View, view},
};

use crate::{app::new_chart_form, features::charts as chart_store};

pub mod chart_id;

/// Saves a new chart, then redirects to it. On bad input, shows the form again
/// with the error.
#[page(POST)]
pub async fn create(cx: &Cx, Form(form): Form<chart_store::NewChart>) -> Result<impl View> {
    let error = match chart_store::create(cx, &form).await? {
        Ok(id) => return Err(see_other(href!(chart_id::show, chart_id: id).resolve(cx)).into()),
        Err(error) => error.to_string(),
    };
    Ok(view! {
        <main class="mx-auto max-w-xl px-4 pb-16">
            new_chart_form(error: Some(error))
        </main>
    })
}
