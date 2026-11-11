//! Lá bài phụ 1, Vận May: where Hóa Lộc and Lộc Tồn sit, the lucky color
//! from the ngũ hành of Bản mệnh, and the tháng cát. The card takes the
//! lucky color as its hue. It does not change the page accent.

use topcoat::{
    Result,
    context::Cx,
    icon::iconify::iconify_icon,
    view::{View, view},
};

use super::{
    card::{BODY, tarot},
    facts::{Facts, loc_blurb},
};

#[topcoat::view::component]
pub(super) async fn luck_card(cx: &Cx, facts: &Facts<'_>) -> Result<impl View> {
    let hue = facts.ban_menh_hanh.css();
    let key = match (facts.hoa_loc, facts.loc_ton) {
        (Some(hoa_loc), Some(loc_ton)) if hoa_loc == loc_ton => format!("Lộc tụ tại {hoa_loc}"),
        _ => "Hai nguồn lộc".to_owned(),
    };
    let locs = [("Hóa Lộc", facts.hoa_loc), ("Lộc Tồn", facts.loc_ton)];
    let months = facts.thang_cat.map(|month| month.to_string()).join(" · ");
    Ok(view! {
        tarot(
            hue: format!("var(--element-{hue})"),
            icon: iconify_icon!("lucide:star"),
            title: "Vận May",
            key: key,
            tag: format!("Tháng cát: {months} (âm lịch)"),
            for (sao, role) in locs {
                if let Some(role) = role {
                    <p class=(BODY)>
                        <b class="font-semibold text-ink">
                            (sao)
                            " tại "
                            (role)
                            "."
                        </b>
                        " "
                        (loc_blurb(role))
                    </p>
                }
            }
            <div
                class="mt-3 flex items-center gap-2.75 rounded-full border border-glass-line bg-glass py-2 pr-3.5 pl-2.25 text-[13px] text-muted"
            >
                <span
                    aria-hidden="true"
                    class="size-6.5 shrink-0 rounded-full bg-[radial-gradient(circle_at_34%_28%,var(--color-ink),var(--t-hue)_60%)] shadow-[0_0_14px_color-mix(in_srgb,var(--t-hue)_55%,transparent)]"
                ></span>
                "Màu may mắn"
                <b class="ml-auto text-sm font-semibold text-ink">
                    (facts.ban_menh_hanh.luck_color())
                </b>
            </div>
            <p class=(BODY)>
                "Theo nạp âm bản mệnh "
                <b class="font-semibold text-ink">(facts.ban_menh)</b>
                "."
            </p>
        )
    })
}
