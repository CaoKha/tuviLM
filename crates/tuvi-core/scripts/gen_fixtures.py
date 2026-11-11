"""Generate parity fixtures for tuvi-core from the old Python engine (the oracle).

The Python engine is no longer in this repo. Run this script inside a checkout
of commit 0d69bed (the last commit with `src/refactored/`), from the root of
that checkout, with its uv environment:

    git worktree add /tmp/py-oracle 0d69bed
    cd /tmp/py-oracle
    uv sync
    uv run python <this-repo>/crates/tuvi-core/scripts/gen_fixtures.py

Writes <this-repo>/crates/tuvi-core/tests/fixtures/charts.json (next to this
script). Each case has the input and either the expected chart (same JSON shape
as `tuvi_core::Chart`) or the expected error name. Every expected value comes
from the Python engine. Do not edit the output by hand.
"""

from __future__ import annotations

import datetime as dt
import json
import random
import sys
from pathlib import Path

# The Python engine is imported from the current directory (the 0d69bed checkout).
ORACLE_ROOT = Path.cwd()
sys.path.insert(0, str(ORACLE_ROOT))

try:
    from src.refactored.calendar.lunar import get_year_info  # noqa: E402
except ModuleNotFoundError:
    sys.exit("Run this script from the root of a checkout of commit 0d69bed.")
from src.refactored.components.definitions.sao import (  # noqa: E402
    ChinhPhuTinh,
    TuanTriet,
    TuHoa,
    VongTrangSinh,
)
from src.refactored.la_so import LaSo  # noqa: E402
from src.refactored.model.elementary import DiaChi  # noqa: E402
from src.refactored.model.layer import NATAL_LAYER_ID  # noqa: E402
from src.refactored.model.prior import Gender, LaSoPrior  # noqa: E402
from src.refactored.placement.rules.loader import (  # noqa: E402
    load_tu_hoa_target_mapping,
)

OUT = Path(__file__).resolve().parents[1] / "tests/fixtures/charts.json"
JD_ORDINAL_OFFSET = 1721425  # JDN(0001-01-01) - 1
GENDERS = {"Nam": Gender.MALE, "Nu": Gender.FEMALE}
TU_HOA_TARGETS = load_tu_hoa_target_mapping()


def jd_to_date(jd: int) -> dt.date:
    return dt.date.fromordinal(jd - JD_ORDINAL_OFFSET)


def van_han_by_cung(la_so: LaSo) -> tuple[dict, dict]:
    """Invert the period focus maps: cung -> Đại hạn age range, cung -> the
    year Địa Chi whose Tiểu hạn falls there. Both maps are one-to-one."""
    dai_han = {}
    for age_range, focus in la_so.dai_han_focus_map().by_range.items():
        assert focus not in dai_han, focus
        dai_han[focus] = (age_range.start_age, age_range.end_age)
    tieu_han = {}
    for year_dia_chi, focus in la_so.tieu_han_focus_map().items():
        assert focus not in tieu_han, focus
        tieu_han[focus] = year_dia_chi
    assert len(dai_han) == len(tieu_han) == len(DiaChi)
    return dai_han, tieu_han


def build_cung(la_so: LaSo, dia_chi: DiaChi, van_han: tuple[dict, dict]) -> dict:
    """One cung. Lists keep the Python order: component ids sorted as strings."""
    name = lambda component_id: la_so.component(component_id).name  # noqa: E731
    cung = la_so.cung_at(dia_chi)
    dai_han, tieu_han = van_han
    tu_hoa_targets = TU_HOA_TARGETS[la_so.prior.thien_can]
    chinh_tinh, phu_tinh, tu_hoa = [], [], []
    tuan = triet = False
    for component_id in cung.components_in_layer(NATAL_LAYER_ID):
        component = la_so.component(component_id)
        if isinstance(component, ChinhPhuTinh) and component.is_chinh_tinh:
            chinh_tinh.append(component.name)
        elif isinstance(component, (ChinhPhuTinh, VongTrangSinh)):
            phu_tinh.append(component.name)
        elif isinstance(component, TuHoa):
            target = tu_hoa_targets[component_id]
            assert la_so.position_of(target) == dia_chi, (component_id, target)
            tu_hoa.append({"sao": name(target), "hoa": component.name})
        elif isinstance(component, TuanTriet):
            if component_id.startswith("tuan_"):
                tuan = True
            elif component_id.startswith("triet_"):
                triet = True
            else:
                raise ValueError(f"unknown Tuần/Triệt id: {component_id}")
        else:
            raise TypeError(f"unexpected natal component: {component_id}")
    return {
        "dia_chi": name(cung.dia_chi.value),
        "thien_can": name(cung.thien_can.value),
        "role": name(cung.natal_role.value),
        "is_than": cung.is_cung_than,
        "chinh_tinh": chinh_tinh,
        "phu_tinh": phu_tinh,
        "tu_hoa": tu_hoa,
        "tuan": tuan,
        "triet": triet,
        "dai_han_start": dai_han[dia_chi][0],
        "dai_han_end": dai_han[dia_chi][1],
        "tieu_han": name(tieu_han[dia_chi].value),
    }


