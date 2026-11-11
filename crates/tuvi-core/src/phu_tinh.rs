//! Phụ tinh: every natal star that is not one of the 14 chính tinh. This
//! includes the Lộc Tồn, Bác Sĩ, Thái Tuế and Tràng Sinh rings.
//!
//! Port of `placement/rules/data/phu_tinh.yaml`, the formulas in
//! `positions/absolute.py` and the transforms in `positions/relative.py`.
//! Names come from `components/data/sao.json`.

use crate::can_chi::{DiaChi, ThienCan};
use crate::cuc::Cuc;

use self::PhuTinh as P;
use crate::can_chi::DiaChi as D;

/// Phụ tinh, declared in component-id order. The old engine sorted the stars
/// of a cung by id, so iterating `ALL` gives the same order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PhuTinh {
    AnQuang,
    BacSi,
    BachHo,
    BatToa,
    Benh,
    BenhPhu,
    CoThan,
    DaLa,
    DaiHao,
    DaoHoa,
    DauQuan,
    DeVuong,
    DiaGiai,
    DiaKhong,
    DiaKiep,
    DiaVong,
    DieuKhach,
    Duong,
    DuongPhu,
    GiaiThan,
    HoaCai,
    HoaTinh,
    HongLoan,
    HuuBat,
    HyThan,
    KiepSat,
    KinhDuong,
    LamQuan,
    LinhTinh,
    LnVanTinh,
    LocTon,
    LongDuc,
    LongTri,
    LucSi,
    LuuHa,
    Mo,
    MocDuc,
    NguyetDuc,
    PhaToai,
    PhiLiem,
    PhongCao,
    PhucBinh,
    PhuongCac,
    QuaTu,
    QuanDoi,
    /// Quan Phù, in the Thái Tuế ring (id `quan_phuf`).
    QuanPhuf,
    /// Quan Phủ, in the Bác Sĩ ring (id `quan_phur`).
    QuanPhur,
    QuocAn,
    /// The star Phúc Đức (id `sao_phuc_duc`), not the cung role.
    SaoPhucDuc,
    Suy,
    TaPhu,
    TamThai,
    TangMon,
    TauThu,
    Thai,
    ThaiPhu,
    ThaiTue,
    ThanhLong,
    ThienDieu,
    ThienDuc,
    ThienGiai,
    ThienHi,
    ThienHinh,
    ThienHu,
    ThienKhoc,
    ThienKhoi,
    ThienKhong,
    ThienLa,
    ThienMa,
    ThienPhuc,
    ThienQuan,
    ThienQuy,
    ThienSu,
    ThienTai,
    ThienTho,
    ThienThuong,
    ThienTru,
    ThienViet,
    ThienY,
    ThieuAm,
    ThieuDuong,
    TieuHao,
    TrangSinh,
    TrucPhu,
    Tu,
    TuPhu,
    TuePha,
    TuongQuan,
    Tuyet,
    VanKhuc,
    VanXuong,
}

impl PhuTinh {
    pub const COUNT: usize = 91;

