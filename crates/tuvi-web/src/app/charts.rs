//! `POST /charts`: saves the An sao form (`features::an_sao`).

use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, page},
    view::{View, view},
};

use crate::features::{
    an_sao::an_sao_screen,
    auth::require_user,
    charts::{NewChart, create as save_chart},
};

pub mod chart_id;

/// Saves a new chart, then opens its Bản mệnh deck. On bad input, shows the An sao
/// screen again with the sent values and the error.
#[page(POST)]
pub async fn create(cx: &Cx, Form(form): Form<NewChart>) -> Result<impl View> {
    let user = require_user(cx).await?;
    let error = match form.validate() {
        Ok(chart) => {
            let id = save_chart(cx, user.id, chart).await?;
            return Err(
                see_other(href!(chart_id::ban_menh::show, chart_id: id).resolve(cx)).into(),
            );
        }
        Err(error) => error.to_string(),
    };
    Ok(view! {
        <main class="mx-auto max-w-5xl px-4 pb-16">
            an_sao_screen(
                action: href!(create).resolve(cx),
                sent: Some(&form),
                error: Some(error)
            )
        </main>
    })
}
