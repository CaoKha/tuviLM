//! The birth clock: 12 hour radios on a face, and a readout in the middle
//! with the canh giờ (two-hour block) of the picked hour.

use std::f64::consts::PI;

use topcoat::{
    Result,
    context::Cx,
    runtime::{Expr, Signal, expr},
    view::{View, view},
};

/// The Địa Chi of each canh giờ, Tý (23:00–01:00) first.
const CHI: [&str; 12] = [
    "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi",
];

/// The range of each canh giờ, Tý first.
const RANGE: [&str; 12] = [
    "23:00 – 01:00",
    "01:00 – 03:00",
    "03:00 – 05:00",
    "05:00 – 07:00",
    "07:00 – 09:00",
    "09:00 – 11:00",
    "11:00 – 13:00",
    "13:00 – 15:00",
    "15:00 – 17:00",
    "17:00 – 19:00",
    "19:00 – 21:00",
    "21:00 – 23:00",
];

/// The index of the picked hour's canh giờ in `CHI` and `RANGE`.
fn canh_index(gio: &Signal<usize>, pm: Expr<bool>) -> Expr<usize> {
    expr!({
        let hour = gio.get() % 12 + if pm { 12 } else { 0 };
        (hour + 1) % 24 / 2
    })
}

/// The Địa Chi of the picked hour's canh giờ, as a runtime expression.
pub(super) fn canh_chi(gio: &Signal<usize>, pm: Expr<bool>) -> Expr<String> {
    let index = canh_index(gio, pm);
    // Runtime expressions index owned strings, not `&str` arrays.
    let chi = CHI.map(str::to_owned);
    expr!(chi.index(index).to_owned())
}

/// The hours in face order (12 at the top, then clockwise), each with its
/// inline position on the face.
fn hours() -> impl Iterator<Item = (usize, String)> {
    (0..12usize).map(|position| {
        let angle = position as f64 * PI / 6.0;
        let (x, y) = (50.0 + angle.sin() * 39.0, 50.0 - angle.cos() * 39.0);
        let hour = if position == 0 { 12 } else { position };
        (hour, format!("left: {x:.2}%; top: {y:.2}%"))
    })
}

/// The clock. `checked` is the hour (1–12) the form starts with. `gio`
/// holds the picked hour, or 0 before the first pick.
#[topcoat::view::component]
pub(super) async fn birth_clock(
    cx: &Cx,
    checked: Option<u8>,
    gio: &Signal<usize>,
    buoi: &Signal<String>,
) -> Result<impl View> {
    let pm = expr!(buoi.get() == "PM");
    let chi = canh_chi(gio, pm.clone());
    let index = canh_index(gio, pm);
    let range = RANGE.map(str::to_owned);
    let range = expr!(range.index(index).to_owned());
    Ok(view! {
        <div
            class="birth-clock group"
            role="radiogroup"
            aria-label="Giờ sinh"
            data-clock=""
        >
            <div class="clock-face" aria-hidden="true"></div>
            <div class="clock-ticks" aria-hidden="true"></div>
            <div class="clock-hand" aria-hidden="true"></div>
            for (hour, position) in hours() {
                <label class="clock-hour" style=(position)>
                    <input
                        type="radio"
                        name="gio"
                        value=(hour)
                        required=(hour == 12)
                        checked=(checked.map(usize::from) == Some(hour))
                        class="sr-only"
                        @change=$(|_e| gio.set(hour))
                    >
                    (hour)
                </label>
            }
            <span
                class="pointer-events-none absolute top-1/2 left-1/2 z-5 size-3 -translate-1/2 rounded-full border-3 border-bg-1 bg-accent shadow-[0_0_13px_var(--accent-glow)]"
                aria-hidden="true"
            ></span>
            <span
                class="pointer-events-none absolute top-1/2 left-1/2 z-3 flex w-[104px] -translate-1/2 flex-col items-center text-[9px] leading-tight whitespace-nowrap text-muted"
                aria-hidden="true"
            >
                <strong
                    class="mb-[23px] font-display text-xl leading-[1.1] font-semibold text-ink group-has-[:checked]:text-accent group-has-[:checked]:[text-shadow:0_0_12px_var(--accent-glow)]"
                >
                    // Without JavaScript the readout cannot follow the pick, so
                    // CSS hides the empty state at least.
                    <span class="group-has-[:checked]:hidden" :hidden=$(gio.get() != 0)>
                        "Chọn giờ"
                    </span>
                    <span :hidden=$(gio.get() == 0)>
                        $(gio.get())
                        " "
                        $(buoi.get())
                    </span>
                </strong>
                <span :hidden=$(gio.get() == 0)>
                    "Giờ "
                    $(chi)
                </span>
                <span :hidden=$(gio.get() == 0)>$(range)</span>
                <span class="group-has-[:checked]:hidden" :hidden=$(gio.get() != 0)>
                    "--"
                </span>
            </span>
        </div>
    })
}
