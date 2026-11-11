//! The public chart types and [`build_chart`].

use serde::{Deserialize, Serialize};

use crate::ban_menh;
use crate::calendar::{self, LunarBirth};
use crate::can_chi::{DiaChi, ThienCan};
use crate::chinh_tinh::{self, Sao};
use crate::cuc::Cuc;
use crate::cung::{self, CungRole};
use crate::phu_tinh::{self, PhuTinh};
use crate::tu_hoa::{self, Hoa};
use crate::tuan_triet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Nam,
    Nu,
}

/// Solar (Gregorian) birth moment, local Vietnam time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthInput {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    /// 0..=23
    pub hour: u8,
    pub gender: Gender,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartError {
    /// Date is not a real Gregorian date, or hour > 23.
    InvalidDate,
    /// Year is outside the lunar table (1800..=2199).
    YearOutOfRange,
}

impl std::fmt::Display for ChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidDate => "invalid date",
            Self::YearOutOfRange => "year out of supported range (1800-2199)",
        })
    }
}

impl std::error::Error for ChartError {}

/// One of the 12 boxes of the chart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Cung {
    /// Địa Chi name with accents, e.g. "Tý".
    pub dia_chi: &'static str,
    /// Thiên Can name with accents, e.g. "Giáp".
    pub thien_can: &'static str,
    /// Role name with accents, e.g. "Mệnh", "Tài Bạch".
    pub role: &'static str,
    pub is_than: bool,
    /// Chính tinh names with accents, e.g. ["Tử Vi", "Thiên Phủ"].
    pub chinh_tinh: Vec<&'static str>,
    /// Every other natal star in this cung, Tràng Sinh ring included, e.g.
    /// ["Lộc Tồn", "Tràng Sinh"]. Sorted by the old engine's component id.
    pub phu_tinh: Vec<&'static str>,
    /// Tứ Hóa of the birth year on stars in this cung. Sorted by Hóa id:
    /// Khoa, Kỵ, Lộc, Quyền.
    pub tu_hoa: Vec<TuHoa>,
    pub tuan: bool,
    pub triet: bool,
}

/// One Hóa of the birth year and the star that takes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TuHoa {
    /// Star name with accents, e.g. "Liêm Trinh".
    pub sao: &'static str,
    /// e.g. "Hóa Lộc".
    pub hoa: &'static str,
}

/// A computed chart. Not stored: the database keeps only [`BirthInput`] and
/// rebuilds the chart on read (it takes microseconds).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Chart {
    pub input: BirthInput,
    pub lunar_day: u8,
    pub lunar_month: u8,
    pub lunar_year: i32,
    /// Hour branch, e.g. "Tý".
    pub hour_chi: &'static str,
    /// Year Can Chi, e.g. "Giáp Tý".
    pub nam_can: &'static str,
    pub nam_chi: &'static str,
    /// e.g. "Thủy Nhị cục".
    pub cuc: &'static str,
    /// Bản mệnh (nạp âm) of the lunar year, e.g. "Lộ Bàng Thổ".
    pub ban_menh: &'static str,
    /// Indexed by Địa Chi: 0 = Tý, 1 = Sửu, …, 11 = Hợi.
    pub cungs: [Cung; 12],
}

