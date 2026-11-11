use topcoat::{
    Result,
    context::Cx,
    router::{href, module_param, page},
    view::{View, view},
};

use crate::features::charts::{accent_style, chart_grid, load};

module_param!(pub chart_id: u64 else not_found);

pub mod ban_menh;
pub mod hoi_ai;

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let id = *chart_id(cx)?;
    let (saved, chart) = load(cx, id).await?;

    Ok(view! {
        <main class="mx-auto max-w-3xl px-3.5 pb-16" style=(accent_style(&chart))>
            chart_grid(name: saved.name.as_str(), chart: &chart)
            <a
                href=(href!(hoi_ai::show, chart_id: id))
                class="pill-primary mx-auto mt-6 flex h-12 max-w-sm items-center justify-center rounded-full text-sm font-semibold"
            >
                "✦ Hỏi Thiên Hạc về lá số này"
            </a>
        </main>
    })
}
