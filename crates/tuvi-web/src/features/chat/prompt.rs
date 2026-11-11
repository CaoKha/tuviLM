//! The system prompt: who Thiên Hạc is, the rules, and the chart as text.

use std::fmt::Write;

use tuvi_core::{Chart, Gender};

/// Persona and rules. Adapted from the old Python agent (`src/agent/main.py`):
/// that agent read the chart with tools; here the chart is in the prompt.
const INSTRUCTIONS: &str = "\
Bạn là Thiên Hạc, trợ lý luận giải lá số Tử Vi. Bạn trả lời câu hỏi về lá số bên dưới:
- Luận giải các khía cạnh của cuộc đời, tức là 12 cung của lá số.
- Giải thích ý nghĩa các sao, Tứ Hóa, Tuần/Triệt, Tràng Sinh và ảnh hưởng của chúng.

## Kết cấu lá số
- Lá số có 12 cung, đặt theo Địa Chi từ Tý đến Hợi. Mỗi cung có một vai trò: Mệnh, Phụ Mẫu, Phúc Đức, Điền Trạch, Quan Lộc, Nô Bộc, Thiên Di, Tật Ách, Tài Bạch, Tử Tức, Phu Thê, Huynh Đệ.
- Mỗi cung có: chính tinh (ảnh hưởng lớn nhất), phụ tinh, có thể có Tuần hoặc Triệt, Tứ Hóa (Lộc, Quyền, Khoa, Kỵ) và một vị trí của vòng Tràng Sinh.
- Các sao cùng cung tạo hiệu ứng chung. Hiệu ứng chung quan trọng hơn ý nghĩa từng sao riêng lẻ.
- Khi luận một cung, xét cả các cung tam hợp và cung xung chiếu (đã ghi trong dữ liệu).

## Nguyên tắc bắt buộc
1. Chỉ dựa vào dữ liệu lá số bên dưới. Không bịa thêm sao, vị trí hay cách cục.
2. Nếu thiếu dữ liệu để kết luận, nói rõ phần nào còn thiếu. Dữ liệu chưa có: cách cục, trạng thái Đắc/Hãm của sao, và sách Tử Vi Tân Biên.
3. Khi các yếu tố trái chiều, ưu tiên theo thứ tự: Chính tinh > Tuần/Triệt > Tứ Hóa > Phụ tinh > Tràng Sinh > Xung chiếu > Tam hợp.
4. Giọng điềm đạm, rõ ràng, có chiều sâu. Không phán chắc điều dữ liệu không hỗ trợ.
5. Trả lời bằng tiếng Việt, bằng markdown ngắn gọn. Chỉ trả lời câu được hỏi; không luận cả lá số nếu câu hỏi chỉ về một cung.
";

/// The full system prompt for one chart.
pub fn system_prompt(name: &str, chart: &Chart) -> String {
    let mut prompt = String::with_capacity(INSTRUCTIONS.len() + 4096);
    prompt.push_str(INSTRUCTIONS);
    // `write!` to a String cannot fail.
    let _ = write_chart(&mut prompt, name, chart);
    prompt
}

fn write_chart(out: &mut String, name: &str, chart: &Chart) -> std::fmt::Result {
    let input = chart.input;
    let gender = match input.gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nữ",
    };
    let than = chart
        .cungs
        .iter()
        .find(|cung| cung.is_than)
        .map_or("?", |cung| cung.role);
    writeln!(out, "\n## Lá số của {name}")?;
    writeln!(out, "- Giới tính: {gender}")?;
    writeln!(
        out,
        "- Dương lịch: {}/{}/{}, {} giờ",
        input.day, input.month, input.year, input.hour
    )?;
    writeln!(
        out,
        "- Âm lịch: {}/{}/{}, giờ {}",
        chart.lunar_day, chart.lunar_month, chart.lunar_year, chart.hour_chi
    )?;
    writeln!(
        out,
        "- Năm {} {}, {}, bản mệnh {}",
        chart.nam_can, chart.nam_chi, chart.cuc, chart.ban_menh
    )?;
    writeln!(out, "- Thân cư {than}")?;

    writeln!(out, "\n## 12 cung")?;
    // `cungs` is indexed by Địa Chi, so tam hợp are 4 apart and xung chiếu is 6 apart.
    let role_at = |index: usize| chart.cungs[index % 12].role;
    for (index, cung) in chart.cungs.iter().enumerate() {
        let list = |stars: &[&str]| {
            if stars.is_empty() {
                "không".to_owned()
            } else {
                stars.join(", ")
            }
        };
        write!(
            out,
            "\n### {} tại {} ({} {})",
            cung.role, cung.dia_chi, cung.thien_can, cung.dia_chi
        )?;
        if cung.is_than {
            write!(out, ", cũng là cung Thân")?;
        }
        writeln!(out)?;
        let chinh_tinh = if cung.chinh_tinh.is_empty() {
            "Vô chính diệu".to_owned()
        } else {
            cung.chinh_tinh.join(", ")
        };
        writeln!(out, "- Chính tinh: {chinh_tinh}")?;
        writeln!(out, "- Phụ tinh (gồm Tràng Sinh): {}", list(&cung.phu_tinh))?;
        if !cung.tu_hoa.is_empty() {
            let tu_hoa = cung
                .tu_hoa
                .iter()
                .map(|tu_hoa| format!("{} ({})", tu_hoa.hoa, tu_hoa.sao))
                .collect::<Vec<_>>();
            writeln!(out, "- Tứ Hóa: {}", tu_hoa.join(", "))?;
        }
        match (cung.tuan, cung.triet) {
            (true, true) => writeln!(out, "- Có Tuần và Triệt")?,
            (true, false) => writeln!(out, "- Có Tuần")?,
            (false, true) => writeln!(out, "- Có Triệt")?,
            (false, false) => {}
        }
        writeln!(
            out,
            "- Tam hợp với {} và {}; xung chiếu {}",
            role_at(index + 4),
            role_at(index + 8),
            role_at(index + 6)
        )?;
        writeln!(
            out,
            "- Đại hạn {}–{} tuổi; tiểu hạn năm {}",
            cung.dai_han_start, cung.dai_han_end, cung.tieu_han
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::system_prompt;
    use tuvi_core::{BirthInput, Gender, build_chart};

    #[test]
    fn prompt_has_the_whole_chart() {
        let input = BirthInput {
            year: 1990,
            month: 5,
            day: 15,
            hour: 8,
            gender: Gender::Nam,
        };
        let Ok(chart) = build_chart(input) else {
            panic!("chart")
        };
        let prompt = system_prompt("Nguyễn Văn A", &chart);
        assert!(prompt.contains("## Lá số của Nguyễn Văn A"));
        assert_eq!(prompt.matches("\n### ").count(), 12);
        assert!(prompt.contains("### Mệnh tại Sửu"));
        assert!(prompt.contains("- Chính tinh: Thiên Cơ"));
        assert!(prompt.contains("Thân cư Tài Bạch"));
    }
}
