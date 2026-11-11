//! The chart drawing: a summary row, the 12 cung around a center panel, and
//! one detail popup per cung.
//!
//! The popups use the HTML `popover` attribute: a cung is a button with
//! `popovertarget`. The browser opens and closes them, handles Esc and a tap
//! outside, and moves focus. No script is needed.

use topcoat::{
    Result,
    context::Cx,
    view::{View, class, view},
};
use tuvi_core::{Chart, Cung, Gender, TuHoa};

use super::accent::element;

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

/// The Tràng Sinh stages. The engine lists them with the phụ tinh; the chart
/// shows the stage on its own, on the bottom row of the cung.
const TRANG_SINH: [&str; 12] = [
    "Tràng Sinh",
    "Mộc Dục",
    "Quan Đới",
    "Lâm Quan",
    "Đế Vượng",
    "Suy",
    "Bệnh",
    "Tử",
    "Mộ",
    "Tuyệt",
    "Thai",
    "Dưỡng",
];

/// How many phụ tinh a cell shows before "+N".
const PHU_TINH_PREVIEW: usize = 2;

/// The usual reading order of Tứ Hóa. The engine lists them in id order.
const TU_HOA_ORDER: [&str; 4] = ["Hóa Lộc", "Hóa Quyền", "Hóa Khoa", "Hóa Kỵ"];

/// What a cell and its popup show, split from one [`Cung`].
struct CungView<'a> {
    cung: &'a Cung,
    /// Position in `Chart::cungs`, which is also the Địa Chi index.
    index: usize,
    trang_sinh: Option<&'static str>,
    phu_tinh: Vec<&'static str>,
    tu_hoa: Vec<&'a TuHoa>,
    /// "Cung Thân", "Tuần", "Triệt": the marks shown in coral.
    flags: Vec<&'static str>,
}

impl<'a> CungView<'a> {
    fn new(index: usize, cung: &'a Cung) -> Self {
        let (trang_sinh, phu_tinh) = cung
            .phu_tinh
            .iter()
            .partition::<Vec<_>, _>(|sao| TRANG_SINH.contains(sao));
        let mut tu_hoa = cung.tu_hoa.iter().collect::<Vec<_>>();
        tu_hoa.sort_by_key(|tu_hoa| TU_HOA_ORDER.iter().position(|hoa| *hoa == tu_hoa.hoa));
        let flags = [
            (cung.is_than, "Cung Thân"),
            (cung.tuan, "Tuần"),
            (cung.triet, "Triệt"),
        ]
        .into_iter()
        .filter_map(|(on, flag)| on.then_some(flag))
        .collect();
        Self {
            cung,
            index,
            trang_sinh: trang_sinh.first().copied(),
            phu_tinh,
            tu_hoa,
            flags,
        }
    }

    fn popup_id(&self) -> String {
        format!("cung-{}", self.index)
    }

    /// Âm/Dương of the cung's Địa Chi: Tý is Dương, then they alternate.
    fn am_duong(&self) -> &'static str {
        if self.index.is_multiple_of(2) {
            "Dương"
        } else {
            "Âm"
        }
    }

    fn is_menh(&self) -> bool {
        self.cung.role == "Mệnh"
    }

    /// "Lộc Tồn · Bác Sĩ · +5"
    fn phu_tinh_preview(&self) -> String {
        let mut preview = self
            .phu_tinh
            .iter()
            .take(PHU_TINH_PREVIEW)
            .copied()
            .collect::<Vec<_>>()
            .join(" · ");
        if let Some(more) = self
            .phu_tinh
            .len()
            .checked_sub(PHU_TINH_PREVIEW)
            .filter(|n| *n > 0)
        {
            preview.push_str(&format!(" · +{more}"));
        }
        preview
    }

    fn dai_han(&self) -> String {
        format!("{}–{}", self.cung.dai_han_start, self.cung.dai_han_end)
    }
}

