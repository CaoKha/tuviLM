//! The Bản mệnh screen: the header and the deck. On a phone the deck is a
//! row of cards that scrolls sideways and snaps to each card. From `lg` the
//! main card is on the left and the two other cards are on the right.

use topcoat::{
    Result,
    context::Cx,
    router::error::not_found,
    view::{View, view},
};
use tuvi_core::Chart;

use super::{advice::advice_card, destiny::destiny_card, facts::Facts, luck::luck_card};

#[topcoat::view::component]
pub async fn ban_menh_screen(cx: &Cx, name: &str, chart: &Chart) -> Result<impl View> {
    let Some(facts) = Facts::of(chart) else {
        return Err(not_found().into());
    };
    Ok(view! {
        <header class="px-4">
            <div class="flex items-center justify-between gap-3.5">
                <p
                    class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
                >
                    "Chào bạn"
                </p>
                <p
                    class="flex items-center gap-2 rounded-full border border-accent-glow bg-accent/12 px-3.5 py-1.5 text-[13px] font-semibold"
                >
                    <span
                        class="size-2.25 rounded-full bg-linear-135 from-accent to-accent-2 shadow-[0_0_10px_var(--accent-glow)]"
                    ></span>
                    if facts.chinh_tinh.is_empty() {
                        <span class="text-accent">"Vô Chính Diệu"</span>
                    }
                    for (index, sao) in facts.chinh_tinh.iter().enumerate() {
                        if index > 0 {
                            <span class="text-muted">"·"</span>
                        }
                        <span
                            class=(if index == 0 {
                                "text-accent"
                            } else {
                                "text-accent-2"
                            })
                        >
                            (*sao)
                        </span>
                    }
                </p>
            </div>
            <h1 class="mt-2.5 text-[26px] leading-[1.06] font-semibold">(name)</h1>
            <p class="mt-1.75 text-[13.5px] text-muted lg:hidden">
                "Lá Bài Bản Mệnh ở đầu, vuốt sang để lật hai lá bài phụ."
            </p>
        </header>
        <div
            class="flex snap-x snap-mandatory items-center gap-3.5 overflow-x-auto px-[max(16px,calc(50%-165px))] py-10 lg:grid lg:grid-cols-[364px_minmax(0,1fr)] lg:items-stretch lg:gap-5 lg:px-4"
        >
            destiny_card(facts: &facts)
            luck_card(facts: &facts)
            advice_card(facts: &facts)
        </div>
    })
}
