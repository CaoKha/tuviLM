//! `/charts/{chart_id}/ban-menh`: the Bản mệnh deck. A new chart lands here.

use topcoat::{
    Result,
    context::Cx,
    router::page,
    view::{View, view},
};

use super::chart_id;
use crate::features::{
    ban_menh::ban_menh_screen,
    charts::{accent_style, load},
};

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let (saved, chart) = load(cx, *chart_id(cx)?).await?;
    Ok(view! {
        <main class="mx-auto max-w-5xl pb-16" style=(accent_style(&chart))>
            ban_menh_screen(name: saved.name.as_str(), chart: &chart)
        </main>
    })
}
