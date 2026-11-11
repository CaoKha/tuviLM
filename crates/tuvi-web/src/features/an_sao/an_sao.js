// The An sao screen: the parts that Topcoat runtime expressions cannot do.
//
// - Wheels: when a scroll settles, check the radio in the middle band.
// - Clock: drag left or right to wind the hand, tap the ring to jump.
// - Submit: the confirm dialog, then the casting loader, then the real POST.
//
// Every pick goes through a native `change` event. The Topcoat handlers on
// the radios then update the signals, the readout and the preview shard.
// Without this file the radios still work by tap, and the form still posts.

const ROW_PX = 46;
const SETTLE_MS = 120;
const DRAG_STEP_PX = 22;
const DRAG_START_PX = 6;
const MIN_CASTING_MS = 2600;
const TOAST_MS = 2600;
const MUTE_KEY = "tuvi.muteBirthConfirm";

const form = document.querySelector("[data-an-sao]");
if (form) setUp(form);

/** Checks `input` and tells the page, like a tap would. */
function choose(input) {
  if (!input || input.checked) return;
  input.checked = true;
  input.dispatchEvent(new Event("change", { bubbles: true }));
}

function setUp(form) {
  form.querySelectorAll("[data-wheel]").forEach(setUpWheel);
  const clock = form.querySelector("[data-clock]");
  setUpClock(clock);
  setUpSubmit(form, clock);
}

function setUpWheel(wheel) {
  const options = [...wheel.querySelectorAll("input[type=radio]")];
  const centre = (input, behavior) => {
    const top = options.indexOf(input) * ROW_PX;
    if (Math.abs(wheel.scrollTop - top) > 1) wheel.scrollTo({ top, behavior });
  };
  centre(options.find((option) => option.checked), "instant");

  let timer;
  wheel.addEventListener(
    "scroll",
    () => {
      clearTimeout(timer);
      timer = setTimeout(() => {
        const index = Math.round(wheel.scrollTop / ROW_PX);
        choose(options[Math.max(0, Math.min(options.length - 1, index))]);
      }, SETTLE_MS);
    },
    { passive: true },
  );
  // A tap or an arrow key checks a radio: bring it to the middle band.
  wheel.addEventListener("change", (event) => centre(event.target, "smooth"));
}

function setUpClock(clock) {
  // Face order: 12 at the top, then 1 to 11 clockwise.
  const hours = [...clock.querySelectorAll("input[name=gio]")];
  const current = () => hours.findIndex((hour) => hour.checked);
  const wrap = (position) => ((position % 12) + 12) % 12;

  // Wind along the shortest arc and keep the total, so the hand never spins
  // back the long way from 11 to 12.
  let angle = 0;
  const turn = (position) => {
    angle += ((((position * 30 - angle) % 360) + 540) % 360) - 180;
    clock.style.setProperty("--hand-angle", `${angle}deg`);
  };
  if (current() >= 0) turn(current());
  clock.addEventListener("change", (event) => {
    const position = hours.indexOf(event.target);
    if (position >= 0) turn(position);
  });

  /** The hour under the pointer, or null in the readout. */
  const positionAt = (event) => {
    const rect = clock.getBoundingClientRect();
    const x = event.clientX - rect.left - rect.width / 2;
    const y = event.clientY - rect.top - rect.height / 2;
    if (Math.hypot(x, y) < rect.width * 0.24) return null;
    return wrap(Math.round((Math.atan2(x, -y) * 6) / Math.PI));
  };

  let drag = null;
  clock.addEventListener("pointerdown", (event) => {
    if (event.pointerType === "mouse" && event.button !== 0) return;
    drag = {
      id: event.pointerId,
      x: event.clientX,
      y: event.clientY,
      start: Math.max(0, current()),
      moved: false,
    };
    clock.setPointerCapture(event.pointerId);
  });
  clock.addEventListener("pointermove", (event) => {
    if (!drag || event.pointerId !== drag.id) return;
    const dx = event.clientX - drag.x;
    if (!drag.moved && Math.hypot(dx, event.clientY - drag.y) <= DRAG_START_PX) return;
    drag.moved = true;
    clock.classList.add("dragging");
    choose(hours[wrap(drag.start + Math.round(dx / DRAG_STEP_PX))]);
  });
  const end = (event) => {
    if (!drag || event.pointerId !== drag.id) return;
    if (!drag.moved && event.type === "pointerup") {
      const position = positionAt(event);
      if (position !== null) {
        choose(hours[position]);
        hours[position].focus({ preventScroll: true });
      }
    }
    drag = null;
    clock.classList.remove("dragging");
  };
  clock.addEventListener("pointerup", end);
  clock.addEventListener("pointercancel", end);
  // The pointer handlers own taps. A keyboard click (detail 0) still works.
  clock.addEventListener(
    "click",
    (event) => {
      if (event.detail > 0) event.preventDefault();
    },
    true,
  );
}

