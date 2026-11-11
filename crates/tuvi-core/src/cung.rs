//! Cung structure: Mệnh/Thân positions, the 12 roles and the Thiên Can of
//! each cung.
//!
//! Ports `context/natal.py::_get_menh_position`,
//! `placement/rules/data/cung_roles.yaml` and `model/cung.py`.

use crate::elementary::{DiaChi, ThienCan};

/// The 12 cung roles, in the clockwise order of `cung_roles.yaml`
/// (Mệnh first). Names come from `components/data/cung_role.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CungRole {
    Menh,
    PhuMau,
    PhucDuc,
    DienTrach,
    QuanLoc,
    NoBoc,
    ThienDi,
    TatAch,
    TaiBach,
    TuTuc,
    PhuThe,
    HuynhDe,
}

impl CungRole {
    pub const ALL: [Self; 12] = [
        Self::Menh,
        Self::PhuMau,
        Self::PhucDuc,
        Self::DienTrach,
        Self::QuanLoc,
        Self::NoBoc,
        Self::ThienDi,
        Self::TatAch,
        Self::TaiBach,
        Self::TuTuc,
        Self::PhuThe,
        Self::HuynhDe,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Menh => "Mệnh",
            Self::PhuMau => "Phụ Mẫu",
            Self::PhucDuc => "Phúc Đức",
            Self::DienTrach => "Điền Trạch",
            Self::QuanLoc => "Quan Lộc",
            Self::NoBoc => "Nô Bộc",
            Self::ThienDi => "Thiên Di",
            Self::TatAch => "Tật Ách",
            Self::TaiBach => "Tài Bạch",
            Self::TuTuc => "Tử Tức",
            Self::PhuThe => "Phu Thê",
            Self::HuynhDe => "Huynh Đệ",
        }
    }

    /// Role of the cung at `dia_chi`: roles go clockwise from Mệnh.
    pub const fn at(dia_chi: DiaChi, menh: DiaChi) -> Self {
        Self::ALL[(dia_chi.index() + 12 - menh.index()) % 12]
    }
}

/// Mệnh: start at Dần for month 1, go clockwise by month, then counter-clockwise
/// by birth hour.
pub const fn menh_position(lunar_month: u8, hour: DiaChi) -> DiaChi {
    DiaChi::Dan.add(lunar_month as i32 - 1 - hour.index() as i32)
}

/// Thân: from Mệnh, clockwise by 2 × birth hour (`move_by_birth_hour`,
/// `step_multiplier: 2`).
pub const fn than_position(menh: DiaChi, hour: DiaChi) -> DiaChi {
    menh.add(2 * hour.index() as i32)
}

/// Ngũ Hổ Độn: Thiên Can of the Dần cung for a year Thiên Can, indexed by
/// `ThienCan`.
const NGU_HO_DON: [ThienCan; 10] = [
    ThienCan::Binh, // Giáp
    ThienCan::Mau,  // Ất
    ThienCan::Canh, // Bính
    ThienCan::Nham, // Đinh
    ThienCan::Giap, // Mậu
    ThienCan::Binh, // Kỷ
    ThienCan::Mau,  // Canh
    ThienCan::Canh, // Tân
    ThienCan::Nham, // Nhâm
    ThienCan::Giap, // Quý
];

/// Thiên Can of a cung: advance from the Dần Thiên Can by the clockwise
/// distance (0..12) from Dần.
pub const fn cung_thien_can(year_can: ThienCan, cung: DiaChi) -> ThienCan {
    let distance_from_dan = (cung.index() + 12 - DiaChi::Dan.index()) % 12;
    NGU_HO_DON[year_can.index()].add(distance_from_dan as i32)
}
