//! The casting loader: orbit rings and the crane, shown by `an_sao.js`
//! between the confirm and the chart page.

use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    view::{View, view},
};

/// The crane. It has no alpha channel: `mix-blend-mode: screen` drops its
/// black background.
const CRANE_VIDEO: Asset = asset!("assets/golden_crane_loading_md.webm");

/// The lines under "đang luận giải". CSS shows them one after another.
const LINES: [&str; 3] = [
    "Thiên Hạc đang đọc vị trí các vì tinh tú…",
    "Đang xếp 12 cung mệnh của bạn…",
    "Chòm sao bản mệnh đang dần hiện hình…",
];

#[topcoat::view::component]
pub(super) async fn casting_loader(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div
            hidden=""
            class="cosmos fixed inset-0 z-50 flex flex-col items-center justify-center gap-[26px] p-10"
            data-casting=""
        >
            <div
                class="relative grid size-[190px] place-items-center"
                aria-hidden="true"
            >
                <div
                    class="absolute inset-0 animate-[spin_22s_linear_infinite_reverse] rounded-full border border-accent-glow opacity-40"
                ></div>
                <div
                    class="absolute inset-[22px] animate-[spin_14s_linear_infinite] rounded-full border border-dashed border-accent-glow opacity-40"
                ></div>
                <div
                    class="absolute top-[-4px] left-1/2 -ml-1.5 size-3 origin-[50%_99px] animate-[spin_6s_linear_infinite] rounded-full bg-accent shadow-[0_0_16px_var(--accent-glow)]"
                ></div>
                <video
                    src=(CRANE_VIDEO)
                    muted=""
                    loop=""
                    playsinline=""
                    preload="auto"
                    class="size-[172px] mix-blend-screen"
                    data-crane=""
                ></video>
            </div>
            <div class="text-center">
                <p class="font-display text-[26px] font-medium">
                    "đang luận giải"
                    <span class="load-dots" aria-hidden="true">
                        <span>"."</span>
                        <span>"."</span>
                        <span>"."</span>
                    </span>
                </p>
                <div class="load-lines mt-2.5 grid">
                    for line in LINES {
                        <p
                            class="col-start-1 row-start-1 text-sm font-light text-muted"
                        >
                            (line)
                        </p>
                    }
                </div>
            </div>
        </div>
    })
}
