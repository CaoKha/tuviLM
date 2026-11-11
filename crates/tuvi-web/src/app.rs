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

use crate::{
    components::card::{card, card_content, card_description, card_header, card_title},
    features::charts::recent,
};

pub mod charts;

use charts::new_chart_form;
// `href!` needs the path parameter in scope.
use charts::chart_id::chart_id;

/// Be Vietnam Pro covers every Vietnamese diacritic.
const BODY_FONT: Font = fontsource_font!(BE_VIETNAM_PRO, weight: [400, 600], style: Normal);

/// How many saved charts the home page lists.
const RECENT_LIMIT: usize = 20;

/// Builds the application's router.
pub fn router() -> RouterBuilder {
    module_router!()
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
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"TuviLM"</title>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::font::link(font: BODY_FONT)
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body class="min-h-dvh bg-background text-foreground antialiased">
                <header class="mx-auto flex max-w-5xl items-center px-4 py-5">
                    <a href=(href!(home)) class="text-lg font-semibold tracking-tight">
                        "TuviLM"
                    </a>
                </header>
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let saved_charts = recent(cx, RECENT_LIMIT).await?;

    Ok(view! {
        <main class="mx-auto grid max-w-5xl gap-6 px-4 pb-16 md:grid-cols-[1fr_1fr]">
            new_chart_form(error: None)
            card(
                card_header(
                    card_title("Lá số đã lưu")
                    card_description("Mới nhất ở trên.")
                )
                card_content(
                    if saved_charts.is_empty() {
                        <p class="text-sm text-muted-foreground">
                            "Chưa có lá số nào."
                        </p>
                    } else {
                        <ul class="divide-y divide-border">
                            for saved in &saved_charts {
                                <li>
                                    <a
                                        href=(href!(charts::chart_id::show, chart_id: saved.id))
                                        class="flex justify-between gap-4 py-2 hover:text-primary"
                                    >
                                        <span class="truncate">(saved.name.as_str())</span>
                                        <span
                                            class="shrink-0 text-sm text-muted-foreground tabular-nums"
                                        >
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
                )
            )
        </main>
    })
}