/// The whole chart: a summary, then the 12 cung around a center panel.
#[topcoat::view::component]
pub async fn chart_grid(cx: &Cx, name: &str, chart: &Chart) -> Result<impl View> {
    let cungs = chart
        .cungs
        .iter()
        .enumerate()
        .map(|(index, cung)| CungView::new(index, cung))
        .collect::<Vec<_>>();

    Ok(view! {
        <section class="px-1 pb-4">
            <p class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase">
                "Lá số gốc"
            </p>
            <h1 class="mt-1 text-3xl leading-tight font-semibold">(name)</h1>
            <dl class="mt-4 grid grid-cols-4 border-y border-glass-line text-center">
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
                fact(label: "Bản mệnh", value: chart.ban_menh.to_owned())
            </dl>
        </section>
        <div
            class="grid grid-cols-4 grid-rows-[repeat(4,minmax(112px,auto))] overflow-hidden rounded-lg border border-glass-line bg-[rgba(14,8,32,0.34)] shadow-[0_18px_42px_rgba(8,4,20,0.28)] sm:grid-rows-[repeat(4,minmax(150px,auto))]"
        >
            for view in &cungs {
                cung_cell(view: view)
            }
            center_panel(name: name, chart: chart)
        </div>
        <p class="mt-3 text-center text-[10px] text-muted">
            "Chạm vào một cung để xem tất cả các sao."
        </p>
        for view in &cungs {
            cung_popup(view: view)
        }
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

/// One cung on the grid. A tap opens its popup. Cung Mệnh has an accent frame.
#[topcoat::view::component]
async fn cung_cell(cx: &Cx, view: &CungView<'_>) -> Result<impl View> {
    let cung = view.cung;
    Ok(view! {
        <button
            type="button"
            popovertarget=(view.popup_id())
            aria-label=(format!("Cung {} tại {}", cung.role, cung.dia_chi))
            class=(class!(
                "relative flex min-w-0 cursor-pointer flex-col overflow-hidden px-1.5 pt-2 pb-5 text-left shadow-[inset_-1px_-1px_0_var(--color-glass-line)] transition-colors focus-visible:outline-1 focus-visible:outline-accent sm:px-2.5 sm:pt-2.5",
                CUNG_CELL[view.index],
                "bg-white/[0.025] hover:bg-white/[0.055]" if !view.is_menh(),
                "z-[2] bg-[color-mix(in_srgb,var(--accent)_11%,rgba(26,16,51,0.78))] shadow-[inset_0_0_0_1.5px_var(--accent),inset_0_0_24px_color-mix(in_srgb,var(--accent)_9%,transparent)]" if view.is_menh(

                ),
            ))
        >
            <span class="flex min-h-6 items-start justify-between gap-1">
                <span
                    class="text-[9px] leading-tight font-bold break-words uppercase sm:text-xs"
                >
                    (cung.role)
                </span>
                <span
                    class="shrink-0 font-display text-xs leading-none text-muted sm:text-sm"
                >
                    (cung.dia_chi)
                </span>
            </span>
            <span class="mt-0.5 flex min-h-7 flex-col gap-px">
                for sao in &cung.chinh_tinh {
                    <span
                        class="font-display text-[11px] leading-tight font-semibold break-words text-accent sm:text-[15px]"
                    >
                        (*sao)
                    </span>
                }
                if cung.chinh_tinh.is_empty() {
                    <span class="text-[9px] font-medium text-muted italic sm:text-xs">
                        "Vô chính diệu"
                    </span>
                }
            </span>
            <span
                class="mt-1 line-clamp-2 block text-[7.5px] leading-snug text-muted sm:text-[11px]"
            >
                (view.phu_tinh_preview())
            </span>
            if !view.flags.is_empty() {
                <span
                    class="mt-1 block text-[7px] font-bold text-coral uppercase sm:text-[10px]"
                >
                    (view.flags.join(" · "))
                </span>
            }
            <span
                class="absolute right-1.5 bottom-1.5 left-1.5 flex justify-between gap-1 text-[7px] leading-none text-muted uppercase sm:right-2.5 sm:left-2.5 sm:text-[10px]"
            >
                <span class="truncate">(view.trang_sinh.unwrap_or(""))</span>
                <span class="tabular-nums">(view.dai_han())</span>
            </span>
        </button>
    })
}

/// The center of the chart: name, birth data and where Thân sits.
#[topcoat::view::component]
async fn center_panel(cx: &Cx, name: &str, chart: &Chart) -> Result<impl View> {
    let input = chart.input;
    let gender = match input.gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nữ",
    };
    let than = chart
        .cungs
        .iter()
        .find(|cung| cung.is_than)
        .map_or("", |cung| cung.role);
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
                <br>
                "Thân cư "
                (than)
            </p>
        </section>
    })
}

