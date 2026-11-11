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
(`crates/tuvi-web/src/features/charts/accent.rs`). The An sao preview sets it
on `:root` with a `<style>` rule, so the whole page changes.

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

## Navigation

A logged-in user gets one `<nav>` (`crates/tuvi-web/src/features/shell/`).
The prototype calls it `.tabbar`.

- Below `lg` (1024px) it is a bottom tab bar, fixed and as wide as the phone
  column (480px). Pages keep room for it with `pb-tabbar`. Sticky and fixed
  parts at the bottom add `var(--spacing-tabbar)` to their offset.
- From `lg` it is a 250px rail on the left (`lg:pl-rail`) with the brand on
  top. The page header then hides, so the brand shows once.
- Five tabs, in order: Bản mệnh, Vận hạn, Hỏi AI, Nâng cao (the 12-cung
  chart), Hồ sơ. The icons are lucide icons.
- The current tab has the accent text in the bar and the `pill-primary` fill
  in the rail.
- The chart tabs open the chart in the URL, else the newest chart. Without a
  chart they are locked: an `<a>` without `href`, at 40% opacity. A screen
  that is not ported yet shows a small "Sắp có" label.

## An sao screen

The home page for a logged-in user. It casts a new chart
(`crates/tuvi-web/src/features/an_sao/`).

- Order on a phone: title, the preview, the input card, the main button, then
  the saved charts. From 768px: title and preview on the left, the input card
  and the button on the right.
- The input card (`glass`): name, then three scroll-snap wheels (day, month,
  year 1900–2099) over one accent band, then AM/PM, the birth clock and gender.
- AM/PM and gender are segments (`.segment`): the picked one has the accent
  gradient and `bg-0` text.
- The birth clock (`.birth-clock`) has 12 hours, 12 at the top. A tap on the
  ring picks an hour, a drag left or right winds the hand one hour per 22px,
  and arrow keys step one hour. The center shows the hour, the canh giờ and
  its range. The hand always turns the short way.
- The preview (`.reward`) sleeps until the user picks an hour: a scattered
  sky in `muted`. Then the page accent takes the color of the chart, and the
  caption says "Chòm sao … đã thức". Each chính tinh of cung Mệnh shows its
  constellation (`.chom-*`): a faint ghost portrait (24% opacity, radial
  mask) under a star pattern in the accent. A wave draws the lines every
  4.2 s, and each star pulses on a 4.5 s cycle. Song tinh show two smaller
  figures side by side. Vô chính diệu keeps the sky and the neutral accent.
  Under reduced motion, the lines and stars stay still and lit.
- The main button opens the confirm dialog (`.confirm-card`, a native
  `<dialog>`). "Không nhắc lại" skips it next time (in `localStorage`).
- Then the casting loader covers the page for 2.6 s: orbit rings, the crane
  video (`mix-blend-mode: screen`) and three rotating lines. Under reduced
  motion, the crane stays on its first frame.
- A missing hour or gender shows a toast, and the clock pulses.
- Every control is a native radio, so the form works without JavaScript. The
  clock hand and the picked styles then come from CSS `:has()`.

## Chart

- A 4×4 grid. The 12 cung go around the edge, read clockwise from Tị at the top
  left. The center 2×2 shows the name and birth data.
- Each cung shows: role (top left), Địa Chi (top right), chính tinh in the
  accent color ("Vô chính diệu" when empty), the first two phụ tinh and "+N",
  coral flags (Cung Thân, Tuần, Triệt), and on the bottom row the Tràng Sinh
  stage (left) and the Đại hạn ages (right).
- Cung Mệnh has an accent border and a light accent fill.
- A tap on a cung opens its popup: Thiên Can, Địa Chi and Âm/Dương, the
  Tràng Sinh stage, Đại hạn and Tiểu hạn, all chính tinh with Tứ Hóa (in the
  order Lộc, Quyền, Khoa, Kỵ), and all phụ tinh. Each star has a dot in the
  color of its element; stars without a known element use `muted`.
- Popups use the HTML `popover` attribute, so the browser handles Esc, a tap
  outside and focus. Do not add a script for this.

## Chat

- Thiên Hạc's messages: `glass` bubble on the left, a small "THIÊN HẠC" label
  in the accent color. The user's messages: `pill-primary` bubble on the right.
- The composer sticks to the bottom: one round field and a round send button.
- Answers are markdown. `strong` text uses the accent color.

## Rules

- Mobile first. Check every page at 390px wide and at 1280px wide.
- Honor `prefers-reduced-motion` (`styles.css` handles it once for all pages).
- Scrollbars are hidden, but every region still scrolls.
