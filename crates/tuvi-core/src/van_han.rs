//! Đại hạn and Tiểu hạn of the natal chart (port of
//! `model/period_focus.py::build_dai_han_focus_map` and
//! `build_tieu_han_focus_map`).
//!
//! Only the natal data that a chart shows: which ages and which years fall in
//! each cung. The stars that move per period (`assembly/period.py`) are not
//! ported.

use crate::can_chi::DiaChi;

/// Start age (tuổi mụ) of the 10-year Đại hạn in each cung, indexed by
/// `DiaChi`. The first Đại hạn starts at the Cục number in Mệnh. The next ones
/// go 1 cung at a time in the `van` direction. The end age is start + 9.
pub fn dai_han_start(cuc_number: i32, menh: DiaChi, van: i32) -> [u8; 12] {
    let mut start = [0; 12];
    for k in 0..12_i32 {
        // At most 6 + 110 = 116, so the cast does not truncate.
        start[menh.add(van * k).index()] = (cuc_number + 10 * k) as u8;
    }
    start
}

/// Tiểu hạn: for each cung (indexed by `DiaChi`), the Địa Chi of the years
/// whose Tiểu hạn falls in that cung. The birth year Địa Chi starts in a cung
/// set by its tam hợp group; the next years go 1 cung at a time in the `van`
/// direction.
pub fn tieu_han(nam_chi: DiaChi, van: i32) -> [DiaChi; 12] {
    // By `nam_chi % 4`: Thân Tý Thìn → Tuất, Tị Dậu Sửu → Mùi,
    // Dần Ngọ Tuất → Thìn, Hợi Mão Mùi → Sửu.
    const START: [DiaChi; 4] = [DiaChi::Tuat, DiaChi::Mui, DiaChi::Thin, DiaChi::Suu];
    let start = START[nam_chi.index() % 4];
    let mut year = [nam_chi; 12];
    for k in 0..12_i32 {
        year[start.add(van * k).index()] = nam_chi.add(k);
    }
    year
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dai_han_covers_every_cung() {
        let start = dai_han_start(6, DiaChi::Ngo, -1);
        assert_eq!(start[DiaChi::Ngo.index()], 6);
        assert_eq!(start[DiaChi::Ti.index()], 16);
        assert_eq!(start[DiaChi::Mui.index()], 116);
    }

    #[test]
    fn tieu_han_example() {
        // Mão year, counter-clockwise: Mão in Sửu, Thìn in Tý.
        let year = tieu_han(DiaChi::Meo, -1);
        assert_eq!(year[DiaChi::Suu.index()], DiaChi::Meo);
        assert_eq!(year[DiaChi::Ty.index()], DiaChi::Thin);
    }
}
