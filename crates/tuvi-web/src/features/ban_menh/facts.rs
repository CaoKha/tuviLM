//! What the three cards read from the chart: the old `selectors.ts`, and the
//! fields that the old API added to the chart (Mệnh–Cục relation, âm dương,
//! ngũ hành of Bản mệnh).

use tuvi_core::Chart;

use super::content::{ADVICE, DESTINY, LOC_ROLES, LUCK_COLORS};
use crate::features::charts::menh_chinh_tinh;

/// Ngũ hành in the sinh order: each one sinh the next, and khắc the one
/// after that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NguHanh {
    Tho,
    Kim,
    Thuy,
    Moc,
    Hoa,
}

impl NguHanh {
    /// From a name that `tuvi_core` writes, such as "Thổ".
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "Thổ" => Self::Tho,
            "Kim" => Self::Kim,
            "Thủy" => Self::Thuy,
            "Mộc" => Self::Moc,
            "Hỏa" => Self::Hoa,
            _ => return None,
        })
    }

    /// The suffix of its CSS color variables: `--element-{css}`.
    pub(super) fn css(self) -> &'static str {
        ["tho", "kim", "thuy", "moc", "hoa"][self as usize]
    }

    pub(super) fn luck_color(self) -> &'static str {
        LUCK_COLORS[self as usize]
    }
}

/// How the ngũ hành of Bản mệnh meets the ngũ hành of Cục.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenhCuc {
    SinhXuat,
    SinhNhap,
    KhacXuat,
    KhacNhap,
    BinhHoa,
}

impl MenhCuc {
    fn of(menh: NguHanh, cuc: NguHanh) -> Self {
        match (cuc as usize + 5 - menh as usize) % 5 {
            0 => Self::BinhHoa,
            1 => Self::SinhXuat,
            2 => Self::KhacXuat,
            3 => Self::KhacNhap,
            _ => Self::SinhNhap,
        }
    }

    pub(super) fn label(self) -> &'static str {
        [
            "Mệnh sinh cục",
            "Cục sinh mệnh",
            "Mệnh khắc cục",
            "Cục khắc mệnh",
            "Mệnh cục bình hoà",
        ][self as usize]
    }
}

/// Everything the deck shows about one chart.
pub(super) struct Facts<'a> {
    /// The chính tinh of cung Mệnh. Empty for vô chính diệu.
    pub chinh_tinh: &'a [&'static str],
    /// The destiny card: `(archetype, mantra)`.
    pub destiny: (&'static str, &'static str),
    /// The roles of the cung that hold Hóa Lộc and Lộc Tồn.
    pub hoa_loc: Option<&'static str>,
    pub loc_ton: Option<&'static str>,
    /// Bản mệnh (nạp âm) and its ngũ hành, which gives the lucky color.
    pub ban_menh: &'static str,
    pub ban_menh_hanh: NguHanh,
    /// Tháng cát: the lunar months of the tam hợp and lục hợp partners of
    /// the birth year's Địa Chi, in order.
    pub thang_cat: [usize; 3],
    pub menh_cuc: MenhCuc,
    /// Thuận lý when the birth year's Địa Chi and cung Mệnh are both Dương
    /// or both Âm.
    pub thuan_ly: bool,
}

impl<'a> Facts<'a> {
    /// `None` only when the chart holds a name this module does not know.
    pub(super) fn of(chart: &'a Chart) -> Option<Self> {
        let chinh_tinh = menh_chinh_tinh(chart);
        let menh = chart.cungs.iter().position(|cung| cung.role == "Mệnh")?;
        let year = chart
            .cungs
            .iter()
            .position(|cung| cung.dia_chi == chart.nam_chi)?;
        // Bản mệnh ends with its ngũ hành ("Lộ Bàng Thổ"), Cục starts with
        // it ("Thủy Nhị cục").
        let ban_menh_hanh = NguHanh::from_name(chart.ban_menh.rsplit(' ').next()?)?;
        let cuc_hanh = NguHanh::from_name(chart.cuc.split(' ').next()?)?;
        // Tháng Giêng is Dần (index 2).
        let month = |chi: usize| (chi + 10) % 12 + 1;
        let mut thang_cat = [
            month((year + 4) % 12),
            month((year + 8) % 12),
            month((13 - year) % 12),
        ];
        thang_cat.sort_unstable();
        Some(Self {
            chinh_tinh,
            destiny: destiny(chinh_tinh),
            hoa_loc: role_holding(chart, "Hóa Lộc"),
            loc_ton: role_holding(chart, "Lộc Tồn"),
            ban_menh: chart.ban_menh,
            ban_menh_hanh,
            thang_cat,
            menh_cuc: MenhCuc::of(ban_menh_hanh, cuc_hanh),
            thuan_ly: year % 2 == menh % 2,
        })
    }

