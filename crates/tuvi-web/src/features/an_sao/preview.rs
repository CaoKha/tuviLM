//! The live preview: the chính tinh of cung Mệnh for the picked birth data.
//!
//! The browser re-renders the shard each time a wheel settles or the clock
//! moves. The arguments come from the browser, so the shard parses and checks
//! each one, and `tuvi_core::build_chart` checks the date.

use topcoat::{
    Result,
    context::Cx,
    runtime::shard,
    view::{View, class, view},
};
use tuvi_core::{BirthInput, ChartError, Gender, build_chart};

use crate::features::charts::{Buoi, accent_style, hour_24, menh_chinh_tinh};

/// Scattered dots for the sky before the stars wake, in a 320×100 box.
const SCATTER: [(u16, u16); 10] = [
    (26, 34),
    (66, 62),
    (108, 40),
    (150, 72),
    (192, 34),
    (232, 66),
    (272, 46),
    (300, 56),
    (90, 80),
    (210, 22),
];

/// What the preview shows.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Preview {
    /// No hour yet: the constellation sleeps.
    Ngu,
    /// The date is not on the calendar.
    NgoaiLich,
    /// The year is outside 1800–2199.
    NgoaiKhoang,
    /// The chart builds. `chinh_tinh` is empty for vô chính diệu.
    Menh {
        chinh_tinh: Vec<&'static str>,
        accent: Option<String>,
    },
}

/// Builds the preview from the raw shard arguments. Never fails: bad input
/// gives a state to show.
pub(super) fn preview(day: &str, month: &str, year: &str, gio: usize, pm: bool) -> Preview {
    let buoi = if pm { Buoi::Chieu } else { Buoi::Sang };
    let Some(hour) = u8::try_from(gio).ok().and_then(|gio| hour_24(gio, buoi)) else {
        return Preview::Ngu;
    };
    let (Ok(day), Ok(month), Ok(year)) = (
        day.trim().parse::<u8>(),
        month.trim().parse::<u8>(),
        year.trim().parse::<i32>(),
    ) else {
        return Preview::NgoaiLich;
    };
    let birth = BirthInput {
        year,
        month,
        day,
        hour,
        // Gender changes the Đại hạn direction, not cung Mệnh.
        gender: Gender::Nam,
    };
    match build_chart(birth) {
        Ok(chart) => Preview::Menh {
            chinh_tinh: menh_chinh_tinh(&chart).to_vec(),
            accent: accent_style(&chart),
        },
        Err(ChartError::InvalidDate) => Preview::NgoaiLich,
        Err(ChartError::YearOutOfRange) => Preview::NgoaiKhoang,
    }
}

/// The reward under the title: the sky, the star names and a caption. It
/// also sets the page accent with a `<style>` rule, so the whole screen
/// takes the color of the chart.
#[shard("/an-sao/menh")]
pub async fn menh_preview(
    cx: &Cx,
    day: String,
    month: String,
    year: String,
    gio: usize,
    pm: bool,
) -> Result<impl View> {
    let state = preview(&day, &month, &year, gio, pm);
    let lit = matches!(state, Preview::Menh { .. });
    let (names, accent) = match &state {
        Preview::Menh { chinh_tinh, accent } => (chinh_tinh.join(" · "), accent.clone()),
        _ => (String::new(), None),
    };
    let caption = match (&state, names.is_empty()) {
        (Preview::Ngu, _) => "Chòm sao mệnh đang ngủ".to_owned(),
        (Preview::NgoaiLich, _) => "Ngày này không có trên lịch".to_owned(),
        (Preview::NgoaiKhoang, _) => "Thiên Hạc an được từ 1800 đến 2199".to_owned(),
        (Preview::Menh { .. }, true) => "Mệnh vô chính diệu · trời rộng mở".to_owned(),
        (Preview::Menh { .. }, false) => format!("Chòm sao {names} đã thức"),
    };

    Ok(view! {
        <div
            class=(class!(
                "relative h-[154px]",
                "lit" if lit,
                "quiet" if names.is_empty(),
            ))
            role="status"
            aria-live="polite"
        >
            if let Some(accent) = &accent {
                <style>
                    ":root { "
                    (accent.as_str())
                    " }"
                </style>
            }
            <svg
                aria-hidden="true"
                viewBox="0 0 320 100"
                class="absolute inset-x-0 top-0 h-[116px] w-full overflow-visible"
            >
                for (x, y) in SCATTER {
                    <circle class="constel-node" cx=(x) cy=(y) r="2.4"></circle>
                }
            </svg>
            if !names.is_empty() {
                <p
                    class="absolute inset-x-0 top-[38px] animate-[reward-in_0.62s_ease-out_both] text-center font-display text-3xl font-semibold text-accent [text-shadow:0_0_18px_var(--accent-glow)]"
                >
                    (names.as_str())
                </p>
            }
            <p
                class="absolute inset-x-0 bottom-1 px-2 text-center text-xs tracking-[0.2em] text-muted uppercase"
            >
                (caption)
            </p>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_hour_sleeps() {
        assert_eq!(preview("1", "1", "2000", 0, false), Preview::Ngu);
        assert_eq!(preview("1", "1", "2000", 13, true), Preview::Ngu);
    }

    #[test]
    fn bad_dates_have_their_own_state() {
        assert_eq!(preview("31", "2", "2000", 3, false), Preview::NgoaiLich);
        assert_eq!(preview("x", "2", "2000", 3, false), Preview::NgoaiLich);
        assert_eq!(preview("1", "1", "2300", 3, false), Preview::NgoaiKhoang);
    }

    #[test]
    fn a_real_date_shows_the_menh_stars() {
        let birth = BirthInput {
            year: 1996,
            month: 4,
            day: 15,
            hour: 22,
            gender: Gender::Nam,
        };
        let chart = build_chart(birth).unwrap();
        assert_eq!(
            preview("15", "04", "1996", 10, true),
            Preview::Menh {
                chinh_tinh: menh_chinh_tinh(&chart).to_vec(),
                accent: accent_style(&chart),
            }
        );
    }
}
