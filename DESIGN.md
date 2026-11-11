# Design

The product name is **Thiên Hạc**. The look is a dark "twilight" sky with
frosted glass cards. One accent color comes from the user's chart.

**Reference:** [`docs/design/prototype.html`](docs/design/prototype.html) is the
full design prototype. Open it in a browser. When this file and the prototype
do not agree, the prototype is correct.

**Code:** the tokens and utilities are in
[`crates/tuvi-web/styles.css`](crates/tuvi-web/styles.css) (Tailwind v4).

## Colors

| Token | Value | Use |
| --- | --- | --- |
| `page` | `#120a24` | Page background |
| `bg-0`, `bg-1`, `bg-2` | `#1a1033`, `#2e1a4a`, `#3b2063` | Twilight shades. `bg-0` is also text on accent fills. |
| `ink` | `#f3effa` | Main text |
| `muted` | `#b9aed1` | Labels and secondary text |
| `coral` | `#ff9e7d` | Flags, for example "Thân" |
| `fire` | `#ff5d6c` | Errors |
| `glass`, `glass-line` | white 6%, ink 14% | Card fill and hairlines |
| `accent`, `accent-2`, `accent-glow` | per chart | Highlights, star names, primary button |

Never use black. Never put a color value in a `.rs` file: use a token class.

## Accent from ngũ hành

The accent is the element of the chính tinh in cung Mệnh:

| Element | Color | Chính tinh |
| --- | --- | --- |
| Kim | `#e0dbcf` | Thất Sát, Vũ Khúc |
| Mộc | `#63c79a` | Tham Lang, Thiên Cơ, Thiên Lương |
| Thủy | `#79b7ff` | Phá Quân, Thái Âm, Thiên Tướng, Thiên Đồng, Cự Môn |
| Hỏa | `#ff5d57` | Thái Dương, Liêm Trinh |
| Thổ | `#b3855a` | Tử Vi, Thiên Phủ |

- One star: `accent` is the element, `accent-2` is its light shade.
- Two stars: `accent` is the first element, `accent-2` is the second.
- No star (vô chính diệu) or no chart: the neutral lavender `#bca6f5`.

The chart page sets `--accent` on its `<main>`
(`crates/tuvi-web/src/features/charts/accent.rs`).

## Type

| Font | Use |
| --- | --- |
| Fraunces (`font-display`) | Titles (`h1`–`h3`), star names, Địa Chi |
| Be Vietnam Pro (`font-sans`) | Everything else |

Both fonts load with the Vietnamese subset. Small labels are uppercase, bold,
10px, with wide letter spacing (`tracking-[0.2em]` or more).

## Surfaces and controls

| Class | Use |
| --- | --- |
| `glass` | Every card and panel: glass fill, hairline border, 26px radius, blur |
| `pill-primary` | The one main action on a screen: accent gradient, round |
| `pill-ghost` | Other choices, for example the gender toggle |
| `cosmos` | The fixed page backdrop, with two slow nebulae |

## Chart

- A 4×4 grid. The 12 cung go around the edge, read clockwise from Tị at the top
  left. The center 2×2 shows the name and birth data.
- Each cung shows: role (top left), Địa Chi (top right), chính tinh in the
  accent color, "Vô chính diệu" when empty, and Thiên Can (bottom).
- Cung Mệnh has an accent border and a light accent fill.

## Rules

- Mobile first. Check every page at 390px wide and at 1280px wide.
- Honor `prefers-reduced-motion` (`styles.css` handles it once for all pages).
- Scrollbars are hidden, but every region still scrolls.
