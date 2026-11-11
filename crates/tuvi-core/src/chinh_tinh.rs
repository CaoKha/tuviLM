//! The 14 chính tinh (port of `placement/rules/data/chinh_tinh.yaml`,
//! `placement/transforms.py` and `positions/absolute.py::tuvi_position_fn`).
//!
//! Names come from `components/data/sao.json`.

use crate::elementary::DiaChi;

/// Chính tinh, declared in component-id order. Python sorts the stars of a
/// cung by id, so iterating `ALL` gives the same order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Sao {
    CuMon,
    LiemTrinh,
    PhaQuan,
    ThaiAm,
    ThaiDuong,
    ThamLang,
    ThatSat,
    ThienCo,
    ThienDong,
    ThienLuong,
    ThienPhu,
    ThienTuong,
    TuVi,
    VuKhuc,
}

impl Sao {
    pub const ALL: [Self; 14] = [
        Self::CuMon,
        Self::LiemTrinh,
        Self::PhaQuan,
        Self::ThaiAm,
        Self::ThaiDuong,
        Self::ThamLang,
        Self::ThatSat,
        Self::ThienCo,
        Self::ThienDong,
        Self::ThienLuong,
        Self::ThienPhu,
        Self::ThienTuong,
        Self::TuVi,
        Self::VuKhuc,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::CuMon => "Cự Môn",
            Self::LiemTrinh => "Liêm Trinh",
            Self::PhaQuan => "Phá Quân",
            Self::ThaiAm => "Thái Âm",
            Self::ThaiDuong => "Thái Dương",
            Self::ThamLang => "Tham Lang",
            Self::ThatSat => "Thất Sát",
            Self::ThienCo => "Thiên Cơ",
            Self::ThienDong => "Thiên Đồng",
            Self::ThienLuong => "Thiên Lương",
            Self::ThienPhu => "Thiên Phủ",
            Self::ThienTuong => "Thiên Tướng",
            Self::TuVi => "Tử Vi",
            Self::VuKhuc => "Vũ Khúc",
        }
    }
}

// Transforms from `placement/transforms.py`.

const fn xung_chieu(position: DiaChi) -> DiaChi {
    position.add(6)
}

/// `tam_hop`: 4 boxes, `cw = true` for clockwise.
const fn tam_hop(position: DiaChi, cw: bool) -> DiaChi {
    position.add(if cw { 4 } else { -4 })
}

const fn nhi_hop(position: DiaChi) -> DiaChi {
    DiaChi::from_index(13 - position.index() as i32)
}

const fn luc_hai(position: DiaChi) -> DiaChi {
    DiaChi::from_index(7 - position.index() as i32)
}

/// `mirror_across` with axis (Dần, Thân).
const fn mirror_dan_than(position: DiaChi) -> DiaChi {
    DiaChi::from_index(2 * DiaChi::Dan.index() as i32 - position.index() as i32)
}

/// An Tử Vi: start at Dần, from lunar day and cục number.
pub const fn tu_vi_position(lunar_day: u8, cuc_number: i32) -> DiaChi {
    let date = lunar_day as i32;
    let rem = date % cuc_number;
    let (div, borrow) = if rem == 0 {
        (date / cuc_number, 0)
    } else {
        (date / cuc_number + 1, cuc_number - rem)
    };
    let offset = if borrow % 2 == 0 {
        div - 1 + borrow
    } else {
        div - 1 - borrow
    };
    DiaChi::Dan.add(offset)
}

/// Positions of all 14 chính tinh, indexed by `Sao`.
pub const fn chinh_tinh_positions(tu_vi: DiaChi) -> [DiaChi; 14] {
    let thien_phu = mirror_dan_than(tu_vi);
    let vu_khuc = tam_hop(tu_vi, false);
    let liem_trinh = tam_hop(tu_vi, true);
    let that_sat = xung_chieu(thien_phu);
    let pha_quan = tam_hop(that_sat, true);
    let tham_lang = tam_hop(that_sat, false);

    let mut positions = [tu_vi; 14];
    positions[Sao::CuMon as usize] = luc_hai(tu_vi);
    positions[Sao::LiemTrinh as usize] = liem_trinh;
    positions[Sao::PhaQuan as usize] = pha_quan;
    positions[Sao::ThaiAm as usize] = nhi_hop(vu_khuc);
    positions[Sao::ThaiDuong as usize] = nhi_hop(thien_phu);
    positions[Sao::ThamLang as usize] = tham_lang;
    positions[Sao::ThatSat as usize] = that_sat;
    positions[Sao::ThienCo as usize] = nhi_hop(pha_quan);
    positions[Sao::ThienDong as usize] = nhi_hop(tham_lang);
    positions[Sao::ThienLuong as usize] = nhi_hop(liem_trinh);
    positions[Sao::ThienPhu as usize] = thien_phu;
    positions[Sao::ThienTuong as usize] = xung_chieu(pha_quan);
    positions[Sao::TuVi as usize] = tu_vi;
    positions[Sao::VuKhuc as usize] = vu_khuc;
    positions
}