    pub const ALL: [Self; Self::COUNT] = [
        P::AnQuang,
        P::BacSi,
        P::BachHo,
        P::BatToa,
        P::Benh,
        P::BenhPhu,
        P::CoThan,
        P::DaLa,
        P::DaiHao,
        P::DaoHoa,
        P::DauQuan,
        P::DeVuong,
        P::DiaGiai,
        P::DiaKhong,
        P::DiaKiep,
        P::DiaVong,
        P::DieuKhach,
        P::Duong,
        P::DuongPhu,
        P::GiaiThan,
        P::HoaCai,
        P::HoaTinh,
        P::HongLoan,
        P::HuuBat,
        P::HyThan,
        P::KiepSat,
        P::KinhDuong,
        P::LamQuan,
        P::LinhTinh,
        P::LnVanTinh,
        P::LocTon,
        P::LongDuc,
        P::LongTri,
        P::LucSi,
        P::LuuHa,
        P::Mo,
        P::MocDuc,
        P::NguyetDuc,
        P::PhaToai,
        P::PhiLiem,
        P::PhongCao,
        P::PhucBinh,
        P::PhuongCac,
        P::QuaTu,
        P::QuanDoi,
        P::QuanPhuf,
        P::QuanPhur,
        P::QuocAn,
        P::SaoPhucDuc,
        P::Suy,
        P::TaPhu,
        P::TamThai,
        P::TangMon,
        P::TauThu,
        P::Thai,
        P::ThaiPhu,
        P::ThaiTue,
        P::ThanhLong,
        P::ThienDieu,
        P::ThienDuc,
        P::ThienGiai,
        P::ThienHi,
        P::ThienHinh,
        P::ThienHu,
        P::ThienKhoc,
        P::ThienKhoi,
        P::ThienKhong,
        P::ThienLa,
        P::ThienMa,
        P::ThienPhuc,
        P::ThienQuan,
        P::ThienQuy,
        P::ThienSu,
        P::ThienTai,
        P::ThienTho,
        P::ThienThuong,
        P::ThienTru,
        P::ThienViet,
        P::ThienY,
        P::ThieuAm,
        P::ThieuDuong,
        P::TieuHao,
        P::TrangSinh,
        P::TrucPhu,
        P::Tu,
        P::TuPhu,
        P::TuePha,
        P::TuongQuan,
        P::Tuyet,
        P::VanKhuc,
        P::VanXuong,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::AnQuang => "Ân Quang",
            Self::BacSi => "Bác Sĩ",
            Self::BachHo => "Bạch Hổ",
            Self::BatToa => "Bát Toạ",
            Self::Benh => "Bệnh",
            Self::BenhPhu => "Bệnh Phù",
            Self::CoThan => "Cô Thần",
            Self::DaLa => "Đà La",
            Self::DaiHao => "Đại Hao",
            Self::DaoHoa => "Đào Hoa",
            Self::DauQuan => "Đẩu Quân",
            Self::DeVuong => "Đế Vượng",
            Self::DiaGiai => "Địa Giải",
            Self::DiaKhong => "Địa Không",
            Self::DiaKiep => "Địa Kiếp",
            Self::DiaVong => "Địa Võng",
            Self::DieuKhach => "Điếu Khách",
            Self::Duong => "Dưỡng",
            Self::DuongPhu => "Đường Phù",
            Self::GiaiThan => "Giải Thần",
            Self::HoaCai => "Hoa Cái",
            Self::HoaTinh => "Hỏa Tinh",
            Self::HongLoan => "Hồng Loan",
            Self::HuuBat => "Hữu Bật",
            Self::HyThan => "Hỷ Thần",
            Self::KiepSat => "Kiếp Sát",
            Self::KinhDuong => "Kình Dương",
            Self::LamQuan => "Lâm Quan",
            Self::LinhTinh => "Linh Tinh",
            Self::LnVanTinh => "LN Văn Tinh",
            Self::LocTon => "Lộc Tồn",
            Self::LongDuc => "Long Đức",
            Self::LongTri => "Long Trì",
            Self::LucSi => "Lực Sĩ",
            Self::LuuHa => "Lưu Hà",
            Self::Mo => "Mộ",
            Self::MocDuc => "Mộc Dục",
            Self::NguyetDuc => "Nguyệt Đức",
            Self::PhaToai => "Phá Toái",
            Self::PhiLiem => "Phi Liêm",
            Self::PhongCao => "Phong Cáo",
            Self::PhucBinh => "Phục Binh",
            Self::PhuongCac => "Phượng Các",
            Self::QuaTu => "Quả Tú",
            Self::QuanDoi => "Quan Đới",
            Self::QuanPhuf => "Quan Phù",
            Self::QuanPhur => "Quan Phủ",
            Self::QuocAn => "Quốc Ấn",
            Self::SaoPhucDuc => "Phúc Đức",
            Self::Suy => "Suy",
            Self::TaPhu => "Tả Phù",
            Self::TamThai => "Tam Thai",
            Self::TangMon => "Tang Môn",
            Self::TauThu => "Tấu Thư",
            Self::Thai => "Thai",
            Self::ThaiPhu => "Thai Phụ",
            Self::ThaiTue => "Thái Tuế",
            Self::ThanhLong => "Thanh Long",
            Self::ThienDieu => "Thiên Diêu",
            Self::ThienDuc => "Thiên Đức",
            Self::ThienGiai => "Thiên Giải",
            Self::ThienHi => "Thiên Hỉ",
            Self::ThienHinh => "Thiên Hình",
            Self::ThienHu => "Thiên Hư",
            Self::ThienKhoc => "Thiên Khốc",
            Self::ThienKhoi => "Thiên Khôi",
            Self::ThienKhong => "Thiên Không",
            Self::ThienLa => "Thiên La",
            Self::ThienMa => "Thiên Mã",
            Self::ThienPhuc => "Thiên Phúc",
            Self::ThienQuan => "Thiên Quan",
            Self::ThienQuy => "Thiên Quý",
            Self::ThienSu => "Thiên Sứ",
            Self::ThienTai => "Thiên Tài",
            Self::ThienTho => "Thiên Thọ",
            Self::ThienThuong => "Thiên Thương",
            Self::ThienTru => "Thiên Trù",
            Self::ThienViet => "Thiên Việt",
            Self::ThienY => "Thiên Y",
            Self::ThieuAm => "Thiếu Âm",
            Self::ThieuDuong => "Thiếu Dương",
            Self::TieuHao => "Tiểu Hao",
            Self::TrangSinh => "Tràng Sinh",
            Self::TrucPhu => "Trực Phù",
            Self::Tu => "Tử",
            Self::TuPhu => "Tử Phù",
            Self::TuePha => "Tuế Phá",
            Self::TuongQuan => "Tướng Quân",
            Self::Tuyet => "Tuyệt",
            Self::VanKhuc => "Văn Khúc",
            Self::VanXuong => "Văn Xương",
        }
    }
}

