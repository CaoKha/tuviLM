//! Tử Vi chart engine. It started as a port of the old Python engine.
//!
//! Pure computation: no IO, no network, no database.
//!
//! The public types below are the API for apps. Keep them stable.

use serde::{Deserialize, Serialize};

pub mod calendar;
pub mod chinh_tinh;
pub mod cuc;
pub mod cung;
pub mod elementary;
mod lunar_table;

use chinh_tinh::Sao;
use cuc::Cuc;
use cung::CungRole;
use elementary::{DiaChi, ThienCan};

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
pub struct CungView {
    /// Địa Chi name with accents, e.g. "Tý".
    pub dia_chi: &'static str,
    /// Thiên Can name with accents, e.g. "Giáp".
    pub thien_can: &'static str,
    /// Role name with accents, e.g. "Mệnh", "Tài Bạch".
    pub role: &'static str,
    pub is_than: bool,
    /// Chính tinh names with accents, e.g. ["Tử Vi", "Thiên Phủ"].
    pub chinh_tinh: Vec<&'static str>,
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
    /// Indexed by Địa Chi: 0 = Tý, 1 = Sửu, …, 11 = Hợi.
    pub cungs: [CungView; 12],
}

/// Build the natal chart: lunar date, year Can Chi, Mệnh/Thân, the 12 cung
/// roles and Thiên Can, Cục, and the 14 chính tinh.
///
/// Only the non-empty `chinh_tinh` vectors allocate. Release build: about
/// 0.3 µs per call, allocations included (see `tests/bench.rs`).
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
    if !(lunar_table::FIRST_YEAR..=lunar_table::LAST_YEAR).contains(&year) {
        return Err(ChartError::YearOutOfRange);
    }

    let prior = calendar::Prior::from_solar(year, month, day, hour)?;
    let lunar = prior.lunar;
    let nam_can = ThienCan::from_index(lunar.year + 6);
    let nam_chi = DiaChi::from_index(lunar.year + 8);

    let menh = cung::menh_position(lunar.month, prior.hour);
    let than = cung::than_position(menh, prior.hour);
    let cuc = Cuc::from_menh(nam_can, menh);
    let tu_vi = chinh_tinh::tu_vi_position(lunar.day, cuc.number());
    let sao_positions = chinh_tinh::chinh_tinh_positions(tu_vi);

    let cungs = std::array::from_fn(|i| {
        let dia_chi = DiaChi::ALL[i];
        let count = sao_positions.iter().filter(|&&p| p == dia_chi).count();
        let mut chinh_tinh = Vec::with_capacity(count);
        chinh_tinh.extend(
            Sao::ALL
                .iter()
                .filter(|&&sao| sao_positions[sao as usize] == dia_chi)
                .map(|sao| sao.name()),
        );
        CungView {
            dia_chi: dia_chi.name(),
            thien_can: cung::cung_thien_can(nam_can, dia_chi).name(),
            role: CungRole::at(dia_chi, menh).name(),
            is_than: dia_chi == than,
            chinh_tinh,
        }
    });

    Ok(Chart {
        input,
        lunar_day: lunar.day,
        lunar_month: lunar.month,
        lunar_year: lunar.year,
        hour_chi: prior.hour.name(),
        nam_can: nam_can.name(),
        nam_chi: nam_chi.name(),
        cuc: cuc.name(),
        cungs,
    })
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
