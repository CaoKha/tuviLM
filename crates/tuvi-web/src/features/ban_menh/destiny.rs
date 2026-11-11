//! Lá Bài Bản Mệnh, the main card: the chính tinh of cung Mệnh, their
//! archetype and mantra, and their constellations from the An sao preview.
//! Vô chính diệu gets its own entry and the scattered sky.

use topcoat::{
    Result,
    context::Cx,
    view::{View, view},
};

use super::facts::Facts;
use crate::features::an_sao::{chom_figure, chom_sao_of, sky};

#[topcoat::view::component]
pub(super) async fn destiny_card(cx: &Cx, facts: &Facts<'_>) -> Result<impl View> {
    let (archetype, mantra) = facts.destiny;
    let figures: Vec<_> = facts
        .chinh_tinh
        .iter()
        .filter_map(|sao| chom_sao_of(sao))
        .collect();
    let song_tinh = facts.chinh_tinh.len() == 2;
    Ok(view! {
        <article
            class="relative flex aspect-[9/16] w-[min(330px,84vw)] shrink-0 snap-center flex-col overflow-hidden rounded-[22px] border border-accent/40 bg-linear-168 from-bg-2 via-bg-1 to-bg-0 px-6 pt-6.5 pb-6 shadow-[0_0_40px_color-mix(in_srgb,var(--accent)_26%,transparent)] lg:row-span-2 lg:w-full"
        >
            <h2 class="font-display text-[19px] font-semibold tracking-[0.06em]">
                "TỬ VI CÁ NHÂN"
            </h2>
            <p class="mt-1.25 text-[13.5px] text-muted">
                if facts.chinh_tinh.is_empty() {
                    "Mệnh "
                    <b class="font-semibold text-ink">"vô chính diệu"</b>
                    " · cả bầu trời soi chiếu"
                } else {
                    "Chính tinh chiếu mệnh: "
                    <b class="font-semibold text-ink">
                        (facts.chinh_tinh.join(" · "))
                    </b>
                    if song_tinh {
                        " · sao đôi"
                    }
                }
            </p>
            <p
                class="mt-3.5 self-start rounded-full border border-accent/52 bg-glass px-4 py-2 text-[13.5px] font-semibold"
            >
                <span class="text-accent">"✦ "</span>
                (archetype)
            </p>
            <div class="lit flex min-h-0 flex-1 items-center justify-center gap-0.5">
                if figures.is_empty() {
                    sky(class: "h-[45%] w-full")
                }
                for (index, chom) in figures.into_iter().enumerate() {
                    chom_figure(chom: chom, song_tinh: song_tinh, delay_ms: index * 110)
                }
            </div>
            <p
                class="border-t border-accent/22 pt-3.5 font-display text-lg leading-[1.42] italic"
            >
                (mantra)
            </p>
        </article>
    })
}