    /// The advice card: `(phrase, advice)`.
    pub(super) fn advice(&self) -> (&'static str, &'static str) {
        ADVICE[self.menh_cuc as usize * 2 + usize::from(!self.thuan_ly)]
    }
}

/// The destiny entry of a set of chính tinh, in any order. An unknown set
/// gets the vô chính diệu entry, so the card never shows blank.
fn destiny(chinh_tinh: &[&str]) -> (&'static str, &'static str) {
    let entry = DESTINY
        .iter()
        .find(|(stars, ..)| {
            stars.len() == chinh_tinh.len() && stars.iter().all(|sao| chinh_tinh.contains(sao))
        })
        .unwrap_or(&DESTINY[0]);
    (entry.1, entry.2)
}

/// The role of the cung that holds `sao`, as a Tứ Hóa or as a phụ tinh.
fn role_holding(chart: &Chart, sao: &str) -> Option<&'static str> {
    chart
        .cungs
        .iter()
        .find(|cung| cung.phu_tinh.contains(&sao) || cung.tu_hoa.iter().any(|hoa| hoa.hoa == sao))
        .map(|cung| cung.role)
}

/// What a cung role means as the place of lộc.
pub(super) fn loc_blurb(role: &str) -> &'static str {
    LOC_ROLES
        .iter()
        .find(|(name, _)| *name == role)
        .map_or("", |(_, blurb)| blurb)
}

#[cfg(test)]
mod tests {
    use tuvi_core::{BirthInput, Gender, build_chart};

    use super::*;

    #[test]
    fn menh_cuc_follows_sinh_and_khac() {
        use NguHanh::*;
        assert_eq!(MenhCuc::of(Hoa, Tho), MenhCuc::SinhXuat);
        assert_eq!(MenhCuc::of(Tho, Hoa), MenhCuc::SinhNhap);
        assert_eq!(MenhCuc::of(Tho, Thuy), MenhCuc::KhacXuat);
        assert_eq!(MenhCuc::of(Thuy, Tho), MenhCuc::KhacNhap);
        assert_eq!(MenhCuc::of(Kim, Kim), MenhCuc::BinhHoa);
    }

    #[test]
    fn destiny_ignores_star_order() {
        assert_eq!(destiny(&["Tử Vi", "Thiên Phủ"]).0, "Bậc Quân Chủ");
        assert_eq!(destiny(&["Thiên Phủ", "Tử Vi"]).0, "Bậc Quân Chủ");
        assert_eq!(destiny(&[]).0, "Người Tự Định Hình");
        assert_eq!(destiny(&["Tử Vi", "Cự Môn"]).0, "Người Tự Định Hình");
    }

    /// Charts over one 60-year cycle (4 dates a year, every second hour)
    /// all give facts, a known destiny, both lộc and a known role blurb.
    #[test]
    fn every_chart_has_all_facts() {
        for year in 1950..2010 {
            for month in [1, 4, 7, 11] {
                for hour in (0..24).step_by(2) {
                    let chart = chart(year, month, hour);
                    let facts = Facts::of(&chart).unwrap();
                    assert!(facts.chinh_tinh.is_empty() || facts.destiny.0 != DESTINY[0].1);
                    for role in [facts.hoa_loc.unwrap(), facts.loc_ton.unwrap()] {
                        assert!(!loc_blurb(role).is_empty(), "{role}");
                    }
                }
            }
        }
    }

    #[test]
    fn thang_cat_matches_the_old_table() {
        // 1984 is Giáp Tý: Tý has tam hợp Thìn, Thân and lục hợp Sửu.
        let ty = Facts::of(&chart(1984, 6, 8)).map(|facts| facts.thang_cat);
        assert_eq!(ty, Some([3, 7, 12]));
        // 1990 is Canh Ngọ: tam hợp Dần, Tuất and lục hợp Mùi.
        let ngo = Facts::of(&chart(1990, 6, 8)).map(|facts| facts.thang_cat);
        assert_eq!(ngo, Some([1, 6, 9]));
    }

    fn chart(year: i32, month: u8, hour: u8) -> Chart {
        let birth = BirthInput {
            year,
            month,
            day: 15,
            hour,
            gender: Gender::Nu,
        };
        build_chart(birth).unwrap()
    }
}
