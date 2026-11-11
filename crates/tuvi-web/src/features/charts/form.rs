//! The "new chart" form: what the browser sends and how it is checked.

use serde::Deserialize;
use tuvi_core::{BirthInput, ChartError, Gender};

/// Max length of a chart name, in characters.
const NAME_MAX_CHARS: usize = 80;

/// The form fields, as the browser sends them.
#[derive(Debug, Deserialize)]
pub struct NewChart {
    pub name: String,
    /// `YYYY-MM-DD`, from `<input type="date">`.
    pub birth_date: String,
    pub hour: u8,
    pub gender: Gender,
}

/// A form that passed every check. Only this type can be saved.
#[derive(Debug)]
pub struct ValidChart<'a> {
    pub name: &'a str,
    pub birth: BirthInput,
}

/// A user mistake. `Display` gives the message to show next to the form.
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

        let mut parts = self.birth_date.splitn(3, '-');
        let (Some(year), Some(month), Some(day)) = (parts.next(), parts.next(), parts.next())
        else {
            return Err(NewChartError::BadDate);
        };
        let (Ok(year), Ok(month), Ok(day)) = (year.parse(), month.parse(), day.parse()) else {
            return Err(NewChartError::BadDate);
        };

        let birth = BirthInput {
            year,
            month,
            day,
            hour: self.hour,
            gender: self.gender,
        };
        tuvi_core::build_chart(birth).map_err(NewChartError::Chart)?;
        Ok(ValidChart { name, birth })
    }
}
