//! `/ho-so`: the profile: account details, saved charts and log out.

use topcoat::{
    Result,
    context::Cx,
    router::{href, page},
    view::{View, view},
};

use super::chart_id;
use crate::features::{auth::require_user, charts::recent};

/// How many charts the profile lists.
const CHART_LIMIT: usize = 100;

#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let charts = recent(cx, user.id, CHART_LIMIT).await?;

    Ok(view! {
        <main class="mx-auto flex max-w-xl flex-col gap-6 px-4 pb-16">
            <section class="glass p-6">
                <p
                    class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
                >
                    "Hồ sơ"
                </p>
                <h1 class="mt-1 text-3xl font-semibold">
                    (user.display_name.as_str())
                </h1>
                <p class="mt-1 text-sm text-muted">(user.email.as_str())</p>
                <form method="post" action=(href!(super::logout::submit)) class="mt-5">
                    <button
                        type="submit"
                        class="pill-ghost h-11 w-full cursor-pointer rounded-full text-sm font-semibold"
                    >
                        "Đăng xuất"
                    </button>
                </form>
            </section>
            <section class="glass p-6">
                <h2 class="text-[22px] font-semibold">"Lá số đã lưu"</h2>
                if charts.is_empty() {
                    <p class="mt-3 text-sm font-light text-muted">
                        "Chưa có lá số nào."
                    </p>
                } else {
                    <ul class="mt-3 divide-y divide-glass-line">
                        for saved in &charts {
                            <li class="flex items-center justify-between gap-3 py-3">
                                <a
                                    href=(href!(super::charts::chart_id::show, chart_id: saved.id))
                                    class="min-w-0 truncate font-display text-base hover:text-accent"
                                >
                                    (saved.name.as_str())
                                </a>
                                <a
                                    href=(href!(super::charts::chart_id::hoi_ai::show, chart_id: saved.id))
                                    class="shrink-0 text-xs font-semibold text-accent"
                                >
                                    "Hỏi Thiên Hạc →"
                                </a>
                            </li>
                        }
                    </ul>
                }
            </section>
        </main>
    })
}