/// What the phụ tinh rules read from the chart.
pub struct Input {
    pub nam_can: ThienCan,
    pub nam_chi: DiaChi,
    pub lunar_month: u8,
    pub lunar_day: u8,
    pub hour: DiaChi,
    pub menh: DiaChi,
    pub than: DiaChi,
    pub cuc: Cuc,
    /// +1 clockwise, -1 counter-clockwise (see `cung::van_direction`).
    pub van: i32,
}

// Rings. Member k sits k boxes from the first member.

/// Bác Sĩ ring: starts at Lộc Tồn, goes in the `van` direction.
const BAC_SI_RING: [PhuTinh; 12] = [
    P::BacSi,
    P::LucSi,
    P::ThanhLong,
    P::TieuHao,
    P::TuongQuan,
    P::TauThu,
    P::PhiLiem,
    P::HyThan,
    P::BenhPhu,
    P::DaiHao,
    P::PhucBinh,
    P::QuanPhur,
];

/// Tràng Sinh ring: starts at `TRANG_SINH_START`, goes in the `van` direction.
const TRANG_SINH_RING: [PhuTinh; 12] = [
    P::TrangSinh,
    P::MocDuc,
    P::QuanDoi,
    P::LamQuan,
    P::DeVuong,
    P::Suy,
    P::Benh,
    P::Tu,
    P::Mo,
    P::Tuyet,
    P::Thai,
    P::Duong,
];

/// Thái Tuế ring: starts at the year Địa Chi, goes clockwise. Some boxes hold
/// two stars (`same_position` in the YAML).
const THAI_TUE_RING: [&[PhuTinh]; 12] = [
    &[P::ThaiTue],
    &[P::ThienKhong, P::ThieuDuong],
    &[P::TangMon],
    &[P::ThieuAm],
    &[P::LongTri, P::QuanPhuf],
    &[P::NguyetDuc, P::TuPhu],
    &[P::ThienHu, P::TuePha],
    &[P::LongDuc],
    &[P::BachHo],
    &[P::ThienDuc, P::SaoPhucDuc],
    &[P::DieuKhach],
    &[P::TrucPhu],
];

