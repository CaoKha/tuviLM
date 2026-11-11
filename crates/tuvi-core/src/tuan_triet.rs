//! Tuần and Triệt (port of `placement/rules/data/tuan_triet.yaml` and
//! `positions/absolute.py::tuan_positions_fn`, `triet_positions_fn`).
//!
//! Each one covers 2 neighbouring cung.

use crate::can_chi::{DiaChi, ThienCan};

/// Triệt, by year Thiên Can: Giáp/Kỷ → Thân Dậu, Ất/Canh → Ngọ Mùi,
/// Bính/Tân → Thìn Tị, Đinh/Nhâm → Dần Mão, Mậu/Quý → Tý Sửu.
pub const fn triet(nam_can: ThienCan) -> [DiaChi; 2] {
    const FIRST: [DiaChi; 5] = [
        DiaChi::Than,
        DiaChi::Ngo,
        DiaChi::Thin,
        DiaChi::Dan,
        DiaChi::Ty,
    ];
    let first = FIRST[nam_can.index() % 5];
    [first, first.add(1)]
}

/// Tuần: the 2 Địa Chi that the decade (tuần giáp) of the birth year leaves
/// out. The decade starts at `nam_chi - nam_can`, for example Giáp Tý → Tuất Hợi.
pub const fn tuan(nam_can: ThienCan, nam_chi: DiaChi) -> [DiaChi; 2] {
    let giap = nam_chi.add(-(nam_can.index() as i32));
    [giap.add(-2), giap.add(-1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuan_examples() {
        // Giáp Tý decade → Tuất Hợi; Giáp Ngọ decade (e.g. Canh Tý) → Thìn Tị.
        assert_eq!(
            tuan(ThienCan::Giap, DiaChi::Ty),
            [DiaChi::Tuat, DiaChi::Hoi]
        );
        assert_eq!(tuan(ThienCan::Canh, DiaChi::Ty), [DiaChi::Thin, DiaChi::Ti]);
        assert_eq!(triet(ThienCan::Quy), [DiaChi::Ty, DiaChi::Suu]);
    }
}
