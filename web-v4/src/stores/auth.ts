import { create } from "zustand";
import { persist } from "zustand/middleware";
import { authApi, api } from "../services";
import type { User, RegisterData } from "../types";

interface AuthState {
  // State
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;

  // Actions
  login: (email: string, password: string) => Promise<void>;
  register: (data: RegisterData) => Promise<void>;
  logout: () => Promise<void>;
  checkAuth: () => Promise<void>;
  clearError: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,

      login: async (email, password) => {
        set({ isLoading: true, error: null });
        try {
          const { user, access_token } = await authApi.login(email, password);
          api.setToken(access_token);
          set({ user, isAuthenticated: true, isLoading: false });
        } catch (error) {
          const message =
            error instanceof Error ? error.message : "Login failed";
          set({ error: message, isLoading: false });
          throw error;
        }
      },

      register: async (data) => {
        set({ isLoading: true, error: null });
        try {
          const { user, access_token } = await authApi.register(data);
          api.setToken(access_token);
          set({ user, isAuthenticated: true, isLoading: false });
        } catch (error) {
          const message =
            error instanceof Error ? error.message : "Registration failed";
          set({ error: message, isLoading: false });
          throw error;
        }
      },

      logout: async () => {
        set({ isLoading: true });
        try {
          await authApi.logout();
        } catch {
          // Ignore logout errors
        } finally {
          api.clearToken();
          set({
            user: null,
            isAuthenticated: false,
            isLoading: false,
            error: null,
          });
        }
      },

      checkAuth: async () => {
        try {
          const user = await authApi.getMe();
          set({ user, isAuthenticated: true });
        } catch {
          set({ user: null, isAuthenticated: false });
        }
      },

      clearError: () => set({ error: null }),
    }),
    {
      name: "auth-storage",
      partialize: (state) => ({
        user: state.user,
        isAuthenticated: state.isAuthenticated,
      }),
    },
  ),
);
