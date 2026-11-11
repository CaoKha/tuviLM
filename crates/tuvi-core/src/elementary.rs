//! Địa Chi and Thiên Can cycles (port of `model/elementary.py`).
//!
//! Names come from `components/data/dia_chi.json` and `thien_can.json`.

/// The 12 Địa Chi, in cycle order. The discriminant is the index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DiaChi {
    Ty,
    Suu,
    Dan,
    Meo,
    Thin,
    Ti,
    Ngo,
    Mui,
    Than,
    Dau,
    Tuat,
    Hoi,
}

impl DiaChi {
    pub const ALL: [Self; 12] = [
        Self::Ty,
        Self::Suu,
        Self::Dan,
        Self::Meo,
        Self::Thin,
        Self::Ti,
        Self::Ngo,
        Self::Mui,
        Self::Than,
        Self::Dau,
        Self::Tuat,
        Self::Hoi,
    ];

    /// Cyclic constructor: any integer, taken modulo 12.
    pub const fn from_index(index: i32) -> Self {
        Self::ALL[index.rem_euclid(12) as usize]
    }

    pub const fn index(self) -> usize {
        self as usize
    }

    /// Move `steps` boxes clockwise (negative = counter-clockwise).
    pub const fn add(self, steps: i32) -> Self {
        Self::from_index(self as i32 + steps)
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Ty => "Tý",
            Self::Suu => "Sửu",
            Self::Dan => "Dần",
            Self::Meo => "Mão",
            Self::Thin => "Thìn",
            Self::Ti => "Tị",
            Self::Ngo => "Ngọ",
            Self::Mui => "Mùi",
            Self::Than => "Thân",
            Self::Dau => "Dậu",
            Self::Tuat => "Tuất",
            Self::Hoi => "Hợi",
        }
    }
}

/// The 10 Thiên Can, in cycle order. The discriminant is the index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ThienCan {
    Giap,
    At,
    Binh,
    Dinh,
    Mau,
    Ky,
    Canh,
    Tan,
    Nham,
    Quy,
}

impl ThienCan {
    pub const ALL: [Self; 10] = [
        Self::Giap,
        Self::At,
        Self::Binh,
        Self::Dinh,
        Self::Mau,
        Self::Ky,
        Self::Canh,
        Self::Tan,
        Self::Nham,
        Self::Quy,
    ];

    /// Cyclic constructor: any integer, taken modulo 10.
    pub const fn from_index(index: i32) -> Self {
        Self::ALL[index.rem_euclid(10) as usize]
    }

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn add(self, steps: i32) -> Self {
        Self::from_index(self as i32 + steps)
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Giap => "Giáp",
            Self::At => "Ất",
            Self::Binh => "Bính",
            Self::Dinh => "Đinh",
            Self::Mau => "Mậu",
            Self::Ky => "Kỷ",
            Self::Canh => "Canh",
            Self::Tan => "Tân",
            Self::Nham => "Nhâm",
            Self::Quy => "Quý",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic_index() {
        assert_eq!(DiaChi::from_index(-1), DiaChi::Hoi);
        assert_eq!(DiaChi::Dan.add(-3), DiaChi::Hoi);
        assert_eq!(ThienCan::from_index(23), ThienCan::Dinh);
        for (i, dia_chi) in DiaChi::ALL.iter().enumerate() {
            assert_eq!(dia_chi.index(), i);
        }
        for (i, thien_can) in ThienCan::ALL.iter().enumerate() {
            assert_eq!(thien_can.index(), i);
        }
    }
}
