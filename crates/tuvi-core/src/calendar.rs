//! Solar → Vietnamese lunar date (port of `calendar/lunar.py`) and the birth
//! hour rules of `LaSoPrior.from_solar_day` (`model/prior.py`).
//!
//! No allocation: the lunar year data is a packed `static` table.

use crate::ChartError;
use crate::elementary::DiaChi;
use crate::lunar_table::{FIRST_YEAR, LAST_YEAR, LUNAR_YEARS};

/// A lunar date. `is_leap_month` is kept for completeness; the chart uses
/// only the month number (same as Python).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LunarDate {
    pub day: u8,
    pub month: u8,
    pub year: i32,
    pub is_leap_month: bool,
}

/// One decoded word of [`LUNAR_YEARS`].
#[derive(Debug, Clone, Copy)]
struct YearInfo(u32);

impl YearInfo {
    fn get(year: i32) -> Option<Self> {
        if !(FIRST_YEAR..=LAST_YEAR).contains(&year) {
            return None;
        }
        let index = usize::try_from(year - FIRST_YEAR).ok()?;
        LUNAR_YEARS.get(index).copied().map(Self)
    }

    /// Length of regular month `month` (1..=12).
    const fn month_length(self, month: u8) -> i32 {
        29 + ((self.0 >> (month - 1)) & 1) as i32
    }

    /// Leap month number, 0 if none.
    const fn leap_month(self) -> u8 {
        ((self.0 >> 12) & 0xf) as u8
    }

    const fn leap_month_length(self) -> i32 {
        29 + ((self.0 >> 16) & 1) as i32
    }

    const fn tet_offset_days(self) -> i32 {
        ((self.0 >> 17) & 0x7f) as i32
    }
}

/// Julian Day Number of a Gregorian date (same integer formula as Python).
pub const fn jdn(day: i32, month: i32, year: i32) -> i32 {
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;
    day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub const fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Convert a valid Gregorian date to the lunar date.
pub fn solar_to_lunar(year: i32, month: u8, day: u8) -> Result<LunarDate, ChartError> {
    let Some(mut info) = YearInfo::get(year) else {
        return Err(ChartError::YearOutOfRange);
    };
    let jd = jdn(i32::from(day), i32::from(month), year);
    let mut lunar_year = year;
    let mut tet = jdn(1, 1, year) + info.tet_offset_days();
    if jd < tet {
        lunar_year = year - 1;
        let Some(previous) = YearInfo::get(lunar_year) else {
            return Err(ChartError::YearOutOfRange);
        };
        info = previous;
        tet = jdn(1, 1, lunar_year) + info.tet_offset_days();
    }
    Ok(find_lunar_date(jd, lunar_year, tet, info))
}

/// Walk the month starts of one lunar year and keep the last start <= `jd`
/// (Python `find_lunar_date`).
fn find_lunar_date(jd: i32, lunar_year: i32, tet: i32, info: YearInfo) -> LunarDate {
    let leap_month = info.leap_month();
    let mut start = tet;
    let mut found = (tet, 1_u8, false);
    for month in 1..=12_u8 {
        if start > jd {
            break;
        }
        found = (start, month, false);
        start += info.month_length(month);
        if month == leap_month {
            if start > jd {
                break;
            }
            found = (start, month, true);
            start += info.leap_month_length();
        }
    }
    let (month_start, month, is_leap_month) = found;
    // A month has at most 30 days, so the offset fits in u8.
    let day = u8::try_from(jd - month_start + 1).unwrap_or(u8::MAX);
    LunarDate {
        day,
        month,
        year: lunar_year,
        is_leap_month,
    }
}

/// Birth data after the hour rules: hour branch and lunar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prior {
    pub hour: DiaChi,
    pub lunar: LunarDate,
}

impl Prior {
    /// Port of `LaSoPrior.from_solar_day`: 23h belongs to the Tý hour of the
    /// next day. The date must already be valid.
    pub fn from_solar(year: i32, month: u8, day: u8, hour: u8) -> Result<Self, ChartError> {
        let (year, month, day, hour_chi) = if hour == 23 {
            let (y, m, d) = next_day(year, month, day);
            (y, m, d, DiaChi::Ty)
        } else {
            (
                year,
                month,
                day,
                DiaChi::from_index((i32::from(hour) + 1) / 2),
            )
        };
        Ok(Self {
            hour: hour_chi,
            lunar: solar_to_lunar(year, month, day)?,
        })
    }
}

const fn next_day(year: i32, month: u8, day: u8) -> (i32, u8, u8) {
    if day < days_in_month(year, month) {
        (year, month, day + 1)
    } else if month < 12 {
        (year, month + 1, 1)
    } else {
        (year + 1, 1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jdn_reference() {
        assert_eq!(jdn(1, 1, 2000), 2_451_545);
        assert_eq!(jdn(1, 1, 1), 1_721_426);
    }

    #[test]
    fn tet_2024() {
        let lunar = solar_to_lunar(2024, 2, 10);
        assert_eq!(
            lunar,
            Ok(LunarDate {
                day: 1,
                month: 1,
                year: 2024,
                is_leap_month: false
            })
        );
    }

    #[test]
    fn hour_23_moves_to_next_day() {
        let Ok(prior) = Prior::from_solar(2023, 12, 31, 23) else {
            panic!("in range");
        };
        assert_eq!(prior.hour, DiaChi::Ty);
        assert_eq!(solar_to_lunar(2024, 1, 1), Ok(prior.lunar));
    }

    #[test]
    fn leap_month_2023() {
        // 2023 has a leap 2nd month starting 2023-03-22.
        let Ok(lunar) = solar_to_lunar(2023, 3, 22) else {
            panic!("in range");
        };
        assert_eq!((lunar.day, lunar.month, lunar.is_leap_month), (1, 2, true));
    }
}