/// Lộc Tồn offsets (`offset_group`).
const LOC_TON_OFFSETS: [(PhuTinh, i32); 6] = [
    (P::LocTon, 0),
    (P::KinhDuong, 1),
    (P::DaLa, -1),
    (P::LnVanTinh, 3),
    (P::DuongPhu, 5),
    (P::QuocAn, 8),
];

// Tables indexed by Thiên Can (Giáp, Ất, Bính, Đinh, Mậu, Kỷ, Canh, Tân, Nhâm, Quý).

const LOC_TON: [DiaChi; 10] = [
    D::Dan,
    D::Meo,
    D::Ti,
    D::Ngo,
    D::Ti,
    D::Ngo,
    D::Than,
    D::Dau,
    D::Hoi,
    D::Ty,
];

/// `absolute_by_thien_can` rules.
const BY_THIEN_CAN: [(PhuTinh, [DiaChi; 10]); 6] = [
    (
        P::ThienKhoi,
        [
            D::Suu,
            D::Ty,
            D::Hoi,
            D::Hoi,
            D::Suu,
            D::Ty,
            D::Ngo,
            D::Ngo,
            D::Meo,
            D::Meo,
        ],
    ),
    (
        P::ThienViet,
        [
            D::Mui,
            D::Than,
            D::Dau,
            D::Dau,
            D::Mui,
            D::Than,
            D::Dan,
            D::Dan,
            D::Ti,
            D::Ti,
        ],
    ),
    (
        P::LuuHa,
        [
            D::Dau,
            D::Tuat,
            D::Mui,
            D::Thin,
            D::Ti,
            D::Ngo,
            D::Than,
            D::Thin,
            D::Hoi,
            D::Dan,
        ],
    ),
    (
        P::ThienTru,
        [
            D::Ti,
            D::Ngo,
            D::Ty,
            D::Ti,
            D::Ngo,
            D::Than,
            D::Dan,
            D::Ngo,
            D::Dau,
            D::Tuat,
        ],
    ),
    (
        P::ThienQuan,
        [
            D::Mui,
            D::Mui,
            D::Thin,
            D::Dan,
            D::Meo,
            D::Dau,
            D::Hoi,
            D::Dau,
            D::Tuat,
            D::Ngo,
        ],
    ),
    (
        P::ThienPhuc,
        [
            D::Dau,
            D::Dau,
            D::Than,
            D::Hoi,
            D::Meo,
            D::Dan,
            D::Ngo,
            D::Ti,
            D::Ngo,
            D::Ti,
        ],
    ),
];

// Tables indexed by the year Địa Chi (Tý, Sửu, …, Hợi): the
// `absolute_by_dia_chi_groups` rules, with each group expanded.

