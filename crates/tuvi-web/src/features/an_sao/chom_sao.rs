//! Chòm sao: the constellation reward of the An sao preview.
//!
//! Each chính tinh has a faint ghost portrait (a static SVG, loaded with
//! `<img>` only when its star is in cung Mệnh) and a star pattern traced over
//! it. Song tinh show both figures side by side. The animation runs in CSS
//! (`.chom-*` in `styles.css`); this module computes each line's start delay
//! and each star's pulse phase, as `constellation-wave.ts` of the old app did.

use topcoat::{
    Result,
    asset::Asset,
    context::Cx,
    view::{View, view},
};

mod data;

/// One chính tinh's figure and star pattern.
pub struct ChomSao {
    /// The old app's star key, for element ids and the pulse order.
    key: &'static str,
    /// The chính tinh name, as `tuvi_core` writes it.
    chinh_tinh: &'static str,
    /// The god of the portrait, for the screen reader label.
    deity: &'static str,
    ghost: Asset,
    /// `(x, y, power)` in the portrait's 100×100 box.
    points: &'static [(f32, f32, f32)],
    /// Pairs of point indexes.
    lines: &'static [(usize, usize)],
}

/// The figure of a chính tinh, or `None` for an unknown name.
pub fn chom_sao_of(chinh_tinh: &str) -> Option<&'static ChomSao> {
    data::ALL.iter().find(|chom| chom.chinh_tinh == chinh_tinh)
}

/// When the wave of lines starts in each 4.2 s cycle, in ms.
const WAVE_START_MS: f64 = 150.0;
/// How long the wave takes to reach the last star, in ms.
const WAVE_DURATION_MS: f64 = 2100.0;
/// One star pulse cycle, in ms. Keep in step with `.chom-halo` in styles.css.
const STAR_PULSE_MS: f64 = 4500.0;

/// Each line as `(from, to, delay in ms)`: lines draw from the star the wave
/// reaches first. The wave visits connected groups in point order, with a
/// short beat between groups.
fn wave(chom: &ChomSao) -> Vec<(usize, usize, f64)> {
    let count = chom.points.len();
    let mut neighbors = vec![Vec::new(); count];
    for &(from, to) in chom.lines {
        neighbors[from].push(to);
        neighbors[to].push(from);
    }
    let mut slots = vec![None; count];
    let mut slot = 0;
    for start in 0..count {
        if slots[start].is_some() {
            continue;
        }
        let mut group = Vec::new();
        let mut stack = vec![start];
        while let Some(point) = stack.pop() {
            if !group.contains(&point) {
                group.push(point);
                stack.extend(&neighbors[point]);
            }
        }
        group.sort_unstable();
        slot += usize::from(start > 0);
        for point in group {
            slots[point] = Some(slot);
            slot += 1;
        }
    }
    let step = WAVE_DURATION_MS / slot.saturating_sub(1).max(1) as f64;
    let delay = |point: usize| WAVE_START_MS + slots[point].unwrap_or(0) as f64 * step;
    chom.lines
        .iter()
        .map(|&(a, b)| {
            let (from, to) = if delay(a) <= delay(b) { (a, b) } else { (b, a) };
            (from, to, delay(from))
        })
        .collect()
}

/// Each star's pulse phase in ms: spread evenly over one cycle, in an order
/// from an FNV-1a hash of `"{key}-{index}"`, so it looks random but stays.
fn pulse_phases(chom: &ChomSao) -> Vec<f64> {
    let count = chom.points.len();
    let hash = |index: usize| {
        format!("{}-{index}", chom.key)
            .bytes()
            .fold(2_166_136_261u32, |hash, byte| {
                (hash ^ u32::from(byte)).wrapping_mul(16_777_619)
            })
    };
    let mut order: Vec<usize> = (0..count).collect();
    order.sort_by_key(|&index| (hash(index), index));
    let mut phases = vec![0.0; count];
    for (slot, index) in order.into_iter().enumerate() {
        phases[index] = slot as f64 * STAR_PULSE_MS / count as f64;
    }
    phases
}

