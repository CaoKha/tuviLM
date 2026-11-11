//! Cục (port of `context/natal.py::_get_cuc`). Names come from
//! `components/data/cuc.json`.

use crate::elementary::{DiaChi, ThienCan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Cuc {
    ThuyNhi,
    MocTam,
    KimTu,
    ThoNgu,
    HoaLuc,
}

impl Cuc {
    pub const ALL: [Self; 5] = [
        Self::ThuyNhi,
        Self::MocTam,
        Self::KimTu,
        Self::ThoNgu,
        Self::HoaLuc,
    ];

    pub const fn number(self) -> i32 {
        self as i32 + 2
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::ThuyNhi => "Thủy Nhị cục",
            Self::MocTam => "Mộc Tam cục",
            Self::KimTu => "Kim Tứ cục",
            Self::ThoNgu => "Thổ Ngũ cục",
            Self::HoaLuc => "Hỏa Lục cục",
        }
    }

    /// Cục from the year Thiên Can and the Mệnh position.
    pub const fn from_menh(year_can: ThienCan, menh: DiaChi) -> Self {
        // Index into `ALL`, per Thiên Can pair (Giáp/Kỷ, Ất/Canh, ...) and Mệnh group.
        const ORDERS: [[u8; 5]; 5] = [
            [0, 4, 1, 3, 2], // Giáp, Kỷ
            [4, 3, 2, 1, 0], // Ất, Canh
            [3, 1, 0, 2, 4], // Bính, Tân
            [1, 2, 4, 0, 3], // Đinh, Nhâm
            [2, 0, 3, 4, 1], // Mậu, Quý
        ];
        let group = match menh {
            DiaChi::Ty | DiaChi::Suu => 0,
            DiaChi::Dan | DiaChi::Meo | DiaChi::Tuat | DiaChi::Hoi => 1,
            DiaChi::Thin | DiaChi::Ti => 2,
            DiaChi::Ngo | DiaChi::Mui => 3,
            DiaChi::Than | DiaChi::Dau => 4,
        };
        Self::ALL[ORDERS[year_can.index() % 5][group] as usize]
    }
}
