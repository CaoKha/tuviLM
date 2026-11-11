//! The An sao form: the parts of the screen put together.
//!
//! Every control is a native radio group inside one `<form>`, so the form
//! posts without JavaScript. Signals copy the picked values for the clock
//! readout, the confirm summary and the preview shard.

use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    runtime::{expr, signal},
    view::{View, view},
};

use super::{
    clock::{birth_clock, canh_chi},
    confirm::confirm_dialog,
    loader::casting_loader,
    preview::menh_preview,
    segment::segment,
    wheel::wheel,
};
use crate::features::charts::{NewChart, gender_value};

/// Wheel scroll, clock drag, the confirm dialog and the loader delay.
const AN_SAO_JS: Asset = asset!("./an_sao.js");

/// The year wheel. The engine supports 1800–2199; the wheel keeps the years
/// people use, so it stays short.
const YEARS: std::ops::RangeInclusive<i32> = 1900..=2099;
const YEAR_DEFAULT: i32 = 2000;

/// Two digits, as the day and month wheels show them.
fn pad2(value: u8) -> String {
    format!("{value:02}")
}

/// The An sao screen. `action` is the URL the form posts to. `sent` and
/// `error` come back after a failed submit.
#[topcoat::view::component]
pub async fn an_sao_screen(
    cx: &Cx,
    action: String,
    sent: Option<&NewChart>,
    error: Option<String>,
) -> Result<impl View> {
    // The date parts stay strings, as the radios give them. The preview
    // shard parses and checks them on the server.
    let day = signal(cx, || pad2(sent.map_or(1, |sent| sent.day)));
    let month = signal(cx, || pad2(sent.map_or(1, |sent| sent.month)));
    let year = signal(cx, || {
        sent.map_or(YEAR_DEFAULT, |sent| sent.year).to_string()
    });
    let buoi = signal(cx, || {
        sent.map_or("AM", |sent| sent.buoi.value()).to_owned()
    });
    // Clock hour 1–12, or 0 before the first pick.
    let gio = signal(cx, || sent.and_then(|sent| sent.gio).map_or(0, usize::from));
    let pm = expr!(buoi.get() == "PM");
    let chi = canh_chi(&gio, pm.clone());

    Ok(view! {
        <form
            method="post"
            action=(action)
            autocomplete="off"
            class="grid gap-y-4 md:grid-cols-2 md:gap-x-10 md:gap-y-0"
            data-an-sao=""
        >
            <div
                class="text-center md:col-start-1 md:row-start-1 md:self-end md:text-left"
            >
                <p
                    class="text-[10px] font-bold tracking-[0.28em] text-accent uppercase [text-shadow:0_0_14px_var(--accent-glow)]"
                >
                    "An sao · lập lá số"
                </p>
                <h1
                    class="mt-[9px] text-[clamp(26px,7.4vw,31px)] leading-[1.08] font-semibold tracking-[-0.01em] md:text-[clamp(30px,4.4vw,38px)]"
                >
                    "Chạm vào bầu trời"
                    <br>
                    "ngày bạn sinh ra"
                </h1>
                <p
                    class="mt-[5px] text-[13.5px] font-light text-muted md:mt-[9px] md:max-w-[34ch] md:text-[15px]"
                >
                    "Xoay các vòng sao để nhập ngày sinh — chòm sao mệnh của bạn sẽ dần hiện lên."
                </p>
            </div>

            <div class="md:col-start-1 md:row-start-2 md:mt-6 md:self-start">
                menh_preview(
                    day: $(day.get()),
                    month: $(month.get()),
                    year: $(year.get()),
                    gio: $(gio.get()),
                    pm: $(pm)
                )
            </div>

            <div
                class="glass px-3.5 py-3 md:col-start-2 md:row-span-2 md:row-start-1 md:self-center"
            >
                <label class="block">
                    <span class="dial-label block">"Tên lá số"</span>
                    <input
                        name="name"
                        required=""
                        maxlength="80"
                        value=(sent.map_or("", |sent| sent.name.as_str()))
                        placeholder="Ví dụ: Minh An"
                        class="h-11 w-full rounded-full border border-glass-line bg-white/5 px-4 text-center font-display text-lg outline-none placeholder:text-muted/60 focus-visible:border-accent"
                    >
                </label>

                <div class="relative mt-3 grid grid-cols-[1fr_1fr_1.15fr] gap-2.5">
                    // The selection band behind the middle row of the wheels.
                    <div
                        class="pointer-events-none absolute inset-x-1.5 bottom-[55px] h-[46px] rounded-[14px] border border-accent-glow bg-linear-115 from-accent/16 to-accent-2/16"
                        aria-hidden="true"
                    ></div>
                    wheel(
                        name: "day",
                        label: "Ngày",
                        options: (1..=31).map(pad2).collect(),
                        picked: &day
                    )
                    wheel(
                        name: "month",
                        label: "Tháng",
                        options: (1..=12).map(pad2).collect(),
                        picked: &month
                    )
                    wheel(
                        name: "year",
                        label: "Năm",
                        options: YEARS.map(|year| year.to_string()).collect(),
                        picked: &year
                    )
                </div>

                <p class="dial-label mt-3">
                    "Giờ sinh "
                    <span class="text-accent">"· bắt buộc"</span>
                </p>
                segment(
                    name: "buoi",
                    label: "Buổi trong ngày",
                    options: [("AM", "AM"), ("PM", "PM")],
                    checked: Some(sent.map_or("AM", |sent| sent.buoi.value())),
                    picked: Some(&buoi),
                    class: "mx-auto w-[min(218px,66vw)]"
                )
                birth_clock(
                    checked: sent.and_then(|sent| sent.gio),
                    gio: &gio,
                    buoi: &buoi
                )
                <p class="mt-2 text-center text-[11px] text-muted">
                    "Vuốt trái phải hoặc chạm vào số để chọn giờ"
                </p>

                <div class="mt-3.5" data-gender="">
                    <p class="dial-label">
                        "Giới tính "
                        <span class="text-accent">"· bắt buộc"</span>
                    </p>
                    segment(
                        name: "gender",
                        label: "Giới tính",
                        options: [("Nam", "Nam"), ("Nu", "Nữ")],
                        checked: sent.and_then(|sent| sent.gender).map(gender_value),
                        picked: None
                    )
                </div>
            </div>

            <div
                class="flex flex-col gap-2.5 md:col-start-2 md:row-start-3 md:mt-[22px]"
            >
                if let Some(error) = &error {
                    <p role="alert" class="text-center text-sm text-fire">
                        (error.as_str())
                    </p>
                }
                <button
                    type="submit"
                    class="pill-primary h-[52px] rounded-full text-[15px] font-semibold"
                >
                    "✦ Luận giải lá số của tôi"
                </button>
            </div>

            confirm_dialog(
                // A no-break space keeps "Giờ Mùi" on one line.
                $(day.get())
                " · "
                $(month.get())
                " · "
                $(year.get())
                " · "
                $(gio.get())
                " "
                $(buoi.get())
                " · Giờ\u{a0}"
                $(chi)
            )
        </form>

        casting_loader()
        <div
            class="pointer-events-none fixed bottom-[calc(var(--spacing-tabbar)+22px)] left-1/2 z-40 w-max max-w-[calc(100%-32px)] -translate-x-1/2 translate-y-5 rounded-full border border-accent-glow bg-bg-1/95 px-[18px] py-3 text-[13.5px] opacity-0 transition data-show:translate-y-0 data-show:opacity-100"
            role="status"
            aria-live="polite"
            data-toast=""
        ></div>
        <script type="module" src=(AN_SAO_JS)></script>
    })
}
