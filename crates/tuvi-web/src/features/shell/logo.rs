//! The logo and the product name. It links to the An sao screen.

use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    router::href,
    view::{View, view},
};

/// The crane logo, also the 48px browser tab icon.
pub const LOGO: Asset = asset!("assets/icons/favicon-48x48.png");

#[topcoat::view::component]
pub async fn brand(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <a href=(href!(crate::app::home)) class="flex items-center gap-2.5">
            <img
                src=(LOGO)
                alt=""
                width="48"
                height="48"
                class="size-9 drop-shadow-[0_0_10px_var(--accent-glow)]"
            >
            <span>
                <b class="block font-display text-xl leading-tight font-semibold">
                    "Thiên Hạc"
                </b>
                <small
                    class="block text-[9px] font-bold tracking-[0.2em] text-muted uppercase"
                >
                    "An sao & luận giải"
                </small>
            </span>
        </a>
    })
}
