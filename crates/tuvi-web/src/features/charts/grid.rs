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

/// The whole chart: a summary, then the 12 cung around a center panel.
#[topcoat::view::component]
pub async fn chart_grid(cx: &Cx, name: &str, chart: &Chart) -> Result<impl View> {
    Ok(view! {
        <section class="px-1 pb-4">
            <p class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase">
                "Lá số gốc"
            </p>
            <h1 class="mt-1 text-3xl leading-tight font-semibold">(name)</h1>
            <dl class="mt-4 grid grid-cols-3 border-y border-glass-line text-center">
                fact(
                    label: "Âm lịch",
                    value: format!(
                        "{}/{}/{}",
                        chart.lunar_day,
                        chart.lunar_month,
                        chart.lunar_year,
                    )
                )
                fact(
                    label: "Năm",
                    value: format!("{} {}", chart.nam_can, chart.nam_chi)
                )
                fact(label: "Cục", value: chart.cuc.to_owned())
            </dl>
        </section>
        <div
            class="grid grid-cols-4 grid-rows-[repeat(4,minmax(112px,auto))] overflow-hidden rounded-lg border border-glass-line bg-[rgba(14,8,32,0.34)] shadow-[0_18px_42px_rgba(8,4,20,0.28)] sm:grid-rows-[repeat(4,minmax(150px,auto))]"
        >
            for (cung, cell) in chart.cungs.iter().zip(CUNG_CELL) {
                cung_box(cung: cung, cell: cell)
            }
            center_panel(name: name, chart: chart)
        </div>
    })
}

/// One fact in the summary row.
#[topcoat::view::component]
async fn fact(cx: &Cx, label: &'static str, value: String) -> Result<impl View> {
    Ok(view! {
        <div class="min-w-0 px-1.5 py-3 [&+&]:border-l [&+&]:border-glass-line">
            <dt class="text-[8px] text-muted uppercase sm:text-[10px]">(label)</dt>
            <dd class="mt-1 text-[11px] font-semibold break-words sm:text-sm">
                (value)
            </dd>
        </div>
    })
}

/// One cung. Cung Mệnh is highlighted with the accent.
#[topcoat::view::component]
async fn cung_box(cx: &Cx, cung: &Cung, cell: &'static str) -> Result<impl View> {
    let is_menh = cung.role == "Mệnh";
    Ok(view! {
        <section
            class=(class!(
                "relative min-w-0 overflow-hidden px-1.5 pt-2 pb-5 shadow-[inset_-1px_-1px_0_var(--color-glass-line)] sm:px-2.5 sm:pt-2.5",
                cell,
                "bg-white/[0.025]" if !is_menh,
                "z-[2] bg-[color-mix(in_srgb,var(--accent)_11%,rgba(26,16,51,0.78))] shadow-[inset_0_0_0_1.5px_var(--accent),inset_0_0_24px_color-mix(in_srgb,var(--accent)_9%,transparent)]" if is_menh,
            ))
        >
            <header class="flex min-h-6 items-start justify-between gap-1">
                <h2
                    class="font-sans text-[9px] leading-tight font-bold break-words uppercase sm:text-xs"
                >
                    (cung.role)
                </h2>
                <span
                    class="shrink-0 font-display text-xs leading-none text-muted sm:text-sm"
                >
                    (cung.dia_chi)
                </span>
            </header>
            <ul class="mt-0.5 flex min-h-7 flex-col gap-px">
                for sao in &cung.chinh_tinh {
                    <li
                        class="font-display text-[11px] leading-tight font-semibold break-words text-accent sm:text-[15px]"
                    >
                        (*sao)
                    </li>
                }
                if cung.chinh_tinh.is_empty() {
                    <li class="text-[9px] font-medium text-muted italic sm:text-xs">
                        "Vô chính diệu"
                    </li>
                }
            </ul>
            if cung.is_than {
                <p
                    class="mt-1 text-[7px] font-bold text-coral uppercase sm:text-[10px]"
                >
                    "Thân"
                </p>
            }
            <footer
                class="absolute right-1.5 bottom-1.5 left-1.5 text-[7px] leading-none text-muted uppercase sm:text-[10px]"
            >
                (cung.thien_can)
            </footer>
        </section>
    })
}

/// The center of the chart: name, birth data and gender.
#[topcoat::view::component]
async fn center_panel(cx: &Cx, name: &str, chart: &Chart) -> Result<impl View> {
    let input = chart.input;
    let gender = match input.gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nữ",
    };
    Ok(view! {
        <section
            class="col-span-2 col-start-2 row-span-2 row-start-2 flex flex-col items-center justify-center bg-[radial-gradient(circle_at_50%_40%,color-mix(in_srgb,var(--accent)_14%,transparent),transparent_52%),rgba(26,16,51,0.76)] px-3 py-4 text-center shadow-[inset_-1px_-1px_0_var(--color-glass-line)]"
        >
            <span
                aria-hidden="true"
                class="text-2xl text-accent drop-shadow-[0_0_8px_var(--accent-glow)]"
            >
                "✦"
            </span>
            <h2 class="mt-1 text-xl font-semibold sm:text-2xl">(name)</h2>
            <p class="mt-2 text-[9px] leading-relaxed text-muted uppercase sm:text-xs">
                (input.day)
                "/"
                (input.month)
                "/"
                (input.year)
                " · "
                (input.hour)
                "h"
                <br>
                "Giờ "
                (chart.hour_chi)
                " · "
                (gender)
            </p>
        </section>
    })
}