def build_expected(inp: dict) -> dict:
    time = dt.datetime(inp["year"], inp["month"], inp["day"], inp["hour"])
    try:
        prior = LaSoPrior.from_solar_day(time, GENDERS[inp["gender"]])
    except ValueError as exc:
        if "supported range" in str(exc):
            return {"input": inp, "error": "YearOutOfRange"}
        raise
    la_so = LaSo.from_prior(prior)
    name = lambda component_id: la_so.component(component_id).name  # noqa: E731
    van_han = van_han_by_cung(la_so)

    chart = {
        "input": inp,
        "lunar_day": prior.date,
        "lunar_month": prior.month,
        "lunar_year": prior.year,
        "hour_chi": name(prior.hour.value),
        "nam_can": name(prior.thien_can.value),
        "nam_chi": name(prior.dia_chi.value),
        "cuc": la_so.natal_context.cuc.name,
        "ban_menh": la_so.ban_menh.name,
        "cungs": [build_cung(la_so, dia_chi, van_han) for dia_chi in DiaChi],
    }
    return {"input": inp, "expected": chart}


def make_input(d: dt.date, hour: int, gender: str) -> dict:
    return {
        "year": d.year,
        "month": d.month,
        "day": d.day,
        "hour": hour,
        "gender": gender,
    }


def random_inputs(rng: random.Random, count: int) -> list[dict]:
    start = dt.date(1900, 1, 1).toordinal()
    end = dt.date(2100, 12, 31).toordinal()
    inputs = []
    for i in range(count):
        d = dt.date.fromordinal(rng.randint(start, end))
        inputs.append(make_input(d, i % 24, "Nam" if (i // 24) % 2 == 0 else "Nu"))
    return inputs


def edge_inputs(rng: random.Random) -> list[dict]:
    inputs: list[dict] = []
    genders = ("Nam", "Nu")

    # Tết days: the day before Tết at 22h/23h and Tết itself at 0h/23h.
    for year in (1801, 1900, 1950, 1985, 2000, 2024, 2025, 2050, 2100, 2199):
        tet = jd_to_date(get_year_info(year)[0].julian_day_number)
        eve = tet - dt.timedelta(days=1)
        for d, hour in ((eve, 22), (eve, 23), (tet, 0), (tet, 23)):
            inputs.append(make_input(d, hour, genders[len(inputs) % 2]))

    # Leap months: first and last day of each leap month in some years.
    leap_years = [y for y in range(1900, 2101) if any(m.is_leap_month for m in get_year_info(y))]
    for year in rng.sample(leap_years, 12):
        months = get_year_info(year)
        for i, month in enumerate(months):
            if not month.is_leap_month:
                continue
            first = jd_to_date(month.julian_day_number)
            last = jd_to_date(months[i + 1].julian_day_number - 1)
            inputs.append(make_input(first, 12, genders[len(inputs) % 2]))
            inputs.append(make_input(last, 23, genders[len(inputs) % 2]))

    # 23h on the last day of a lunar month (moves to day 1 of the next month).
    for year in rng.sample(range(1900, 2100), 12):
        months = get_year_info(year)
        i = rng.randrange(len(months) - 1)
        last = jd_to_date(months[i + 1].julian_day_number - 1)
        inputs.append(make_input(last, 23, genders[len(inputs) % 2]))
        inputs.append(make_input(last, 22, genders[len(inputs) % 2]))

    # Solar year boundaries and leap days.
    for year in (1899, 1999, 2000, 2023, 2099):
        inputs.append(make_input(dt.date(year, 12, 31), 23, genders[len(inputs) % 2]))
        inputs.append(make_input(dt.date(year + 1, 1, 1), 0, genders[len(inputs) % 2]))
    for year in (1904, 2000, 2024, 2096):
        inputs.append(make_input(dt.date(year, 2, 28), 23, "Nam"))
        inputs.append(make_input(dt.date(year, 2, 29), 23, "Nu"))

    # Table limits: 1800 before/at Tết, 2199 end, years outside the table.
    tet_1800 = jd_to_date(get_year_info(1800)[0].julian_day_number)
    for d, hour in (
        (dt.date(1800, 1, 1), 0),
        (dt.date(1800, 1, 15), 12),
        (tet_1800 - dt.timedelta(days=1), 22),
        (tet_1800 - dt.timedelta(days=1), 23),
        (tet_1800, 0),
        (tet_1800, 13),
        (dt.date(1800, 12, 31), 23),
        (dt.date(1799, 12, 31), 23),
        (dt.date(2199, 1, 1), 0),
        (dt.date(2199, 12, 31), 0),
        (dt.date(2199, 12, 31), 22),
        (dt.date(2199, 12, 31), 23),
        (dt.date(2200, 1, 1), 0),
    ):
        inputs.append(make_input(d, hour, genders[len(inputs) % 2]))
    return inputs


def main() -> None:
    rng = random.Random(20261111)
    inputs = random_inputs(rng, 240) + edge_inputs(rng)
    cases = [build_expected(inp) for inp in inputs]
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(
        json.dumps(cases, ensure_ascii=False, indent=1) + "\n", encoding="utf-8"
    )
    errors = sum("error" in case for case in cases)
    print(f"wrote {len(cases)} cases ({errors} errors) to {OUT}")


if __name__ == "__main__":
    main()
