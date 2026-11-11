import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";

import { ApiErrorException } from "@/lib/http/errors";

import { AuthScreen } from "./auth-screen";

const authMocks = vi.hoisted(() => ({
  login: vi.fn(),
  register: vi.fn(),
  signInWithGoogle: vi.fn(),
  session: { user: null, google_enabled: false } as
    | { user: null; google_enabled: boolean }
    | undefined,
}));
const routerMocks = vi.hoisted(() => ({ replace: vi.fn(), refresh: vi.fn() }));
const toastMocks = vi.hoisted(() => ({ showToast: vi.fn() }));

vi.mock("next/navigation", () => ({
  useRouter: () => routerMocks,
}));

vi.mock("@/lib/api/auth", () => ({
  useLogin: () => ({ mutateAsync: authMocks.login }),
  useRegister: () => ({ mutateAsync: authMocks.register }),
  useSession: () => ({ data: authMocks.session }),
  signInWithGoogle: authMocks.signInWithGoogle,
}));

vi.mock("@/lib/toast", () => toastMocks);

beforeEach(() => {
  vi.clearAllMocks();
  authMocks.session = { user: null, google_enabled: false };
});

function fillLogin(email: string, password: string) {
  fireEvent.change(screen.getByLabelText(/email/i), { target: { value: email } });
  fireEvent.change(screen.getByLabelText(/^mật khẩu$/i), { target: { value: password } });
}

describe("AuthScreen registration validation", () => {
  it("uses the Zod message instead of native browser validation for an empty form", () => {
    render(<AuthScreen mode="register" />);

    const submitButton = screen.getByRole("button", { name: "Tạo tài khoản" });
    const form = submitButton.closest("form");

    expect(form?.noValidate).toBe(true);
    fireEvent.submit(form!);

    expect(toastMocks.showToast).toHaveBeenCalledWith("Vui lòng nhập tên hiển thị.");
    expect(authMocks.register).not.toHaveBeenCalled();
  });
});

describe("AuthScreen login", () => {
  it("logs in through the API and goes home", async () => {
    authMocks.login.mockResolvedValue({ user: { id: "1", email: "a@b.vn", display_name: null } });
    render(<AuthScreen mode="login" />);

    fillLogin("a@b.vn", "secret-123");
    fireEvent.submit(screen.getByRole("button", { name: /đăng nhập/i, hidden: false }).closest("form")!);

    await waitFor(() => expect(routerMocks.replace).toHaveBeenCalledWith("/"));
    expect(authMocks.login).toHaveBeenCalledWith({ email: "a@b.vn", password: "secret-123" });
  });

  it("explains a wrong password", async () => {
    authMocks.login.mockRejectedValue(
      new ApiErrorException({ kind: "http", status: 401, body: { detail: "x" } }),
    );
    render(<AuthScreen mode="login" />);

    fillLogin("a@b.vn", "wrong-pass");
    fireEvent.submit(screen.getByRole("button", { name: /đăng nhập/i }).closest("form")!);

    await waitFor(() =>
      expect(toastMocks.showToast).toHaveBeenCalledWith("Email hoặc mật khẩu chưa đúng."),
    );
    expect(routerMocks.replace).not.toHaveBeenCalled();
  });
});

describe("AuthScreen Google sign-in", () => {
  it("says so when Google is not configured on the server", () => {
    render(<AuthScreen mode="login" />);

    fireEvent.click(screen.getByRole("button", { name: /google/i }));

    expect(toastMocks.showToast).toHaveBeenCalledWith(
      "Google OAuth chưa được cấu hình trên máy chủ.",
    );
    expect(authMocks.signInWithGoogle).not.toHaveBeenCalled();
  });

  it("starts the redirect when Google is configured", () => {
    authMocks.session = { user: null, google_enabled: true };
    render(<AuthScreen mode="login" />);

    fireEvent.click(screen.getByRole("button", { name: /google/i }));

    expect(authMocks.signInWithGoogle).toHaveBeenCalledOnce();
  });

  it("reports a failed Google sign-in after the redirect back", () => {
    render(<AuthScreen mode="login" googleError />);

    expect(toastMocks.showToast).toHaveBeenCalledWith(
      "Đăng nhập Google không thành công. Vui lòng thử lại.",
    );
  });
});
