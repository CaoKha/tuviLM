use topcoat::{
    Result,
    context::Cx,
    router::{module_param, page},
    view::{View, view},
};

use crate::features::charts::{accent_style, chart_grid, load};

module_param!(pub chart_id: u64 else not_found);

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let (saved, chart) = load(cx, *chart_id(cx)?).await?;

    Ok(view! {
        <main class="mx-auto max-w-3xl px-3.5 pb-16" style=(accent_style(&chart))>
            chart_grid(name: saved.name.as_str(), chart: &chart)
        </main>
    })
}
