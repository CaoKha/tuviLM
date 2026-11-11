//! Saved charts.
//!
//! The database keeps only the birth data. The chart itself is rebuilt by
//! `tuvi_core` on every read: it is pure computation and takes microseconds, so
//! there is no stale chart data to migrate when the engine changes.

use serde::Deserialize;
use topcoat::{Result, context::Cx, router::error::not_found};
use tuvi_core::{BirthInput, Chart, ChartError, Gender};

use crate::db::db;

/// Max length of a chart name, in characters.
const NAME_MAX_CHARS: usize = 80;

#[derive(Debug, toasty::Model)]
pub struct SavedChart {
    #[key]
    #[auto]
    pub id: u64,

    pub name: String,

    pub year: i32,
    pub month: i16,
    pub day: i16,
    pub hour: i16,
    /// `"Nam"` or `"Nu"`.
    pub gender: String,
}

impl SavedChart {
    pub fn birth_input(&self) -> Option<BirthInput> {
        Some(BirthInput {
            year: self.year,
            month: u8::try_from(self.month).ok()?,
            day: u8::try_from(self.day).ok()?,
            hour: u8::try_from(self.hour).ok()?,
            gender: match self.gender.as_str() {
                "Nam" => Gender::Nam,
                "Nu" => Gender::Nu,
                _ => return None,
            },
        })
    }
}

/// The form on the home page.
#[derive(Debug, Deserialize)]
pub struct NewChart {
    pub name: String,
    /// `YYYY-MM-DD`, from `<input type="date">`.
    pub birth_date: String,
    pub hour: u8,
    pub gender: Gender,
}

#[derive(Debug)]
pub enum NewChartError {
    EmptyName,
    NameTooLong,
    BadDate,
    Chart(ChartError),
}

impl std::fmt::Display for NewChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => f.write_str("Nhập tên."),
            Self::NameTooLong => write!(f, "Tên dài tối đa {NAME_MAX_CHARS} ký tự."),
            Self::BadDate => f.write_str("Ngày sinh không hợp lệ."),
            Self::Chart(ChartError::InvalidDate) => f.write_str("Ngày hoặc giờ sinh không hợp lệ."),
            Self::Chart(ChartError::YearOutOfRange) => f.write_str("Năm sinh phải từ 1800 đến 2199."),
        }
    }
}

impl NewChart {
    /// Checks the form and builds the chart once, so bad input never reaches the
    /// database.
    fn validate(&self) -> Result<(&str, BirthInput), NewChartError> {
        let name = self.name.trim();
        if name.is_empty() {
            return Err(NewChartError::EmptyName);
        }
        if name.chars().count() > NAME_MAX_CHARS {
            return Err(NewChartError::NameTooLong);
        }

        let mut parts = self.birth_date.splitn(3, '-');
        let (Some(year), Some(month), Some(day)) = (parts.next(), parts.next(), parts.next()) else {
            return Err(NewChartError::BadDate);
        };
        let (Ok(year), Ok(month), Ok(day)) = (year.parse(), month.parse(), day.parse()) else {
            return Err(NewChartError::BadDate);
        };

        let input = BirthInput { year, month, day, hour: self.hour, gender: self.gender };
        tuvi_core::build_chart(input).map_err(NewChartError::Chart)?;
        Ok((name, input))
    }
}

/// Validates the form and saves it. The outer error is a database failure; the
/// inner one is a user mistake to show next to the form.
pub async fn create(cx: &Cx, form: &NewChart) -> Result<Result<u64, NewChartError>> {
    let (name, input) = match form.validate() {
        Ok(valid) => valid,
        Err(error) => return Ok(Err(error)),
    };
    let gender = match input.gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nu",
    };
    let saved = toasty::create!(SavedChart {
        name: name,
        year: input.year,
        month: i16::from(input.month),
        day: i16::from(input.day),
        hour: i16::from(input.hour),
        gender: gender,
    })
    .exec(&mut db(cx))
    .await?;
    Ok(Ok(saved.id))
}

/// The newest charts first.
pub async fn recent(cx: &Cx, limit: usize) -> Result<Vec<SavedChart>> {
    Ok(SavedChart::all()
        .order_by(SavedChart::fields().id().desc())
        .limit(limit)
        .exec(&mut db(cx))
        .await?)
}

/// Loads a saved chart and rebuilds it. Missing or corrupt rows are a 404.
pub async fn load(cx: &Cx, id: u64) -> Result<(SavedChart, Chart)> {
    let saved = SavedChart::get_by_id(&mut db(cx), &id).await.map_err(|_| not_found())?;
    let Some(chart) = saved.birth_input().and_then(|input| tuvi_core::build_chart(input).ok()) else {
        return Err(not_found().into());
    };
    Ok((saved, chart))
}
