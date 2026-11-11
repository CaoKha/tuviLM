//! Lá bài phụ 2, Lời Khuyên: one entry per Mệnh–Cục relation and âm dương
//! (thuận lý or nghịch lý). The tag names both, so the advice reads as
//! about this chart. The hue is the accent's second shade.

use topcoat::{
    Result,
    context::Cx,
    icon::iconify::iconify_icon,
    view::{View, view},
};

use super::{
    card::{BODY, tarot},
    facts::Facts,
};

#[topcoat::view::component]
pub(super) async fn advice_card(cx: &Cx, facts: &Facts<'_>) -> Result<impl View> {
    let (phrase, advice) = facts.advice();
    let am_duong = if facts.thuan_ly {
        "Thuận lý"
    } else {
        "Nghịch lý"
    };
    Ok(view! {
        tarot(
            hue: "var(--accent-2)".to_owned(),
            icon: iconify_icon!("lucide:book-open"),
            title: "Lời Khuyên",
            key: phrase.to_owned(),
            tag: format!("{} · {am_duong}", facts.menh_cuc.label()),
            <p class=(BODY)>(advice)</p>
        )
    })
}
