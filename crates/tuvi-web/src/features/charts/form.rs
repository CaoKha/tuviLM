//! The "new chart" form: what the browser sends and how it is checked.
//!
//! The An sao screen sends the date from three wheels and the hour from a
//! 12-hour clock with an AM/PM toggle, the way people know their birth time.

use serde::Deserialize;
use tuvi_core::{BirthInput, ChartError, Gender};

/// Max length of a chart name, in characters.
const NAME_MAX_CHARS: usize = 80;

/// The half of the day on the 12-hour clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Buoi {
    #[serde(rename = "AM")]
    Sang,
    #[serde(rename = "PM")]
    Chieu,
}

impl Buoi {
    /// The form value.
    pub fn value(self) -> &'static str {
        match self {
            Self::Sang => "AM",
            Self::Chieu => "PM",
        }
    }
}

/// The form value of a gender. The database keeps the same value.
pub fn gender_value(gender: Gender) -> &'static str {
    match gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nu",
    }
}

/// The hour 0–23 for a clock hour 1–12. 12 AM is 0 and 12 PM is 12.
/// Returns `None` when `gio` is not 1–12.
pub fn hour_24(gio: u8, buoi: Buoi) -> Option<u8> {
    if !(1..=12).contains(&gio) {
        return None;
    }
    let base = gio % 12;
    Some(match buoi {
        Buoi::Sang => base,
        Buoi::Chieu => base + 12,
    })
}

/// The form fields, as the browser sends them.
#[derive(Debug, Deserialize)]
pub struct NewChart {
    pub name: String,
    pub day: u8,
    pub month: u8,
    pub year: i32,
    /// Clock hour 1–12. Missing until the user picks one.
    #[serde(default)]
    pub gio: Option<u8>,
    pub buoi: Buoi,
    /// Missing until the user picks one.
    #[serde(default)]
    pub gender: Option<Gender>,
}

/// A form that passed every check. Only this type can be saved.
#[derive(Debug)]
pub struct ValidChart<'a> {
    pub name: &'a str,
    pub birth: BirthInput,
}

/// A user mistake. `Display` gives the message to show next to the form.
#[derive(Debug, PartialEq, Eq)]
pub enum NewChartError {
    EmptyName,
    NameTooLong,
    NoHour,
    NoGender,
    Chart(ChartError),
}

impl std::fmt::Display for NewChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => f.write_str("Nhập tên."),
            Self::NameTooLong => write!(f, "Tên dài tối đa {NAME_MAX_CHARS} ký tự."),
            Self::NoHour => f.write_str("Chọn giờ sinh trên đồng hồ."),
            Self::NoGender => f.write_str("Chọn giới tính."),
            Self::Chart(ChartError::InvalidDate) => f.write_str("Ngày sinh không có trên lịch."),
            Self::Chart(ChartError::YearOutOfRange) => {
                f.write_str("Ngày sinh phải từ 25/01/1800 đến 31/12/2199.")
            }
        }
    }
}

impl NewChart {
    /// Checks the fields and builds the chart once, so bad input never reaches
    /// the database.
    pub fn validate(&self) -> Result<ValidChart<'_>, NewChartError> {
        let name = self.name.trim();
        if name.is_empty() {
            return Err(NewChartError::EmptyName);
        }
        if name.chars().count() > NAME_MAX_CHARS {
            return Err(NewChartError::NameTooLong);
        }
        let hour = self
            .gio
            .and_then(|gio| hour_24(gio, self.buoi))
            .ok_or(NewChartError::NoHour)?;
        let gender = self.gender.ok_or(NewChartError::NoGender)?;

        let birth = BirthInput {
            year: self.year,
            month: self.month,
            day: self.day,
            hour,
            gender,
        };
        tuvi_core::build_chart(birth).map_err(NewChartError::Chart)?;
        Ok(ValidChart { name, birth })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form(body: &str) -> NewChart {
        serde_urlencoded::from_str(body).unwrap()
    }

    const FULL: &str = "name=Lan&day=15&month=4&year=1996&gio=10&buoi=AM&gender=Nu";

    #[test]
    fn full_form_is_valid() {
        let chart = form(FULL);
        let valid = chart.validate().unwrap();
        assert_eq!(valid.name, "Lan");
        assert_eq!(
            valid.birth,
            BirthInput {
                year: 1996,
                month: 4,
                day: 15,
                hour: 10,
                gender: Gender::Nu,
            }
        );
    }

    #[test]
    fn clock_hour_maps_to_24_hours() {
        assert_eq!(hour_24(12, Buoi::Sang), Some(0));
        assert_eq!(hour_24(1, Buoi::Sang), Some(1));
        assert_eq!(hour_24(11, Buoi::Sang), Some(11));
        assert_eq!(hour_24(12, Buoi::Chieu), Some(12));
        assert_eq!(hour_24(11, Buoi::Chieu), Some(23));
        assert_eq!(hour_24(0, Buoi::Sang), None);
        assert_eq!(hour_24(13, Buoi::Chieu), None);
    }

    #[test]
    fn missing_hour_or_gender_is_an_error() {
        let no_hour = form("name=Lan&day=1&month=1&year=2000&buoi=PM&gender=Nam");
        assert_eq!(no_hour.validate().unwrap_err(), NewChartError::NoHour);
        let bad_hour = form("name=Lan&day=1&month=1&year=2000&gio=13&buoi=PM&gender=Nam");
        assert_eq!(bad_hour.validate().unwrap_err(), NewChartError::NoHour);
        let no_gender = form("name=Lan&day=1&month=1&year=2000&gio=3&buoi=PM");
        assert_eq!(no_gender.validate().unwrap_err(), NewChartError::NoGender);
    }

    #[test]
    fn name_is_trimmed_and_checked() {
        let blank = form("name=++&day=1&month=1&year=2000&gio=3&buoi=PM&gender=Nam");
        assert_eq!(blank.validate().unwrap_err(), NewChartError::EmptyName);
        let long = NewChart {
            name: "a".repeat(NAME_MAX_CHARS + 1),
            ..form(FULL)
        };
        assert_eq!(long.validate().unwrap_err(), NewChartError::NameTooLong);
    }

    #[test]
    fn impossible_dates_are_errors() {
        let feb_30 = form("name=Lan&day=30&month=2&year=2000&gio=3&buoi=PM&gender=Nam");
        assert_eq!(
            feb_30.validate().unwrap_err(),
            NewChartError::Chart(ChartError::InvalidDate)
        );
        let too_late = form("name=Lan&day=1&month=1&year=2300&gio=3&buoi=PM&gender=Nam");
        assert_eq!(
            too_late.validate().unwrap_err(),
            NewChartError::Chart(ChartError::YearOutOfRange)
        );
    }

    #[test]
    fn unknown_buoi_does_not_parse() {
        let body = "name=Lan&day=1&month=1&year=2000&gio=3&buoi=XX&gender=Nam";
        assert!(serde_urlencoded::from_str::<NewChart>(body).is_err());
    }
}
