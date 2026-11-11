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

use crate::components::card::{card, card_description, card_header, card_title};

/// Be Vietnam Pro covers every Vietnamese diacritic.
const BODY_FONT: Font = fontsource_font!(BE_VIETNAM_PRO, weight: [400, 600], style: Normal);

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
                    <a href=(href!(home)) class="text-lg font-semibold tracking-tight">"TuviLM"</a>
                </header>
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-5xl px-4 pb-16">
            card(
                card_header(
                    card_title("TuviLM")
                    card_description("Lá số Tử Vi.")
                )
            )
        </main>
    })
}
