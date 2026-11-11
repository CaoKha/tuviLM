use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    font::{Font, RouterBuilderFontExt, fontsource::fontsource_font},
    router::{RouterBuilder, Slot, href, layout, module_router, page},
    runtime::RuntimeLayer,
    tailwind,
    view::{View, attributes, view},
};

use crate::{
    components::{
        button::button,
        card::{card, card_content, card_description, card_header, card_title},
        field::{field, field_group, field_label},
        input::input,
    },
    features::charts as chart_store,
};


pub mod charts;

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
                    <a href=(href!(home)) class="text-lg font-semibold tracking-tight">"TuviLM"</a>
                </header>
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let recent = chart_store::recent(cx, RECENT_LIMIT).await?;

    Ok(view! {
        <main class="mx-auto grid max-w-5xl gap-6 px-4 pb-16 md:grid-cols-[1fr_1fr]">
            new_chart_form(error: None)
            card(
                card_header(
                    card_title("Lá số đã lưu")
                    card_description("Mới nhất ở trên.")
                )
                card_content(
                    if recent.is_empty() {
                        <p class="text-sm text-muted-foreground">"Chưa có lá số nào."</p>
                    } else {
                        <ul class="divide-y divide-border">
                            for saved in &recent {
                                <li>
                                    <a
                                        href=(href!(charts::chart_id::show, chart_id: saved.id))
                                        class="flex justify-between gap-4 py-2 hover:text-primary"
                                    >
                                        <span class="truncate">(saved.name.as_str())</span>
                                        <span class="shrink-0 text-sm text-muted-foreground tabular-nums">
                                            (saved.day) "/" (saved.month) "/" (saved.year)
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

/// The "new chart" form. `error` is shown above the button after a failed submit.
#[topcoat::view::component]
pub async fn new_chart_form(cx: &Cx, error: Option<String>) -> Result<impl View> {
    Ok(view! {
        card(
            card_header(
                card_title("Lập lá số")
                card_description("Ngày sinh dương lịch, giờ Việt Nam.")
            )
            card_content(
                <form method="post" action=(href!(charts::create))>
                    field_group(
                        field(
                            field_label(attrs: attributes! { for="name" }, "Tên")
                            input(attrs: attributes! { id="name" name="name" required="" maxlength="80" })
                        )
                        field(
                            field_label(attrs: attributes! { for="birth_date" }, "Ngày sinh")
                            input(attrs: attributes! {
                                id="birth_date" name="birth_date" type="date" required=""
                                min="1800-01-25" max="2199-12-31"
                            })
                        )
                        field(
                            field_label(attrs: attributes! { for="hour" }, "Giờ sinh")
                            <select
                                id="hour"
                                name="hour"
                                class="h-9 rounded-md border border-border bg-transparent px-3 text-sm"
                            >
                                for hour in 0u8..24 {
                                    <option value=(hour)>(format!("{hour:02}:00 – {hour:02}:59"))</option>
                                }
                            </select>
                        )
                        <fieldset class="flex gap-6">
                            <legend class="mb-2 text-sm font-medium">"Giới tính"</legend>
                            <label class="flex items-center gap-2 text-sm">
                                <input type="radio" name="gender" value="Nam" checked="">
                                "Nam"
                            </label>
                            <label class="flex items-center gap-2 text-sm">
                                <input type="radio" name="gender" value="Nu">
                                "Nữ"
                            </label>
                        </fieldset>
                        if let Some(error) = &error {
                            <p role="alert" class="text-sm text-destructive">(error.as_str())</p>
                        }
                        button(attrs: attributes! { type="submit" }, "Lập lá số")
                    )
                </form>
            )
        )
    })
}
