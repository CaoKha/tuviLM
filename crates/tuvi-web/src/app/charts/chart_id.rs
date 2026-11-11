use topcoat::{
    Result,
    context::Cx,
    router::{module_param, page},
    view::{View, view},
};

use crate::features::charts::{chart_grid, load};

module_param!(pub chart_id: u64 else not_found);

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let (saved, chart) = load(cx, *chart_id(cx)?).await?;

    Ok(view! {
        <main class="mx-auto max-w-5xl px-4 pb-16">
            chart_grid(name: saved.name.as_str(), chart: &chart)
        </main>
    })
}
