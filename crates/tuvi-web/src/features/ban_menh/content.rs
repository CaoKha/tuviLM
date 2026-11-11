//! The card text, from `src/content/` of the old app (branch `main`):
//! `destiny.ts`, `advice.ts` and `loc-roles.ts`.

/// Lá Bài Bản Mệnh: `(chính tinh of cung Mệnh, archetype, mantra)`. The 39
/// sets of chính tinh that cung Mệnh can hold, in any order. The empty set
/// is vô chính diệu.
#[rustfmt::skip]
pub(super) const DESTINY: [(&[&str], &str, &str); 39] = [
    (&[], "Người Tự Định Hình", "Mệnh trống không phải khoảng trống — đó là khoảng trời để ta tự định nghĩa mình là ai."),
    (&["Tử Vi"], "Bậc Đế Vương", "Quyền lực lớn nhất là quyền làm chủ chính mình."),
    (&["Thiên Phủ"], "Người Giữ Kho", "Đầy kho không bằng vững lòng — ta là chỗ cả nhà dựa vào."),
    (&["Thất Sát"], "Vị Tướng Tiên Phong", "Ta đi trước, để đường sau lưng ta trở thành đường."),
    (&["Phá Quân"], "Người Phá Trận", "Phá vỡ không phải kết thúc; đó là cách ta bắt đầu lại từ đầu."),
    (&["Tham Lang"], "Người Khát Khao", "Khát khao là ngọn lửa — ta học cách giữ lửa, không để lửa giữ ta."),
    (&["Thái Dương"], "Người Soi Sáng", "Mặt trời không xin phép ai để tỏa sáng."),
    (&["Thái Âm"], "Người Giữ Trăng", "Dịu dàng là một sức mạnh không cần chứng minh."),
    (&["Vũ Khúc"], "Người Rèn Kim", "Ta tin vào điều đếm được, và làm cho điều đó đáng tin."),
    (&["Liêm Trinh"], "Người Giữ Lửa", "Kỷ luật và đam mê trong ta không đánh nhau — chúng cùng cháy."),
    (&["Thiên Cơ"], "Người Vẽ Cơ", "Nhìn thấy con đường trước khi nó hiện ra là món quà của ta."),
    (&["Thiên Lương"], "Người Che Chở", "Che chở người khác không làm ta nhỏ đi; nó làm ta cao hơn."),
    (&["Thiên Tướng"], "Người Mang Ấn", "Lời hứa của ta nặng như ấn triện — đó là lý do người ta tin."),
    (&["Thiên Đồng"], "Người Giữ Phúc", "Ta chọn hiền hoà không phải vì yếu, mà vì biết đủ."),
    (&["Cự Môn"], "Người Mở Cánh Cửa", "Lời nói của ta là một cánh cửa — ta học cách mở đúng lúc."),
    (&["Thiên Phủ", "Tử Vi"], "Bậc Quân Chủ", "Ta không cần giành ngôi — ta xây nền, rồi ngôi tự đến."),
    (&["Tham Lang", "Tử Vi"], "Kẻ Chinh Phục", "Muốn nhiều không phải lỗi; ta biến muốn nhiều thành làm được nhiều."),
    (&["Thiên Tướng", "Tử Vi"], "Bậc Quyền Ấn", "Uy tín của ta được đóng dấu bằng hành động, không bằng lời."),
    (&["Thất Sát", "Tử Vi"], "Bậc Khai Quốc", "Ta ra lệnh cho bản thân trước — thế giới nghe theo sau."),
    (&["Phá Quân", "Tử Vi"], "Người Khai Hoàng", "Ta dám phá cả những gì mình từng xây, để xây lại cho đúng."),
    (&["Phá Quân", "Vũ Khúc"], "Người Đổi Vận", "Ta dám bỏ cả vốn lẫn vị thế để đổi lấy vận mới."),
    (&["Thiên Phủ", "Vũ Khúc"], "Bậc Trữ Tài", "Tiền đến rồi ở lại với ta, vì ta coi trọng từng đồng một."),
    (&["Tham Lang", "Vũ Khúc"], "Người Săn Tài", "Ta săn cơ hội bằng cả khát khao lẫn bản lĩnh."),
    (&["Thất Sát", "Vũ Khúc"], "Vị Tướng Kiếm Kim", "Quyết định của ta nhanh nhưng không hời — kiếm sắc vì đã qua lò rèn."),
    (&["Thiên Tướng", "Vũ Khúc"], "Người Định Giá", "Ta biết giá của tiền, và biết giá của chữ tín."),
    (&["Liêm Trinh", "Thiên Phủ"], "Người Giữ Thành", "Trong thành có kho, quanh thành có lửa — ta vừa giữ, vừa bảo vệ."),
    (&["Liêm Trinh", "Thất Sát"], "Vị Kiên Tướng", "Ta cứng với mục tiêu, và cứng hơn nữa với chính mình."),
    (&["Liêm Trinh", "Thiên Tướng"], "Bậc Chính Ấn", "Ta sống ngay thẳng đến mức người khác lấy ta làm thước."),
    (&["Liêm Trinh", "Tham Lang"], "Người Đam Mê", "Đam mê của ta không tản ra — nó hội tụ thành một mũi nhọn."),
    (&["Liêm Trinh", "Phá Quân"], "Người Phá Lập", "Ta phá luật cũ rồi tự đặt luật mới, nghiêm khắc hơn."),
    (&["Thái Âm", "Thái Dương"], "Người Hai Vầng Sáng", "Ta mang cả mặt trời lẫn mặt trăng — biết rực rỡ, biết dịu dàng."),
    (&["Thái Dương", "Thiên Lương"], "Người Nắng Ấm", "Ta tỏa sáng để sưởi ấm, không phải để thiêu đốt."),
    (&["Cự Môn", "Thái Dương"], "Người Nói Ánh Sáng", "Lời ta như nắng — thẳng, rõ, và khiến mọi thứ lộ ra."),
    (&["Thái Âm", "Thiên Cơ"], "Người Trăng Trí", "Trí tuệ của ta lên như trăng — lặng lẽ mà soi khắp."),
    (&["Thái Âm", "Thiên Đồng"], "Người Nước Hiền", "Ta mềm như nước; và như nước, không gì mài được ta."),
    (&["Cự Môn", "Thiên Đồng"], "Người Lời Hiền", "Ta nói ít hơn một chút, để lời nói ra đáng nghe hơn."),
    (&["Cự Môn", "Thiên Cơ"], "Người Biện Cơ", "Ta thấy kẽ hở trong lập luận như thấy kẽ hở trong bức tường."),
    (&["Thiên Cơ", "Thiên Lương"], "Người Mưu Đức", "Mưu trí dùng để giúp người mới là mưu trí bền."),
    (&["Thiên Đồng", "Thiên Lương"], "Người Phúc Ấm", "Phúc của ta không ồn ào — nó như bếp lửa, ấm lâu."),
];