/// One figure: the ghost portrait under its star pattern. `song_tinh`
/// makes it smaller, to fit two side by side.
#[topcoat::view::component]
pub async fn chom_figure(
    cx: &Cx,
    chom: &'static ChomSao,
    song_tinh: bool,
    delay_ms: usize,
) -> Result<impl View> {
    let halo = format!("halo-{}", chom.key);
    let phases = pulse_phases(chom);
    Ok(view! {
        <div
            class=(if song_tinh {
                "chom-figure w-[min(150px,44vw)] md:w-[min(170px,44vw)]"
            } else {
                "chom-figure w-[170px] md:w-[min(250px,78vw)]"
            })
            style=(format!("animation-delay: {delay_ms}ms"))
        >
            <img
                src=(chom.ghost)
                alt=""
                decoding="async"
                class="absolute inset-0 size-full scale-[1.025] object-contain opacity-24"
            >
            <svg
                role="img"
                aria-label=(format!(
                    "Chòm sao {} trong hình tượng {}",
                    chom.chinh_tinh,
                    chom.deity,
                ))
                viewBox="0 0 100 100"
                class="absolute inset-0 size-full scale-[1.025] overflow-visible"
            >
                <defs>
                    <radialGradient id=(halo.as_str())>
                        <stop
                            offset="0%"
                            class="[stop-color:var(--color-ink)] [stop-opacity:0.92]"
                        ></stop>
                        <stop
                            offset="38%"
                            class="[stop-color:var(--accent-2)] [stop-opacity:0.72]"
                        ></stop>
                        <stop
                            offset="100%"
                            class="[stop-color:var(--accent)] [stop-opacity:0]"
                        ></stop>
                    </radialGradient>
                </defs>
                for (from, to, delay) in wave(chom) {
                    <line
                        class="chom-line"
                        x1=(chom.points[from].0)
                        y1=(chom.points[from].1)
                        x2=(chom.points[to].0)
                        y2=(chom.points[to].1)
                        pathLength="1"
                        style=(format!("animation-delay: {delay:.0}ms"))
                    ></line>
                }
                for (&(x, y, power), phase) in chom.points.iter().zip(phases) {
                    <circle
                        class="chom-halo"
                        cx=(x)
                        cy=(y)
                        r=(format!("{:.2}", 1.8 * power))
                        style=(format!(
                            "fill: url(#{halo}); animation-delay: -{phase:.0}ms",
                        ))
                    ></circle>
                    <circle
                        class="chom-core"
                        cx=(x)
                        cy=(y)
                        r=(format!("{:.2}", 0.68 * power))
                        style=(format!("animation-delay: -{phase:.0}ms"))
                    ></circle>
                }
            </svg>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_chinh_tinh_has_a_valid_figure() {
        let names = [
            "Tử Vi",
            "Thiên Cơ",
            "Thái Dương",
            "Vũ Khúc",
            "Thiên Đồng",
            "Liêm Trinh",
            "Thiên Phủ",
            "Thái Âm",
            "Tham Lang",
            "Cự Môn",
            "Thiên Tướng",
            "Thiên Lương",
            "Thất Sát",
            "Phá Quân",
        ];
        for name in names {
            let chom = chom_sao_of(name).unwrap();
            let count = chom.points.len();
            assert!(
                chom.lines.iter().all(|&(a, b)| a < count && b < count),
                "{name}"
            );
        }
        assert!(chom_sao_of("Lộc Tồn").is_none());
    }

    #[test]
    fn the_wave_crosses_groups_with_a_beat() {
        // Tử Vi: crown (stars 0–3), lightning (4–6), robe (7–8). With a beat
        // between groups there are 11 slots, so each step is 210 ms.
        let lines = wave(chom_sao_of("Tử Vi").unwrap());
        assert_eq!(lines[0], (0, 1, WAVE_START_MS));
        assert_eq!(lines[4], (4, 5, WAVE_START_MS + 5.0 * 210.0));
        assert_eq!(lines[6], (7, 8, WAVE_START_MS + 9.0 * 210.0));
    }

    #[test]
    fn pulse_phases_spread_over_one_cycle() {
        let chom = chom_sao_of("Thiên Lương").unwrap();
        let mut phases = pulse_phases(chom);
        phases.sort_by(f64::total_cmp);
        let step = STAR_PULSE_MS / chom.points.len() as f64;
        for (slot, phase) in phases.into_iter().enumerate() {
            assert!((phase - slot as f64 * step).abs() < 1e-9);
        }
    }
}