const BY_NAM_CHI: [(PhuTinh, [DiaChi; 12]); 7] = [
    // Groups: Dần Mão Thìn → Tị, Tị Ngọ Mùi → Thân, Thân Dậu Tuất → Hợi,
    // Hợi Tý Sửu → Mùi.
    (
        P::CoThan,
        [
            D::Mui,
            D::Mui,
            D::Ti,
            D::Ti,
            D::Ti,
            D::Than,
            D::Than,
            D::Than,
            D::Hoi,
            D::Hoi,
            D::Hoi,
            D::Mui,
        ],
    ),
    // Groups: Dần Mão Thìn → Sửu, Tị Ngọ Mùi → Thìn, Thân Dậu Tuất → Dần,
    // Hợi Tý Sửu → Tuất.
    (
        P::QuaTu,
        [
            D::Tuat,
            D::Tuat,
            D::Suu,
            D::Suu,
            D::Suu,
            D::Thin,
            D::Thin,
            D::Thin,
            D::Dan,
            D::Dan,
            D::Dan,
            D::Tuat,
        ],
    ),
    // Tam hợp groups: Thân Tý Thìn, Tị Dậu Sửu, Dần Ngọ Tuất, Hợi Mão Mùi.
    (
        P::ThienMa,
        [
            D::Dan,
            D::Hoi,
            D::Than,
            D::Ti,
            D::Dan,
            D::Hoi,
            D::Than,
            D::Ti,
            D::Dan,
            D::Hoi,
            D::Than,
            D::Ti,
        ],
    ),
    (
        P::HoaCai,
        [
            D::Thin,
            D::Suu,
            D::Tuat,
            D::Mui,
            D::Thin,
            D::Suu,
            D::Tuat,
            D::Mui,
            D::Thin,
            D::Suu,
            D::Tuat,
            D::Mui,
        ],
    ),
    (
        P::DaoHoa,
        [
            D::Dau,
            D::Ngo,
            D::Meo,
            D::Ty,
            D::Dau,
            D::Ngo,
            D::Meo,
            D::Ty,
            D::Dau,
            D::Ngo,
            D::Meo,
            D::Ty,
        ],
    ),
    (
        P::KiepSat,
        [
            D::Ti,
            D::Dan,
            D::Hoi,
            D::Than,
            D::Ti,
            D::Dan,
            D::Hoi,
            D::Than,
            D::Ti,
            D::Dan,
            D::Hoi,
            D::Than,
        ],
    ),
    // Groups: Tý Ngọ Mão Dậu → Tị, Thìn Tuất Sửu Mùi → Sửu,
    // Dần Thân Tị Hợi → Dậu.
    (
        P::PhaToai,
        [
            D::Ti,
            D::Suu,
            D::Dau,
            D::Ti,
            D::Suu,
            D::Dau,
            D::Ti,
            D::Suu,
            D::Dau,
            D::Ti,
            D::Suu,
            D::Dau,
        ],
    ),
];

/// Start of Hỏa Tinh, by tam hợp group of the year Địa Chi (index `% 4`:
/// Thân Tý Thìn, Tị Dậu Sửu, Dần Ngọ Tuất, Hợi Mão Mùi).
const HOA_TINH_START: [DiaChi; 4] = [D::Dan, D::Meo, D::Suu, D::Dau];
/// Start of Linh Tinh, same groups as `HOA_TINH_START`.
const LINH_TINH_START: [DiaChi; 4] = [D::Tuat, D::Tuat, D::Meo, D::Tuat];

/// `trang_sinh_position`: by the ngũ hành of the Cục. Thổ uses Thân, as Thủy.
const fn trang_sinh_start(cuc: Cuc) -> DiaChi {
    match cuc {
        Cuc::ThuyNhi | Cuc::ThoNgu => D::Than,
        Cuc::MocTam => D::Hoi,
        Cuc::KimTu => D::Ti,
        Cuc::HoaLuc => D::Dan,
    }
}

