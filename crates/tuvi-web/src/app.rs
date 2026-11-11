use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    font::{Font, RouterBuilderFontExt, fontsource::fontsource_font},
    router::{RouterBuilder, Slot, href, layout, module_router, page},
    runtime::RuntimeLayer,
    tailwind,
    view::{View, view},
};

use crate::features::charts::recent;

pub mod charts;

use charts::new_chart_form;
// `href!` needs the path parameter in scope.
use charts::chart_id::chart_id;

/// Display font for titles and star names.
const DISPLAY_FONT: Font = fontsource_font!(
    FRAUNCES,
    weight: [400, 600],
    style: Normal,
    subset: [Latin, Vietnamese],
);

/// Body font. It covers every Vietnamese diacritic.
const BODY_FONT: Font = fontsource_font!(
    BE_VIETNAM_PRO,
    weight: [300, 400, 600, 700],
    style: Normal,
    subset: [Latin, Vietnamese],
);

/// How many saved charts the home page lists.
const RECENT_LIMIT: usize = 20;

/// Builds the application's router.
pub fn router() -> RouterBuilder {
    module_router!()
        .font(DISPLAY_FONT)
        .font(BODY_FONT)
        .assets(AssetBundle::load().unwrap())
        // Lets the browser runtime rerun pages; register it after other layers.
        .layer(RuntimeLayer::new())
}

#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="vi" class="dark">
            <head>
                <meta charset="utf-8">
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, viewport-fit=cover"
                >
                <meta name="theme-color" content="#1a1033">
                <title>"Thiên Hạc · An sao & luận giải Tử Vi"</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::font::link(font: DISPLAY_FONT)
                topcoat::font::link(font: BODY_FONT)
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body>
                cosmos()
                <div class="relative z-10">
                    <header
                        class="mx-auto flex max-w-5xl items-baseline gap-3 px-4 pt-6 pb-5"
                    >
                        <a
                            href=(href!(home))
                            class="font-display text-2xl font-semibold"
                        >
                            "Thiên Hạc"
                        </a>
                        <span
                            class="text-[10px] font-bold tracking-[0.24em] text-muted uppercase"
                        >
                            "An sao & luận giải Tử Vi"
                        </span>
                    </header>
                    (slot)
                </div>
            </body>
        </html>
    })
}

/// The fixed backdrop: twilight gradient and two slow nebulae. The nebulae
/// are hidden on phones, where the content fills the screen.
#[topcoat::view::component]
async fn cosmos(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div aria-hidden="true" class="cosmos fixed inset-0 z-0 overflow-hidden">
            <div
                class="animate-breathe absolute top-[-15vmax] left-[-10vmax] h-[60vmax] w-[60vmax] rounded-full bg-[radial-gradient(circle,rgba(123,79,166,0.6),transparent_60%)] opacity-50 blur-[60px] max-sm:hidden"
            ></div>
            <div
                class="animate-breathe absolute right-[-12vmax] bottom-[-10vmax] h-[50vmax] w-[50vmax] rounded-full bg-[radial-gradient(circle,rgba(79,157,224,0.35),transparent_60%)] opacity-50 blur-[60px] [animation-delay:-4s] max-sm:hidden"
            ></div>
        </div>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let saved_charts = recent(cx, RECENT_LIMIT).await?;

    Ok(view! {
        <main class="mx-auto grid max-w-5xl gap-6 px-4 pb-16 md:grid-cols-2">
            new_chart_form(error: None)
            <section class="glass p-6">
                <p
                    class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
                >
                    "Lá số đã lưu"
                </p>
                <h2 class="mt-1 text-xl font-semibold">"Mới nhất ở trên"</h2>
                if saved_charts.is_empty() {
                    <p class="mt-4 text-sm font-light text-muted">
                        "Chưa có lá số nào."
                    </p>
                } else {
                    <ul class="mt-4 divide-y divide-glass-line">
                        for saved in &saved_charts {
                            <li>
                                <a
                                    href=(href!(charts::chart_id::show, chart_id: saved.id))
                                    class="flex justify-between gap-4 py-3 transition-colors hover:text-accent"
                                >
                                    <span class="truncate font-display text-base">
                                        (saved.name.as_str())
                                    </span>
                                    <span class="shrink-0 text-sm text-muted tabular-nums">
                                        (saved.day)
                                        "/"
                                        (saved.month)
                                        "/"
                                        (saved.year)
                                    </span>
                                </a>
                            </li>
                        }
                    </ul>
                }
            </section>
        </main>
    })
}
