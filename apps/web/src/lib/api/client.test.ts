import { afterEach, describe, expect, it, vi } from "vitest";

import { isApiError } from "@/lib/http/errors";

import fixture from "./__fixtures__/build-laso.json";
import { newIdempotencyKey, request } from "./client";
import { buildLasoResponseSchema } from "./schemas";

function jsonResponse(body: unknown, status = 200) {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}

/** Unwrap the ApiError the client threw, failing the test if it threw anything else. */
async function errorFrom(promise: Promise<unknown>) {
  try {
    await promise;
  } catch (caught) {
    if (isApiError(caught)) return caught.error;
    throw caught;
  }
  throw new Error("expected the request to reject");
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe("request", () => {
  it("parses a real backend response", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(jsonResponse(fixture)));

    const result = await request("/api/v1/laso/build", {
      method: "POST",
      body: { calendar: "solar", year: 2009, month: 4, day: 4, hour: 5, gender: "M" },
      schema: buildLasoResponseSchema,
    });

    expect(Object.keys(result.cung_by_position)).toHaveLength(12);
    expect(result.ban_menh_name).toBe("Tích Lịch Hỏa");
  });

  it("reports schema drift as a parse error carrying the issues", async () => {
    const drifted = { ...fixture, cung_by_position: { Tý: { position: 1 } } };
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(jsonResponse(drifted)));

    const error = await errorFrom(
      request("/api/v1/laso/build", { method: "POST", schema: buildLasoResponseSchema }),
    );

    expect(error.kind).toBe("parse");
    if (error.kind === "parse") expect(error.issues.length).toBeGreaterThan(0);
  });

  it("reports an unreachable backend as a network error", async () => {
    vi.stubGlobal("fetch", vi.fn().mockRejectedValue(new TypeError("Failed to fetch")));

    const error = await errorFrom(
      request("/api/v1/health", { schema: buildLasoResponseSchema }),
    );

    expect(error).toEqual({ kind: "network" });
  });

  it("preserves status and body on an HTTP error", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(jsonResponse({ detail: "day is out of range for month" }, 422)),
    );

    const error = await errorFrom(
      request("/api/v1/laso/build", { method: "POST", schema: buildLasoResponseSchema }),
    );

    expect(error).toEqual({
      kind: "http",
      status: 422,
      body: { detail: "day is out of range for month" },
    });
  });

  it("lets an abort propagate rather than disguising it as a network failure", async () => {
    const abort = new DOMException("The operation was aborted.", "AbortError");
    vi.stubGlobal("fetch", vi.fn().mockRejectedValue(abort));

    await expect(
      request("/api/v1/health", { schema: buildLasoResponseSchema }),
    ).rejects.toBe(abort);
  });
});

describe("headers", () => {
  it("calls the same-origin path and attaches the idempotency key", async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse(fixture));
    vi.stubGlobal("fetch", fetchMock);

    const key = newIdempotencyKey();
    await request("/api/v1/laso/build", {
      method: "POST",
      body: {},
      schema: buildLasoResponseSchema,
      idempotencyKey: key,
    });

    expect(fetchMock).toHaveBeenCalledTimes(1);
    const [url, init] = fetchMock.mock.calls[0]!;
    // Relative: next.config.ts forwards /api/* to FastAPI, and cookies go along.
    expect(url).toBe("/api/v1/laso/build");
    expect(init.headers["Idempotency-Key"]).toBe(key);
    expect(init.headers["Content-Type"]).toBe("application/json");
    expect(init.headers["X-Anonymous-Owner-Id"]).toBeUndefined();
  });

  it("omits the idempotency header when it is not requested", async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse(fixture));
    vi.stubGlobal("fetch", fetchMock);

    await request("/api/v1/laso/build", { schema: buildLasoResponseSchema });

    const headers = fetchMock.mock.calls[0]![1].headers;
    expect(headers["Idempotency-Key"]).toBeUndefined();
  });
});

describe("newIdempotencyKey", () => {
  it("returns a non-empty key", () => {
    expect(newIdempotencyKey().length).toBeGreaterThan(0);
  });

  it("falls back when crypto.randomUUID is unavailable (plain HTTP over LAN)", () => {
    const original = globalThis.crypto;
    vi.stubGlobal("crypto", { getRandomValues: () => undefined, randomUUID: undefined });
    try {
      const key = newIdempotencyKey();
      expect(key.startsWith("id-")).toBe(true);
      expect(key.length).toBeGreaterThan(3);
    } finally {
      vi.stubGlobal("crypto", original);
    }
  });
});
