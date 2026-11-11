//! `POST /logout`: end this browser's session.

use topcoat::{
    Result,
    context::Cx,
    router::{
        error::{SeeOther, see_other},
        href, route,
    },
};

use crate::features::auth::log_out;

#[route(POST)]
pub async fn submit(cx: &Cx) -> Result<SeeOther> {
    log_out(cx).await?;
    Ok(see_other(href!(super::home).resolve(cx)))
}