/// Lời Khuyên: `(phrase, advice)`, two per Mệnh–Cục relation (in the order
/// of [`super::facts::MenhCuc`]): thuận lý, then nghịch lý.
#[rustfmt::skip]
pub(super) const ADVICE: [(&str, &str); 10] = [
    // sinh_xuat:thuan_ly
    ("Tiên phong, nhưng có chừng mực", "Vận của bạn do chính mình tạo ra — hành động trước, hoàn cảnh sẽ biến đổi theo. Nền thuận lý cho bạn đà tiến nhanh, nên thứ cần rèn duy nhất là sự tiết chế: quyết liệt vừa đủ để đường đi không gây tổn hại cho người xung quanh."),
    // sinh_xuat:nghich_ly
    ("Tự thân vận động", "Hoàn cảnh không tự đổi theo ý bạn — bạn là người phải hành động trước. Bối cảnh đầu đời có thể không thuận, nhưng chính vì vậy rèn được sức vượt khó; lấy dĩ hòa vi quý làm lối đi."),
    // sinh_nhap:thuan_ly
    ("Được nâng, đừng ỷ lại", "Bạn là người được hoàn cảnh hỗ trợ và ưu đãi, đà tiến vì thế thuận hơn người khác. Giữ nhịp quyết liệt của mình ở mức vừa phải — sự dễ dãi đáng sợ hơn nghịch cảnh."),
    // sinh_nhap:nghich_ly
    ("May mắn đến theo cách lạ", "Bạn được hoàn cảnh đỡ, nhưng thường theo cách không giống mình tưởng tượng. Mở lòng với sự khác biệt quanh mình — hòa hợp với nó là cách lộc trời thật sự chảy về."),
    // khac_xuat:thuan_ly
    ("Người cải cách", "Bạn có sức thay đổi môi trường quanh mình, và nền thuận lý khiến điều đó dễ được công nhận. Sức mạnh ấy cần đi cùng sự đúng đắn — cải cách để xây, không phải để thắng."),
    // khac_xuat:nghich_ly
    ("Thay đổi từ trong ra ngoài", "Bạn sinh ra để cải cách môi trường, nhưng môi trường ban đầu có thể chưa đồng điệu với bạn. Đừng ép nó đổi theo mình — hàm dưỡng sự mềm để sức thay đổi của bạn được lắng nghe."),
    // khac_nhap:thuan_ly
    ("Bị ép, nhưng không bị bẻ", "Hoàn cảnh đặt áp lực lên bản tính của bạn; may mà nền thuận lý giữ bạn đứng thẳng. Tiết chế phản ứng quyết liệt và chọn trận đánh đáng đánh — không phải trận nào cũng cần thắng."),
    // khac_nhap:nghich_ly
    ("Rèn qua nghịch cảnh", "Hoàn cảnh gây khó khăn và buộc bạn phải đổi để thích nghi — đó là lò rèn thật sự của bạn. Học cách hoá giải bất hòa giữa người với người; sự bền bỉ rèn từ nghịch cảnh là thứ không ai lấy được."),
    // binh_hoa:thuan_ly
    ("Ngang hàng, tự do", "Mệnh và hoàn cảnh không chi phối nhau — bạn tự do chọn hướng đi của mình. Nền thuận lý giúp đường đi thẳng; rèn cách sống đúng đắn để tự do ấy không trôi thành phóng túng."),
    // binh_hoa:nghich_ly
    ("Cân bằng là nghệ thuật", "Không bên nào ép bên nào giữa bạn và hoàn cảnh, nhưng hai bên cũng chẳng tự nhiên đồng điệu. Hàm dưỡng bản thân và xử lý khéo sự bất hòa — cân bằng là thứ bạn tự tạo, không phải thứ có sẵn."),
];

