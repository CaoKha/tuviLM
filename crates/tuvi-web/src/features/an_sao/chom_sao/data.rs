//! The constellation of each chính tinh, traced over its ghost portrait.
//!
//! Generated from `god-landmark-constellations.ts` and `god-ghosts.ts` of
//! the old app (branch `main`), rounded to 0.1. Points are `(x, y, power)`
//! in the portrait's 100×100 box; lines join two point indexes.

use topcoat::asset::asset;

use super::ChomSao;

#[rustfmt::skip]
pub(super) const ALL: [ChomSao; 14] = [
    ChomSao { key: "tuvi", chinh_tinh: "Tử Vi", deity: "Zeus", ghost: asset!("assets/god-ghosts/zeus.svg"),
        points: &[(44.1, 16.9, 1.0), (46.4, 19.5, 1.4), (51.7, 19.4, 1.4), (54.9, 17.3, 1.0), (26.8, 15.9, 1.4), (32.5, 3.3, 1.2), (18.3, 27.0, 1.2), (57.0, 34.6, 1.1), (53.5, 67.1, 1.2)],
        lines: &[(0, 1), (1, 2), (2, 3), (3, 0), (5, 4), (4, 6), (7, 8)] },
    ChomSao { key: "thienphu", chinh_tinh: "Thiên Phủ", deity: "Hera", ghost: asset!("assets/god-ghosts/hera.svg"),
        points: &[(44.8, 12.8, 1.1), (50.4, 12.6, 1.3), (53.9, 15.7, 1.1), (25.4, 9.1, 1.8), (29.6, 32.3, 1.2), (44.1, 40.7, 1.0), (81.2, 57.2, 1.0), (74.4, 68.7, 1.2), (83.6, 82.4, 1.0), (85.2, 74.2, 1.0), (80.6, 89.0, 1.1)],
        lines: &[(0, 1), (1, 2), (3, 4), (4, 5), (6, 7), (7, 8), (7, 10), (7, 9)] },
    ChomSao { key: "thatsat", chinh_tinh: "Thất Sát", deity: "Ares", ghost: asset!("assets/god-ghosts/ares.svg"),
        points: &[(13.5, 3.0, 1.3), (17.6, 12.5, 0.8), (20.8, 8.3, 0.8), (16.4, 6.7, 1.8), (30.9, 25.9, 1.0), (46.0, 45.2, 1.0), (69.2, 34.3, 1.0), (72.9, 44.5, 1.0), (73.0, 56.3, 1.0), (78.5, 44.8, 1.0)],
        lines: &[(5, 4), (4, 3), (1, 0), (2, 0), (6, 7), (7, 8), (6, 9), (9, 8)] },
    ChomSao { key: "phaquan", chinh_tinh: "Phá Quân", deity: "Prometheus", ghost: asset!("assets/god-ghosts/prometheus.svg"),
        points: &[(41.9, 15.5, 1.8), (18.3, 3.3, 1.4), (23.1, 8.3, 1.0), (18.5, 13.9, 1.0), (21.6, 21.8, 1.2), (35.6, 81.7, 1.0), (32.8, 57.1, 1.0), (49.7, 66.6, 1.0), (59.6, 93.2, 1.2), (41.9, 50.2, 1.0), (51.5, 28.7, 1.0)],
        lines: &[(1, 2), (2, 3), (3, 4), (10, 9), (9, 6), (6, 5), (9, 7), (7, 8), (10, 0)] },
    ChomSao { key: "thamlang", chinh_tinh: "Tham Lang", deity: "Aphrodite", ghost: asset!("assets/god-ghosts/aphodite.svg"),
        points: &[(32.2, 5.3, 1.0), (33.5, 12.9, 2.0), (34.4, 19.3, 1.0), (45.7, 25.6, 1.0), (45.6, 36.6, 1.5), (55.1, 36.0, 1.5), (55.7, 27.7, 1.0), (70.9, 27.9, 1.0), (38.3, 73.4, 1.0), (29.1, 68.2, 1.0), (23.2, 69.3, 1.0), (18.5, 83.0, 1.0), (19.3, 51.8, 1.0), (25.6, 53.8, 1.0)],
        lines: &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (6, 7), (3, 6), (6, 5), (9, 8), (10, 9), (11, 10), (12, 13)] },
    ChomSao { key: "thaiduong", chinh_tinh: "Thái Dương", deity: "Apollo", ghost: asset!("assets/god-ghosts/apollo.svg"),
        points: &[(25.8, 13.5, 1.0), (26.4, 27.5, 1.0), (36.2, 26.1, 1.0), (32.2, 13.2, 1.0), (56.0, 27.7, 2.0), (54.0, 45.4, 2.0), (50.3, 8.1, 2.0), (43.0, 20.2, 1.0), (58.8, 18.3, 1.0)],
        lines: &[(0, 1), (1, 2), (2, 3), (3, 0), (6, 7), (6, 8), (7, 4), (8, 4), (4, 5)] },
    ChomSao { key: "thaiam", chinh_tinh: "Thái Âm", deity: "Artemis", ghost: asset!("assets/god-ghosts/athemis.svg"),
        points: &[(53.5, 15.7, 1.8), (45.0, 44.4, 1.0), (38.6, 28.9, 1.0), (54.6, 29.9, 0.6), (78.8, 28.2, 1.2), (89.4, 27.5, 1.4), (43.5, 56.9, 1.0), (51.3, 68.5, 1.0), (68.9, 82.2, 1.1)],
        lines: &[(0, 2), (2, 1), (3, 4), (4, 5), (6, 7), (7, 8)] },
    ChomSao { key: "vukhuc", chinh_tinh: "Vũ Khúc", deity: "Hermes", ghost: asset!("assets/god-ghosts/hermes.svg"),
        points: &[(59.6, 27.9, 1.2), (46.1, 48.1, 1.2), (34.8, 75.2, 1.2), (59.9, 89.9, 1.2), (70.8, 63.4, 1.0), (73.2, 58.0, 1.0), (79.2, 58.6, 1.0), (79.3, 65.3, 1.2)],
        lines: &[(0, 1), (1, 2), (1, 3), (5, 6), (6, 7), (7, 4), (4, 5)] },
    ChomSao { key: "liemtrinh", chinh_tinh: "Liêm Trinh", deity: "Nemesis", ghost: asset!("assets/god-ghosts/nemesis.svg"),
        points: &[(18.9, 36.6, 1.0), (35.5, 37.0, 1.0), (26.9, 15.5, 1.6), (44.8, 25.6, 1.0), (50.0, 38.9, 1.2), (69.7, 59.3, 1.0), (74.1, 77.9, 1.3)],
        lines: &[(2, 0), (2, 1), (3, 4), (4, 5), (5, 6)] },
    ChomSao { key: "thienco", chinh_tinh: "Thiên Cơ", deity: "Athena", ghost: asset!("assets/god-ghosts/athena.svg"),
        points: &[(29.5, 66.6, 1.2), (32.7, 66.6, 1.2), (26.6, 62.9, 0.8), (34.5, 63.0, 0.8), (53.7, 14.4, 1.6), (41.9, 18.2, 1.0), (44.6, 9.0, 1.0), (54.8, 27.2, 1.0), (47.0, 40.8, 1.8), (44.3, 27.9, 1.0)],
        lines: &[(2, 0), (1, 3), (6, 4), (4, 5), (5, 6), (9, 8), (7, 8)] },
    ChomSao { key: "thienluong", chinh_tinh: "Thiên Lương", deity: "Demeter", ghost: asset!("assets/god-ghosts/demeter.svg"),
        points: &[(44.2, 40.8, 1.6), (56.9, 26.1, 1.6), (68.6, 59.2, 1.3), (62.5, 70.4, 1.0), (74.2, 70.5, 1.0), (65.1, 79.3, 1.0), (71.4, 79.0, 1.0), (29.4, 29.1, 1.3), (34.5, 18.1, 1.0), (28.4, 12.2, 1.0), (21.2, 14.0, 1.0), (19.0, 21.2, 1.0)],
        lines: &[(7, 8), (7, 9), (7, 10), (7, 11), (7, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 4), (3, 5), (4, 6), (5, 6)] },
    ChomSao { key: "thientuong", chinh_tinh: "Thiên Tướng", deity: "Hades", ghost: asset!("assets/god-ghosts/hades.svg"),
        points: &[(53.5, 27.9, 1.0), (40.4, 43.7, 1.0), (51.1, 44.1, 1.0), (25.8, 12.0, 1.8), (26.7, 34.6, 1.2), (63.0, 62.8, 1.0), (70.8, 56.2, 1.0), (75.0, 55.8, 1.0), (83.0, 60.7, 1.0), (86.7, 60.8, 1.0), (59.6, 63.3, 1.0)],
        lines: &[(3, 4), (1, 0), (0, 2), (2, 1), (10, 5), (6, 7), (8, 9)] },
    ChomSao { key: "thiendong", chinh_tinh: "Thiên Đồng", deity: "Dionysus", ghost: asset!("assets/god-ghosts/dionysos.svg"),
        points: &[(28.3, 4.0, 1.0), (31.0, 8.4, 1.0), (34.7, 8.9, 1.0), (37.5, 4.1, 1.0), (29.1, 17.3, 1.0), (34.2, 17.2, 1.0), (38.3, 31.5, 1.0), (51.1, 42.8, 1.0), (51.7, 52.8, 1.0), (63.4, 66.3, 1.0), (60.7, 77.6, 1.0)],
        lines: &[(0, 1), (1, 2), (2, 3), (3, 0), (1, 4), (2, 5), (4, 5), (6, 7), (7, 8), (8, 9), (9, 10)] },
    ChomSao { key: "cumon", chinh_tinh: "Cự Môn", deity: "Iris", ghost: asset!("assets/god-ghosts/iris.svg"),
        points: &[(30.9, 9.4, 2.0), (53.8, 30.5, 1.0), (52.5, 42.2, 1.2), (46.8, 17.7, 1.4)],
        lines: &[(2, 1), (1, 3)] },
];
