"use client";

import { type QueryClient, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { request } from "./client";
import { queryKeys } from "./queryKeys";
import {
  type AuthUser,
  authUserResponseSchema,
  emptyResponseSchema,
  type MeResponse,
  meResponseSchema,
} from "./schemas";

/**
 * Login against the FastAPI backend (`/api/v1/auth/*`).
 *
 * The backend keeps the session in an httpOnly cookie, so the browser never
 * sees a token. Charts and chats belong to whoever the cookies say: after a
 * login, register or logout, every owner-scoped query is refetched.
 */

export const GOOGLE_SIGN_IN_PATH = "/api/v1/auth/google/start";

export function useSession() {
  return useQuery({
    queryKey: queryKeys.auth.me(),
    queryFn: ({ signal }) => request("/api/v1/auth/me", { schema: meResponseSchema, signal }),
    staleTime: 60_000,
  });
}

export interface LoginInput {
  email: string;
  password: string;
}

export interface RegisterInput extends LoginInput {
  displayName: string;
}

export function useLogin() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: LoginInput) =>
      request("/api/v1/auth/login", {
        method: "POST",
        body: input,
        schema: authUserResponseSchema,
      }),
    onSuccess: ({ user }) => onIdentityChange(queryClient, user),
  });
}

export function useRegister() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ displayName, email, password }: RegisterInput) =>
      request("/api/v1/auth/register", {
        method: "POST",
        body: { email, password, display_name: displayName },
        schema: authUserResponseSchema,
      }),
    onSuccess: ({ user }) => onIdentityChange(queryClient, user),
  });
}

export function useLogout() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () =>
      request("/api/v1/auth/logout", { method: "POST", schema: emptyResponseSchema }),
    onSuccess: () => onIdentityChange(queryClient, null),
  });
}

/** Full-page navigation: Google redirects back to the backend callback. */
export function signInWithGoogle(): void {
  // An API route that redirects to Google, not a Next page: the router cannot handle it.
  // eslint-disable-next-line @next/next/no-location-assign-relative-destination
  window.location.assign(GOOGLE_SIGN_IN_PATH);
}

/** A display name for the header: the chosen name, else the email's local part. */
export function userLabel(user: AuthUser): string {
  return user.display_name?.trim() || user.email.split("@")[0] || user.email;
}

function onIdentityChange(queryClient: QueryClient, user: AuthUser | null): Promise<void> {
  queryClient.setQueryData<MeResponse>(queryKeys.auth.me(), (previous) => ({
    user,
    google_enabled: previous?.google_enabled ?? false,
  }));
  return Promise.all([
    queryClient.invalidateQueries({ queryKey: queryKeys.chartProfiles.all() }),
    queryClient.invalidateQueries({ queryKey: queryKeys.sessions.all() }),
  ]).then(() => undefined);
}
