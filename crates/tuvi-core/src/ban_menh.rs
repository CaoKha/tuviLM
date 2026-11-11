//! Bản mệnh (nạp âm) of the birth year (port of
//! `components/definitions/ban_menh.py`). Names come from
//! `components/data/ban_menh.json`.

/// The 30 Bản mệnh of the 60-year cycle. Each one covers 2 years, and the
/// first one starts in 1948 (Mậu Tý).
const CYCLE: [&str; 30] = [
    "Tích Lịch Hỏa",
    "Tùng Bách Mộc",
    "Trường Lưu Thủy",
    "Sa Trung Kim",
    "Sơn Hạ Hỏa",
    "Bình Địa Mộc",
    "Bích Thượng Thổ",
    "Kim Bạch Kim",
    "Phú Đăng Hỏa",
    "Thiên Hà Thủy",
    "Đại Dịch Thổ",
    "Thoa Xuyến Kim",
    "Tang Đố Mộc",
    "Đại Khê Thủy",
    "Sa Trung Thổ",
    "Thiên Thượng Hỏa",
    "Thạch Lựu Mộc",
    "Đại Hải Thủy",
    "Hải Trung Kim",
    "Lộ Trung Hỏa",
    "Đại Lâm Mộc",
    "Lộ Bàng Thổ",
    "Kiếm Phong Kim",
    "Sơn Đầu Hỏa",
    "Giản Hạ Thủy",
    "Thành Đầu Thổ",
    "Bạch Lạp Kim",
    "Dương Liễu Mộc",
    "Tuyền Trung Thủy",
    "Ốc Thượng Thổ",
];

/// Bản mệnh name of a lunar year, e.g. 1990 → "Lộ Bàng Thổ".
pub const fn name(lunar_year: i32) -> &'static str {
    CYCLE[(lunar_year - 1948).rem_euclid(60) as usize / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(name(1990), "Lộ Bàng Thổ");
        assert_eq!(name(1984), "Hải Trung Kim");
        assert_eq!(name(1947), "Ốc Thượng Thổ");
    }
}
