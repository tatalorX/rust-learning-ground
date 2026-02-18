import { api } from "./api";
import type { User, RegisterData, ApiAuthResponse } from "../types";

const AUTH_BASE = "/auth";

export const authApi = {
  login: async (
    email: string,
    password: string,
  ): Promise<{ user: User; access_token: string }> => {
    const response = await api.post<ApiAuthResponse>(AUTH_BASE + "/login", {
      email,
      password,
    });
    if (!response.success || !response.access_token || !response.user) {
      throw new Error(response.message || "Login failed");
    }
    return {
      user: {
        id: response.user.id,
        email: response.user.email,
        username: response.user.username,
        displayName: response.user.display_name,
        avatarUrl: response.user.github_avatar_url || undefined,
        createdAt: response.user.created_at,
      },
      access_token: response.access_token,
    };
  },

  register: async (
    data: RegisterData,
  ): Promise<{ user: User; access_token: string }> => {
    const response = await api.post<ApiAuthResponse>(
      AUTH_BASE + "/register",
      data,
    );
    if (!response.success || !response.access_token || !response.user) {
      throw new Error(response.message || "Registration failed");
    }
    return {
      user: {
        id: response.user.id,
        email: response.user.email,
        username: response.user.username,
        displayName: response.user.display_name,
        avatarUrl: response.user.github_avatar_url || undefined,
        createdAt: response.user.created_at,
      },
      access_token: response.access_token,
    };
  },

  logout: async (): Promise<void> => {
    return api.post(AUTH_BASE + "/logout");
  },

  refresh: async (): Promise<void> => {
    return api.post(AUTH_BASE + "/refresh");
  },

  getMe: async (): Promise<User> => {
    return api.get<User>("/me");
  },

  getProfile: async (): Promise<User> => {
    return api.get<User>("/../users/profile");
  },
};