/// Build the natal chart: lunar date, year Can Chi, Mệnh/Thân, the 12 cung
/// roles and Thiên Can, Cục, Bản mệnh, the 14 chính tinh, the phụ tinh,
/// Tứ Hóa and Tuần/Triệt.
///
/// Only the non-empty `Vec` fields allocate. Release build: about
/// 0.8 µs per call, allocations included (see `tests/bench.rs`).
pub fn build_chart(input: BirthInput) -> Result<Chart, ChartError> {
    let BirthInput {
        year,
        month,
        day,
        hour,
        ..
    } = input;
    if hour > 23 || day == 0 || day > calendar::days_in_month(year, month) {
        return Err(ChartError::InvalidDate);
    }
    if !(calendar::FIRST_YEAR..=calendar::LAST_YEAR).contains(&year) {
        return Err(ChartError::YearOutOfRange);
    }

    let birth = LunarBirth::from_solar(year, month, day, hour)?;
    let lunar = birth.date;
    let nam_can = ThienCan::from_index(lunar.year + 6);
    let nam_chi = DiaChi::from_index(lunar.year + 8);

    let menh = cung::menh_position(lunar.month, birth.hour);
    let than = cung::than_position(menh, birth.hour);
    let cuc = Cuc::from_menh(nam_can, menh);
    let tu_vi = chinh_tinh::tu_vi_position(lunar.day, cuc.number());
    let sao_positions = chinh_tinh::chinh_tinh_positions(tu_vi);

    let phu_tinh_positions = phu_tinh::positions(&phu_tinh::Input {
        nam_can,
        nam_chi,
        lunar_month: lunar.month,
        lunar_day: lunar.day,
        hour: birth.hour,
        menh,
        than,
        cuc,
        van: cung::van_direction(input.gender, nam_chi),
    });
    let tuan = tuan_triet::tuan(nam_can, nam_chi);
    let triet = tuan_triet::triet(nam_can);

    // Bucket the stars by cung in one pass each. `ALL` is in the old engine's
    // id order, so each bucket is already sorted.
    let mut chinh_tinh = buckets(&sao_positions);
    for sao in Sao::ALL {
        chinh_tinh[sao_positions[sao as usize].index()].push(sao.name());
    }
    let mut phu_tinh = buckets(&phu_tinh_positions);
    for sao in PhuTinh::ALL {
        phu_tinh[phu_tinh_positions[sao as usize].index()].push(sao.name());
    }
    let mut tu_hoa: [Vec<TuHoa>; 12] = Default::default();
    for hoa in Hoa::ALL {
        let (sao, position) = match tu_hoa::target(nam_can, hoa) {
            tu_hoa::Target::Chinh(sao) => (sao.name(), sao_positions[sao as usize]),
            tu_hoa::Target::Phu(sao) => (sao.name(), phu_tinh_positions[sao as usize]),
        };
        tu_hoa[position.index()].push(TuHoa {
            sao,
            hoa: hoa.name(),
        });
    }

    let cungs = std::array::from_fn(|i| {
        let dia_chi = DiaChi::ALL[i];
        Cung {
            dia_chi: dia_chi.name(),
            thien_can: cung::cung_thien_can(nam_can, dia_chi).name(),
            role: CungRole::at(dia_chi, menh).name(),
            is_than: dia_chi == than,
            chinh_tinh: std::mem::take(&mut chinh_tinh[i]),
            phu_tinh: std::mem::take(&mut phu_tinh[i]),
            tu_hoa: std::mem::take(&mut tu_hoa[i]),
            tuan: tuan.contains(&dia_chi),
            triet: triet.contains(&dia_chi),
        }
    });

    Ok(Chart {
        input,
        lunar_day: lunar.day,
        lunar_month: lunar.month,
        lunar_year: lunar.year,
        hour_chi: birth.hour.name(),
        nam_can: nam_can.name(),
        nam_chi: nam_chi.name(),
        cuc: cuc.name(),
        ban_menh: ban_menh::name(lunar.year),
        cungs,
    })
}

/// 12 empty vectors, each with the capacity for the stars at that Địa Chi.
fn buckets(positions: &[DiaChi]) -> [Vec<&'static str>; 12] {
    let mut counts = [0_usize; 12];
    for position in positions {
        counts[position.index()] += 1;
    }
    counts.map(Vec::with_capacity)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(year: i32, month: u8, day: u8, hour: u8) -> BirthInput {
        BirthInput {
            year,
            month,
            day,
            hour,
            gender: Gender::Nam,
        }
    }

    #[test]
    fn invalid_dates() {
        for (y, m, d, h) in [
            (2000, 2, 30, 0),
            (1900, 2, 29, 0),
            (2000, 13, 1, 0),
            (2000, 0, 1, 0),
            (2000, 1, 0, 0),
            (2000, 1, 1, 24),
        ] {
            assert_eq!(build_chart(input(y, m, d, h)), Err(ChartError::InvalidDate));
        }
    }

    #[test]
    fn out_of_range() {
        for (y, m, d, h) in [
            (1799, 6, 1, 0),
            (2200, 6, 1, 0),
            (i32::MIN, 1, 1, 0),
            (i32::MAX, 12, 31, 23),
        ] {
            assert_eq!(
                build_chart(input(y, m, d, h)),
                Err(ChartError::YearOutOfRange)
            );
        }
    }
}
