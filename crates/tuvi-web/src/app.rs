use serde_json::json;
use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset, asset_config},
    context::Cx,
    cookie::CookieLayer,
    font::{Font, RouterBuilderFontExt, fontsource::fontsource_font},
    router::{RouterBuilder, Slot, content::Json, href, layout, module_router, page, route},
    runtime::RuntimeLayer,
    tailwind,
    view::{View, view},
};

use crate::features::{
    an_sao::{an_sao_screen, menh_preview},
    auth::current_user,
    charts::recent,
};

pub mod charts;
pub mod ho_so;
pub mod login;
pub mod logout;
pub mod register;

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

/// Browser tab icons and the home screen icons, from the old app.
const FAVICON: Asset = asset!("assets/icons/favicon.ico");
const FAVICON_16: Asset = asset!("assets/icons/favicon-16x16.png");
const FAVICON_32: Asset = asset!("assets/icons/favicon-32x32.png");
const FAVICON_48: Asset = asset!("assets/icons/favicon-48x48.png");
const APPLE_TOUCH_ICON: Asset = asset!("assets/icons/apple-touch-icon.png");
const ANDROID_ICON_192: Asset = asset!("assets/icons/android-chrome-192x192.png");
const ANDROID_ICON_512: Asset = asset!("assets/icons/android-chrome-512x512.png");
const MASKABLE_ICON_512: Asset = asset!("assets/icons/maskable-icon-512x512.png");

/// How many saved charts the home page lists.
const RECENT_LIMIT: usize = 20;

/// Builds the application's router.
pub fn router() -> RouterBuilder {
    module_router!()
        .font(DISPLAY_FONT)
        .font(BODY_FONT)
        .assets(AssetBundle::load().unwrap())
        .route(web_manifest)
        // The live preview of the An sao screen.
        .route(menh_preview)
        // Reads and writes the session cookie.
        .layer(CookieLayer::new())
        // Lets the browser runtime rerun pages; register it after other layers.
        .layer(RuntimeLayer::new())
}

/// `GET /manifest.webmanifest`, for "Add to home screen". It is built per
/// request because the icon URLs carry a content hash.
#[route(GET "/manifest.webmanifest")]
async fn web_manifest(cx: &Cx) -> Result<Json<serde_json::Value>> {
    let icon = |asset, sizes, purpose| json!({ "src": asset_config(cx).resolve(cx, asset), "sizes": sizes, "type": "image/png", "purpose": purpose });
    Ok(Json(json!({
        "name": "Thiên Hạc",
        "short_name": "Thiên Hạc",
        "description": "Thiên Hạc: lá số và luận giải Tử Vi",
        "start_url": "/",
        "display": "standalone",
        // A manifest cannot read CSS variables: these are the `page` and
        // `bg-0` tokens of styles.css.
        "background_color": "#120a24",
        "theme_color": "#1a1033",
        "icons": [
            icon(ANDROID_ICON_192, "192x192", "any"),
            icon(ANDROID_ICON_512, "512x512", "any"),
            icon(MASKABLE_ICON_512, "512x512", "maskable"),
        ],
    })))
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
                <meta
                    name="description"
                    content="An sao, đọc lá số và trò chuyện cùng Thiên Hạc."
                >
                <link rel="icon" href=(FAVICON) sizes="any">
                <link rel="icon" type="image/png" sizes="16x16" href=(FAVICON_16)>
                <link rel="icon" type="image/png" sizes="32x32" href=(FAVICON_32)>
                <link rel="icon" type="image/png" sizes="48x48" href=(FAVICON_48)>
                <link rel="apple-touch-icon" sizes="180x180" href=(APPLE_TOUCH_ICON)>
                <link rel="manifest" href="/manifest.webmanifest">
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
                            class="flex items-center gap-2 self-center font-display text-2xl font-semibold"
                        >
                            <img
                                src=(FAVICON_48)
                                alt=""
                                width="48"
                                height="48"
                                class="size-9 drop-shadow-[0_0_10px_var(--accent-glow)]"
                            >
                            "Thiên Hạc"
                        </a>
                        <span
                            class="text-[10px] font-bold tracking-[0.24em] text-muted uppercase max-sm:hidden"
                        >
                            "An sao & luận giải Tử Vi"
                        </span>
                        account_link()
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

/// Top right: the profile link when logged in, else the login link.
#[topcoat::view::component]
async fn account_link(cx: &Cx) -> Result<impl View> {
    let user = current_user(cx).await?;
    Ok(view! {
        if let Some(user) = &user {
            <a
                href=(href!(ho_so::page))
                class="pill-ghost ml-auto max-w-40 truncate rounded-full px-4 py-1.5 text-xs font-semibold"
            >
                (user.display_name.as_str())
            </a>
        } else {
            <a
                href=(href!(login::page))
                class="pill-ghost ml-auto rounded-full px-4 py-1.5 text-xs font-semibold"
            >
                "Đăng nhập"
            </a>
        }
    })
}

/// Logged out: what the app is, and the way in.
#[topcoat::view::component]
async fn landing(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <main
            class="mx-auto flex max-w-xl flex-col items-center px-4 pt-10 pb-16 text-center"
        >
            <span
                aria-hidden="true"
                class="text-4xl text-accent drop-shadow-[0_0_12px_var(--accent-glow)]"
            >
                "✦"
            </span>
            <h1 class="mt-3 text-4xl leading-tight font-semibold">
                "An sao & luận giải Tử Vi"
            </h1>
            <p class="mt-4 text-[15px] leading-relaxed font-light text-muted">
                "Lập lá số từ ngày giờ sinh, xem đủ 12 cung với chính tinh, phụ tinh, Tứ Hóa và vận hạn, rồi hỏi Thiên Hạc về bất cứ cung nào."
            </p>
            <div class="mt-8 grid w-full max-w-sm grid-cols-2 gap-3">
                <a
                    href=(href!(register::page))
                    class="pill-primary flex h-12 items-center justify-center rounded-full text-sm font-semibold"
                >
                    "Tạo tài khoản"
                </a>
                <a
                    href=(href!(login::page))
                    class="pill-ghost flex h-12 items-center justify-center rounded-full text-sm font-semibold"
                >
                    "Đăng nhập"
                </a>
            </div>
        </main>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let user = current_user(cx).await?;
    let saved_charts = match &user {
        Some(user) => recent(cx, user.id, RECENT_LIMIT).await?,
        None => Vec::new(),
    };

    Ok(view! {
        if user.is_none() {
            landing()
        } else {
            <main class="mx-auto max-w-5xl px-4 pb-16">
                an_sao_screen(
                    action: href!(charts::create).resolve(cx),
                    sent: None,
                    error: None
                )
                <section class="glass mt-10 p-6">
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
        }
    })
}
