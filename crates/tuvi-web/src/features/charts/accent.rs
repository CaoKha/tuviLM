//! The chart's accent color: the ngũ hành (element) of the chính tinh in cung
//! Mệnh. One star tints with its element's light shade; two stars pair their
//! two elements. Vô chính diệu keeps the neutral accent.
//!
//! The colors are CSS variables in `styles.css` (`--element-*`). This module
//! only picks which variables to use.

use tuvi_core::Chart;

/// The role name of cung Mệnh, as `tuvi_core` writes it.
const MENH: &str = "Mệnh";

/// Ngũ hành of a chính tinh, as the CSS variable suffix.
pub(super) fn element(chinh_tinh: &str) -> Option<&'static str> {
    Some(match chinh_tinh {
        "Tử Vi" | "Thiên Phủ" => "tho",
        "Thất Sát" | "Vũ Khúc" => "kim",
        "Tham Lang" | "Thiên Cơ" | "Thiên Lương" => "moc",
        "Thái Dương" | "Liêm Trinh" => "hoa",
        "Phá Quân" | "Thái Âm" | "Thiên Tướng" | "Thiên Đồng" | "Cự Môn" => "thuy",
        _ => return None,
    })
}

/// An inline `style` value that sets `--accent`, `--accent-2` and
/// `--accent-glow`, or `None` to keep the neutral default.
pub fn accent_style(chart: &Chart) -> Option<String> {
    let menh = chart.cungs.iter().find(|cung| cung.role == MENH)?;
    let mut elements = menh.chinh_tinh.iter().map(|sao| element(sao));
    let first = elements.next()??;
    let accent_2 = match elements.next() {
        None => format!("var(--element-{first}-light)"),
        Some(second) => format!("var(--element-{})", second?),
    };
    Some(format!(
        "--accent: var(--element-{first}); --accent-2: {accent_2}; --accent-glow: var(--element-{first}-glow)"
    ))
}
