//! The app navigation. Below `lg` it is a fixed bottom tab bar, as wide as
//! the phone column. From `lg` it is a full-height left rail with the brand.
//! One set of markup: only the classes change.

use topcoat::{
    Result,
    context::Cx,
    icon::icon,
    view::{View, class, view},
};

use super::{
    logo::brand,
    tabs::{Tab, current_chart, tabs},
};

/// The classes every tab shares: a column in the bar, a row in the rail.
const TAB: &str = "relative flex min-w-0 flex-1 flex-col items-center gap-1 px-px py-1.5 text-[10px] font-medium whitespace-nowrap transition-colors lg:flex-none lg:flex-row lg:gap-3 lg:rounded-xl lg:px-3.25 lg:py-2.75 lg:text-sm";

#[topcoat::view::component]
pub async fn app_nav(cx: &Cx, user_id: u64) -> Result<impl View> {
    let tabs = tabs(cx, current_chart(cx, user_id).await?);
    Ok(view! {
        <nav
            aria-label="Điều hướng chính"
            class="fixed inset-x-0 bottom-0 z-35 mx-auto flex max-w-120 gap-0.5 bg-linear-to-t from-page/97 via-page/72 via-70% to-transparent px-2 pt-2.25 pb-[calc(11px+env(safe-area-inset-bottom))] backdrop-blur-[14px] lg:inset-y-0 lg:right-auto lg:mx-0 lg:w-rail lg:max-w-none lg:flex-col lg:gap-1 lg:border-r lg:border-glass-line lg:bg-linear-to-b lg:from-page/75 lg:to-page/90 lg:px-4 lg:pt-5.5 lg:pb-5"
        >
            <div class="hidden px-2 pb-5 lg:block">brand()</div>
            for tab in tabs {
                tab_link(tab: tab)
            }
        </nav>
    })
}

/// One tab. A locked tab is an `<a>` without `href`: it is not a link.
#[topcoat::view::component]
async fn tab_link(cx: &Cx, tab: Tab) -> Result<impl View> {
    let locked = tab.href.is_none();
    Ok(view! {
        <a
            href=(tab.href)
            aria-current=(tab.current.then_some("page"))
            aria-disabled=(locked.then_some("true"))
            class=(class!(
                TAB,
                "text-accent lg:pill-primary" if tab.current,
                "text-muted hover:text-ink lg:hover:bg-glass" if !tab.current && !locked,
                "cursor-not-allowed text-muted opacity-40" if locked,
            ))
        >
            <span
                class=(class!(
                    "text-[23px] leading-none lg:text-xl",
                    "drop-shadow-[0_0_8px_var(--accent-glow)] lg:drop-shadow-none" if tab.current,
                ))
            >
                icon(data: tab.icon)
            </span>
            (tab.label)
            if tab.soon {
                <small
                    class="absolute -top-1 left-1/2 ml-1 rounded-full bg-bg-2 px-1.5 text-[8px] font-semibold text-ink lg:static lg:ml-auto lg:bg-glass lg:text-[10px]"
                >
                    "Sắp có"
                </small>
            }
        </a>
    })
}