/// Vận May: what a cung means as the place of Hóa Lộc or Lộc Tồn.
#[rustfmt::skip]
pub(super) const LOC_ROLES: [(&str, &str); 12] = [
    ("Mệnh", "may mắn đến từ chính con người và quyết định của bạn"),
    ("Phụ Mẫu", "bạn được người lớn, thầy cô và bề trên đỡ đầu"),
    ("Phúc Đức", "phúc phần của bạn nằm ở sự an nhiên trong tâm trí"),
    ("Điền Trạch", "nhà cửa và tài sản là nền may mắn lâu dài của bạn"),
    ("Quan Lộc", "công việc chính là nơi vận may của bạn đổ về"),
    ("Nô Bộc", "bạn được nâng lên nhờ bạn bè và người đi theo mình"),
    ("Thiên Di", "ra ngoài, đi xa, đổi môi trường là mở được vận may"),
    ("Tật Ách", "sức khỏe được chăm tốt chính là nguồn lộc của bạn"),
    ("Tài Bạch", "vận may của bạn đi thẳng vào khả năng kiếm và giữ tiền"),
    ("Tử Tức", "thế hệ sau và những gì bạn tạo ra đem lộc về"),
    ("Phu Thê", "người bạn đời là cánh cửa đưa may mắn vào đời bạn"),
    ("Huynh Đệ", "anh chị em, bạn ngang hàng chính là quý nhân của bạn"),
];

/// Vận May: the lucky color of each ngũ hành, from `luck-colour.ts`, in the
/// order of [`super::facts::NguHanh`].
pub(super) const LUCK_COLORS: [&str; 5] =
    ["Nâu đất", "Vàng kim", "Xanh lam", "Xanh lục", "Đỏ san hô"];
