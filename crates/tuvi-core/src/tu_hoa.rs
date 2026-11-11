//! Tứ Hóa of the birth year (port of `placement/rules/data/tu_hoa.yaml`).
//!
//! Names come from `components/data/tuhoa.json`.

use crate::can_chi::ThienCan;
use crate::chinh_tinh::Sao;
use crate::phu_tinh::PhuTinh;

/// The 4 Hóa, declared in component-id order (`hoa_khoa`, `hoa_ky`,
/// `hoa_loc`, `hoa_quyen`). The old engine sorted the stars of a cung by id,
/// so iterating `ALL` gives the same order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Hoa {
    Khoa,
    Ky,
    Loc,
    Quyen,
}

impl Hoa {
    pub const ALL: [Self; 4] = [Self::Khoa, Self::Ky, Self::Loc, Self::Quyen];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Khoa => "Hóa Khoa",
            Self::Ky => "Hóa Kỵ",
            Self::Loc => "Hóa Lộc",
            Self::Quyen => "Hóa Quyền",
        }
    }
}

/// The star that takes a Hóa: a chính tinh or a phụ tinh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Chinh(Sao),
    Phu(PhuTinh),
}

use Target::{Chinh, Phu};

/// Targets by year Thiên Can, in the order Lộc, Quyền, Khoa, Kỵ (as in the YAML).
const TABLE: [[Target; 4]; 10] = [
    // Giáp
    [
        Chinh(Sao::LiemTrinh),
        Chinh(Sao::PhaQuan),
        Chinh(Sao::VuKhuc),
        Chinh(Sao::ThaiDuong),
    ],
    // Ất
    [
        Chinh(Sao::ThienCo),
        Chinh(Sao::ThienLuong),
        Chinh(Sao::TuVi),
        Chinh(Sao::ThaiAm),
    ],
    // Bính
    [
        Chinh(Sao::ThienDong),
        Chinh(Sao::ThienCo),
        Phu(PhuTinh::VanXuong),
        Chinh(Sao::LiemTrinh),
    ],
    // Đinh
    [
        Chinh(Sao::ThaiAm),
        Chinh(Sao::ThienDong),
        Chinh(Sao::ThienCo),
        Chinh(Sao::CuMon),
    ],
    // Mậu
    [
        Chinh(Sao::ThamLang),
        Chinh(Sao::ThaiAm),
        Phu(PhuTinh::HuuBat),
        Chinh(Sao::ThienCo),
    ],
    // Kỷ
    [
        Chinh(Sao::VuKhuc),
        Chinh(Sao::ThamLang),
        Chinh(Sao::ThienLuong),
        Phu(PhuTinh::VanKhuc),
    ],
    // Canh
    [
        Chinh(Sao::ThaiDuong),
        Chinh(Sao::VuKhuc),
        Chinh(Sao::ThienDong),
        Chinh(Sao::ThaiAm),
    ],
    // Tân
    [
        Chinh(Sao::CuMon),
        Chinh(Sao::ThaiDuong),
        Phu(PhuTinh::VanKhuc),
        Phu(PhuTinh::VanXuong),
    ],
    // Nhâm
    [
        Chinh(Sao::ThienLuong),
        Chinh(Sao::TuVi),
        Chinh(Sao::ThienPhu),
        Chinh(Sao::VuKhuc),
    ],
    // Quý
    [
        Chinh(Sao::PhaQuan),
        Chinh(Sao::CuMon),
        Chinh(Sao::ThaiAm),
        Chinh(Sao::ThamLang),
    ],
];

/// The star that takes `hoa` in a year with Thiên Can `nam_can`.
pub const fn target(nam_can: ThienCan, hoa: Hoa) -> Target {
    let [loc, quyen, khoa, ky] = TABLE[nam_can.index()];
    match hoa {
        Hoa::Loc => loc,
        Hoa::Quyen => quyen,
        Hoa::Khoa => khoa,
        Hoa::Ky => ky,
    }
}
