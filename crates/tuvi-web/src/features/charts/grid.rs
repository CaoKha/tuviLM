//! The chart drawing: 12 cung around a center panel.

use topcoat::{
    Result,
    context::Cx,
    view::{View, class, view},
};
use tuvi_core::{Chart, Cung, Gender};

/// Grid cell for each cung, indexed by Địa Chi (0 = Tý … 11 = Hợi). The chart is
/// a 4×4 grid read clockwise from Tị (top left); the middle 2×2 is the center
/// panel.
const CUNG_CELL: [&str; 12] = [
    "row-start-4 col-start-3", // Tý
    "row-start-4 col-start-2", // Sửu
    "row-start-4 col-start-1", // Dần
    "row-start-3 col-start-1", // Mão
    "row-start-2 col-start-1", // Thìn
    "row-start-1 col-start-1", // Tị
    "row-start-1 col-start-2", // Ngọ
    "row-start-1 col-start-3", // Mùi
    "row-start-1 col-start-4", // Thân
    "row-start-2 col-start-4", // Dậu
    "row-start-3 col-start-4", // Tuất
    "row-start-4 col-start-4", // Hợi
];

/// The whole chart. `name` is shown in the center panel.
#[topcoat::view::component]
pub async fn chart_grid(cx: &Cx, name: &str, chart: &Chart) -> Result<impl View> {
    Ok(view! {
        <div
            class="grid aspect-square grid-cols-4 grid-rows-4 gap-px overflow-hidden rounded-xl border border-border bg-border text-xs sm:text-sm"
        >
            for (cung, cell) in chart.cungs.iter().zip(CUNG_CELL) {
                cung_box(cung: cung, cell: cell)
            }
            center_panel(name: name, chart: chart)
        </div>
    })
}

#[topcoat::view::component]
async fn cung_box(cx: &Cx, cung: &Cung, cell: &'static str) -> Result<impl View> {
    Ok(view! {
        <section
            class=(class!("flex min-w-0 flex-col gap-1 bg-card p-1.5 sm:p-2", cell))
        >
            <header class="flex items-baseline justify-between gap-1">
                <h2 class="truncate font-semibold uppercase">
                    (cung.role)
                    if cung.is_than {
                        <span class="ml-1 text-primary">"(Thân)"</span>
                    }
                </h2>
                <span class="shrink-0 text-muted-foreground">
                    (cung.thien_can)
                    " "
                    (cung.dia_chi)
                </span>
            </header>
            <ul class="font-medium text-primary">
                for sao in &cung.chinh_tinh {
                    <li class="truncate">(*sao)</li>
                }
            </ul>
        </section>
    })
}

#[topcoat::view::component]
async fn center_panel(cx: &Cx, name: &str, chart: &Chart) -> Result<impl View> {
    let input = chart.input;
    let gender = match input.gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nữ",
    };
    Ok(view! {
        <section
            class="col-span-2 col-start-2 row-span-2 row-start-2 flex flex-col items-center justify-center gap-2 bg-background p-3 text-center"
        >
            <h1 class="text-base font-semibold sm:text-xl">(name)</h1>
            <dl class="grid grid-cols-[auto_auto] gap-x-3 gap-y-1 text-left">
                <dt class="text-muted-foreground">"Dương lịch"</dt>
                <dd class="tabular-nums">
                    (input.day)
                    "/"
                    (input.month)
                    "/"
                    (input.year)
                    " · "
                    (input.hour)
                    "h"
                </dd>
                <dt class="text-muted-foreground">"Âm lịch"</dt>
                <dd class="tabular-nums">
                    (chart.lunar_day)
                    "/"
                    (chart.lunar_month)
                    "/"
                    (chart.lunar_year)
                    " · giờ "
                    (chart.hour_chi)
                </dd>
                <dt class="text-muted-foreground">"Năm"</dt>
                <dd>
                    (chart.nam_can)
                    " "
                    (chart.nam_chi)
                </dd>
                <dt class="text-muted-foreground">"Giới tính"</dt>
                <dd>(gender)</dd>
                <dt class="text-muted-foreground">"Cục"</dt>
                <dd>(chart.cuc)</dd>
            </dl>
        </section>
    })
}
