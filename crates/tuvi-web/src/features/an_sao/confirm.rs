//! The confirm dialog before casting. `an_sao.js` opens it on submit.

use topcoat::{
    Result,
    context::Cx,
    view::{Child, View, class, view},
};

/// Classes of the two dialog buttons.
const BUTTON: &str = "rounded-full px-2.5 py-[13px] text-[14.5px] font-semibold whitespace-nowrap";

/// The dialog. `child` is the birth data summary. It sits inside the form:
/// "Đúng rồi, an sao" is a real submit button.
#[topcoat::view::component]
pub(super) async fn confirm_dialog(cx: &Cx, child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <dialog class="confirm-card" aria-labelledby="an-sao-confirm" data-confirm="">
            <p
                aria-hidden="true"
                class="text-[26px] text-accent drop-shadow-[0_0_14px_var(--accent-glow)]"
            >
                "✦"
            </p>
            <h3
                id="an-sao-confirm"
                class="mt-2.5 text-[23px] leading-[1.15] font-semibold"
            >
                "Xác nhận ngày sinh"
            </h3>
            <p
                class="mt-[13px] font-display text-[21px] leading-tight font-semibold tracking-[0.02em] text-accent"
            >
                (child)
            </p>
            <p class="mt-[11px] text-[13.5px] leading-relaxed text-muted">
                "Lá số sẽ được an theo đúng ngày giờ này và lưu vào danh sách lá số của bạn."
            </p>
            <label
                class="mt-[18px] inline-flex cursor-pointer items-center gap-[9px] text-[13px] text-muted select-none"
            >
                <input
                    type="checkbox"
                    class="size-[17px] cursor-pointer accent-accent"
                    data-mute=""
                >
                "Không nhắc lại"
            </label>
            <div class="mt-[22px] grid grid-cols-2 gap-2.5">
                <button
                    type="button"
                    class=(class!("pill-ghost", BUTTON))
                    data-cancel=""
                >
                    "Sửa lại"
                </button>
                <button
                    type="submit"
                    class=(class!("pill-primary", BUTTON))
                    data-cast=""
                >
                    "Đúng rồi, an sao"
                </button>
            </div>
        </dialog>
    })
}