/// Positions of all phụ tinh, indexed by `PhuTinh`.
pub fn positions(input: &Input) -> [DiaChi; PhuTinh::COUNT] {
    let Input {
        nam_can,
        nam_chi,
        lunar_month,
        lunar_day,
        hour,
        menh,
        than,
        cuc,
        van,
    } = *input;
    let can = nam_can.index();
    let chi = nam_chi.index();
    let year_steps = chi as i32;
    let month_steps = i32::from(lunar_month) - 1;
    let day = i32::from(lunar_day);
    let hour_steps = hour.index() as i32;

    let mut p = [D::Ty; PhuTinh::COUNT];

    // Rings and the Lộc Tồn group.
    let loc_ton = LOC_TON[can];
    let trang_sinh = trang_sinh_start(cuc);
    for (k, sao) in (0_i32..).zip(BAC_SI_RING) {
        p[sao as usize] = loc_ton.add(van * k);
    }
    for (k, sao) in (0_i32..).zip(TRANG_SINH_RING) {
        p[sao as usize] = trang_sinh.add(van * k);
    }
    for (k, saos) in (0_i32..).zip(THAI_TUE_RING) {
        for &sao in saos {
            p[sao as usize] = nam_chi.add(k);
        }
    }
    for (sao, offset) in LOC_TON_OFFSETS {
        p[sao as usize] = loc_ton.add(offset);
    }

    // Lookup tables.
    for (sao, table) in BY_THIEN_CAN {
        p[sao as usize] = table[can];
    }
    for (sao, table) in BY_NAM_CHI {
        p[sao as usize] = table[chi];
    }

    // By year Địa Chi.
    p[P::ThienHi as usize] = D::Dau.add(-year_steps);
    p[P::HongLoan as usize] = D::Dau.add(6 - year_steps);
    p[P::GiaiThan as usize] = D::Tuat.add(-year_steps);
    p[P::PhuongCac as usize] = D::Tuat.add(-year_steps);
    p[P::ThienKhoc as usize] = D::Ngo.add(-year_steps);
    p[P::ThienTai as usize] = menh.add(year_steps);
    p[P::ThienTho as usize] = than.add(year_steps);

    // By lunar month.
    let ta_phu = D::Thin.add(month_steps);
    let huu_bat = D::Tuat.add(-month_steps);
    p[P::TaPhu as usize] = ta_phu;
    p[P::HuuBat as usize] = huu_bat;
    p[P::ThienGiai as usize] = D::Than.add(month_steps);
    p[P::DiaGiai as usize] = D::Mui.add(month_steps);
    p[P::ThienHinh as usize] = D::Dau.add(month_steps);
    p[P::ThienDieu as usize] = D::Suu.add(month_steps);
    p[P::ThienY as usize] = D::Suu.add(month_steps);

    // By birth hour.
    let van_xuong = D::Tuat.add(-hour_steps);
    let van_khuc = D::Thin.add(hour_steps);
    p[P::VanXuong as usize] = van_xuong;
    p[P::VanKhuc as usize] = van_khuc;
    p[P::DiaKhong as usize] = D::Hoi.add(-hour_steps);
    p[P::DiaKiep as usize] = D::Hoi.add(hour_steps);
    p[P::ThaiPhu as usize] = D::Ngo.add(hour_steps);
    p[P::PhongCao as usize] = D::Dan.add(hour_steps);
    p[P::HoaTinh as usize] = HOA_TINH_START[chi % 4].add(van * hour_steps);
    p[P::LinhTinh as usize] = LINH_TINH_START[chi % 4].add(-van * hour_steps);

    // By lunar day, from stars placed above.
    p[P::TamThai as usize] = ta_phu.add(day - 1);
    p[P::BatToa as usize] = huu_bat.add(-(day - 1));
    p[P::AnQuang as usize] = van_xuong.add(day - 2);
    p[P::ThienQuy as usize] = van_khuc.add(-(day - 2));

    // Đẩu Quân: from Thái Tuế, back by month, then forward by hour.
    p[P::DauQuan as usize] = nam_chi.add(hour_steps - month_steps);

    // Fixed by cung role or by box.
    p[P::ThienSu as usize] = menh.add(7); // Tật Ách
    p[P::ThienThuong as usize] = menh.add(5); // Nô Bộc
    p[P::ThienLa as usize] = D::Thin;
    p[P::DiaVong as usize] = D::Tuat;

    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_is_in_declaration_order() {
        for (i, sao) in PhuTinh::ALL.iter().enumerate() {
            assert_eq!(*sao as usize, i);
        }
    }

    #[test]
    fn rings_have_no_duplicates() {
        let mut seen = [false; PhuTinh::COUNT];
        let ring_members = BAC_SI_RING
            .iter()
            .chain(&TRANG_SINH_RING)
            .chain(THAI_TUE_RING.iter().flat_map(|saos| saos.iter()));
        for &sao in ring_members {
            assert!(!seen[sao as usize], "{sao:?} is in two rings");
            seen[sao as usize] = true;
        }
    }
}