function setUpSubmit(form, clock) {
  const dialog = form.querySelector("[data-confirm]");
  const mute = dialog.querySelector("[data-mute]");
  const casting = document.querySelector("[data-casting]");
  const crane = casting.querySelector("[data-crane]");
  const toast = document.querySelector("[data-toast]");
  const gender = form.querySelector("[data-gender]");
  let toastTimer;
  let cast = false;

  const say = (text) => {
    toast.textContent = text;
    toast.dataset.show = "";
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => delete toast.dataset.show, TOAST_MS);
  };
  const muted = () => {
    try {
      return localStorage.getItem(MUTE_KEY) === "1";
    } catch {
      return false;
    }
  };

  // Missing hour or gender: a toast and a pulse, not the browser bubble on a
  // hidden radio. The name field keeps the browser bubble.
  let first = null;
  form.addEventListener(
    "invalid",
    (event) => {
      const field = event.target;
      if (first === null) {
        first = field;
        setTimeout(() => (first = null));
      }
      if (field.name !== "gio" && field.name !== "gender") return;
      event.preventDefault();
      if (field !== first) return;
      if (field.name === "gio") {
        say("Chọn giờ sinh để an sao chính xác nhé ✦");
        clock.classList.remove("need");
        void clock.offsetWidth;
        clock.classList.add("need");
        clock.scrollIntoView({ behavior: "smooth", block: "center" });
      } else {
        say("Cho Thiên Hạc biết giới tính của bạn nhé ✦");
        gender.scrollIntoView({ behavior: "smooth", block: "center" });
      }
    },
    true,
  );

  const realDate = () => {
    const value = (name) => Number(form.elements[name].value);
    const [day, month, year] = [value("day"), value("month"), value("year")];
    const date = new Date(Date.UTC(year, month - 1, day));
    return date.getUTCDate() === day && date.getUTCMonth() === month - 1;
  };

  form.addEventListener("submit", (event) => {
    event.preventDefault();
    if (cast) return;
    if (!realDate()) {
      say("Ngày sinh này không có trên lịch, bạn xem lại nhé ✦");
      return;
    }
    if (!event.submitter?.hasAttribute("data-cast") && !muted()) {
      dialog.showModal();
      return;
    }
    if (mute.checked) {
      try {
        localStorage.setItem(MUTE_KEY, "1");
      } catch {
        // Private mode: ask again next time.
      }
    }
    cast = true;
    if (dialog.open) dialog.close();
    casting.hidden = false;
    // Under reduced motion the crane stays still on its first frame.
    if (!matchMedia("(prefers-reduced-motion: reduce)").matches) crane.play().catch(() => {});
    // The interlude has a floor, from the design. The POST then runs the
    // server checks and redirects to the chart.
    setTimeout(() => form.submit(), MIN_CASTING_MS);
  });
  dialog.querySelector("[data-cancel]").addEventListener("click", () => dialog.close());

  // Back from the chart page: the browser may restore this page as it was.
  window.addEventListener("pageshow", (event) => {
    if (!event.persisted) return;
    cast = false;
    casting.hidden = true;
    crane.pause();
  });
}
