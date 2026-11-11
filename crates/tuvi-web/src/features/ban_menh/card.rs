//! The frame of a lá bài phụ (Vận May, Lời Khuyên): glass with a border and
//! a corner glow in the card's own hue, then the icon, the title, the key
//! line, the body and the tag.

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon},
    view::{Child, View, view},
};

/// A body paragraph of a card.
pub(super) const BODY: &str = "mt-2.5 text-xs leading-[1.55] font-light text-muted lg:max-w-[64ch]";

/// `hue` is a CSS color. The card sets it as `--t-hue` for its own parts.
#[topcoat::view::component]
pub(super) async fn tarot(
    cx: &Cx,
    hue: String,
    icon: IconData,
    title: &'static str,
    key: String,
    tag: String,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <article
            class="glass relative flex w-[min(300px,76vw)] shrink-0 snap-center flex-col overflow-hidden border-[color-mix(in_srgb,var(--t-hue)_24%,var(--color-glass-line))] px-5 py-5.5 before:pointer-events-none before:absolute before:inset-0 before:bg-[radial-gradient(120%_70%_at_100%_0%,color-mix(in_srgb,var(--t-hue)_50%,transparent),transparent_55%)] before:opacity-40 before:content-[''] lg:w-auto *:relative"
            style=(format!("--t-hue: {hue}"))
        >
            <div class="flex items-center gap-2.5">
                <span
                    class="grid size-8 place-items-center rounded-[10px] border border-[color-mix(in_srgb,var(--t-hue)_45%,transparent)] bg-[color-mix(in_srgb,var(--t-hue)_18%,transparent)] text-xl text-(--t-hue)"
                >
                    icon(data: icon)
                </span>
                <span class="text-[10px] font-semibold tracking-[0.34em] text-muted">
                    "LÁ BÀI PHỤ"
                </span>
            </div>
            <h3
                class="mt-3.25 font-display text-[13.5px] font-medium tracking-[0.02em] text-muted"
            >
                (title)
            </h3>
            <p
                class="mt-1 font-display text-2xl leading-[1.06] font-semibold [text-shadow:0_0_18px_color-mix(in_srgb,var(--t-hue)_50%,transparent)]"
            >
                (key)
            </p>
            (child)
            <span
                class="mt-4 self-start rounded-full border border-[color-mix(in_srgb,var(--t-hue)_40%,transparent)] bg-[color-mix(in_srgb,var(--t-hue)_10%,transparent)] px-3 py-1.5 text-[11px] font-medium text-(--t-hue)"
            >
                "✦ "
                (tag)
            </span>
        </article>
    })
}