/// The detail popup of one cung: all its stars, Tứ Hóa, and its periods.
#[topcoat::view::component]
async fn cung_popup(cx: &Cx, view: &CungView<'_>) -> Result<impl View> {
    let cung = view.cung;
    let id = view.popup_id();
    Ok(view! {
        <div
            id=(id.as_str())
            popover="auto"
            aria-label=(format!("Cung {}", cung.role))
            class="m-auto max-h-[85dvh] w-[min(370px,calc(100%-32px))] overflow-y-auto rounded-lg border border-accent-glow bg-[linear-gradient(155deg,rgba(59,32,99,0.97),rgba(26,16,51,0.98))] p-5 text-ink shadow-[0_24px_70px_rgba(8,4,20,0.72),0_0_30px_color-mix(in_srgb,var(--accent)_12%,transparent)] backdrop:bg-[rgba(10,5,24,0.72)] backdrop:backdrop-blur-[7px]"
        >
            <button
                type="button"
                popovertarget=(id.as_str())
                popovertargetaction="hide"
                aria-label="Đóng"
                class="pill-ghost absolute top-3 right-3 grid size-8 cursor-pointer place-items-center rounded-full text-lg leading-none"
            >
                "×"
            </button>
            <header class="flex items-start justify-between gap-3 pt-6 pb-3.5">
                <div>
                    <p class="text-[10px] font-bold text-accent uppercase">
                        (cung.thien_can)
                        " "
                        (cung.dia_chi)
                        " · "
                        (view.am_duong())
                    </p>
                    <h3 class="mt-1 text-[26px] leading-tight font-semibold">
                        "Cung "
                        (cung.role)
                    </h3>
                    if !view.flags.is_empty() {
                        <p class="mt-2 text-[9px] font-bold text-coral uppercase">
                            (view.flags.join(" · "))
                        </p>
                    }
                </div>
                <p class="shrink-0 text-right text-[10px] leading-normal text-muted">
                    <b class="block text-xs font-semibold text-ink">
                        (view.trang_sinh.unwrap_or(""))
                    </b>
                    "Đại hạn "
                    (view.dai_han())
                    <br>
                    "Tiểu hạn năm "
                    (cung.tieu_han)
                </p>
            </header>
            <section class="border-t border-glass-line py-3.5">
                <h4 class="mb-2.5 font-sans text-[9px] font-bold text-muted uppercase">
                    "Chính tinh"
                </h4>
                <ul class="flex flex-wrap gap-x-3.5 gap-y-2">
                    for sao in &cung.chinh_tinh {
                        star_item(name: sao, major: true)
                    }
                    if cung.chinh_tinh.is_empty() {
                        <li class="font-display text-sm text-muted italic">
                            "Vô chính diệu"
                        </li>
                    }
                </ul>
                if !view.tu_hoa.is_empty() {
                    <p
                        class="mt-3 flex flex-wrap gap-2 text-[10px] font-semibold text-accent"
                    >
                        <b class="w-full text-[8px] text-muted uppercase">
                            "Tứ Hóa"
                        </b>
                        for tu_hoa in &view.tu_hoa {
                            <span>
                                "✦ "
                                (tu_hoa.hoa)
                                " · "
                                (tu_hoa.sao)
                            </span>
                        }
                    </p>
                }
            </section>
            <section class="border-t border-glass-line py-3.5">
                <h4 class="mb-2.5 font-sans text-[9px] font-bold text-muted uppercase">
                    "Phụ tinh"
                </h4>
                <ul class="flex flex-wrap gap-x-3.5 gap-y-2">
                    for sao in &view.phu_tinh {
                        star_item(name: sao, major: false)
                    }
                </ul>
            </section>
        </div>
    })
}

/// One star in a popup list, with a dot in its element's color when known.
#[topcoat::view::component]
async fn star_item(cx: &Cx, name: &'static str, major: bool) -> Result<impl View> {
    let color = element(name).map_or_else(
        || "--star-color: var(--color-muted)".to_owned(),
        |element| format!("--star-color: var(--element-{element})"),
    );
    Ok(view! {
        <li
            style=(color)
            class=(class!(
                "inline-flex items-center gap-1.5 leading-snug before:size-1.5 before:shrink-0 before:rounded-full before:bg-[var(--star-color)] before:shadow-[0_0_7px_var(--star-color)] before:content-['']",
                "font-display text-[15px] font-semibold" if major,
                "text-[11px]" if !major,
            ))
        >
            (name)
        </li>
    })
}
