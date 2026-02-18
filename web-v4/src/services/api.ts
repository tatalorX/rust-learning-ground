import axios, { type AxiosInstance, type AxiosError } from "axios";
import type { ApiError } from "../types";

const API_BASE = import.meta.env.VITE_API_URL || "http://localhost:8000/api/v4";

export class ApiClientError extends Error {
  status: number;
  code?: string;

  constructor(message: string, status: number, code?: string) {
    super(message);
    this.name = "ApiClientError";
    this.status = status;
    this.code = code;
  }
}

class ApiClient {
  private client: AxiosInstance;
  private accessToken: string | null = null;
  private csrfToken: string | null = null;

  constructor(baseUrl: string) {
    this.client = axios.create({
      baseURL: baseUrl,
      headers: {
        Accept: "application/json",
      },
      withCredentials: true,
    });

    this.client.interceptors.request.use(async (config) => {
      if (this.accessToken && config.headers) {
        config.headers.Authorization = `Bearer ${this.accessToken}`;
      }
      
      // SECURITY FIX: Add CSRF token for state-changing methods (H-05)
      const stateChangingMethods = ["POST", "PUT", "PATCH", "DELETE"];
      if (stateChangingMethods.includes(config.method?.toUpperCase() || "")) {
        if (!this.csrfToken) {
          await this.fetchCsrfToken();
        }
        if (this.csrfToken && config.headers) {
          config.headers["X-CSRF-Token"] = this.csrfToken;
        }
      }
      
      return config;
    });

    this.client.interceptors.response.use(
      (response) => response,
      async (error: AxiosError<ApiError>) => {
        // Don't auto-redirect on 401 - let components handle auth failures
        // This prevents redirect loops during checkAuth() on startup
        return Promise.reject(error);
      },
    );
  }

  setToken(token: string) {
    this.accessToken = token;
  }

  clearToken() {
    this.accessToken = null;
  }

  // SECURITY FIX: Fetch CSRF token for state-changing requests (H-05)
  private async fetchCsrfToken(): Promise<void> {
    try {
      const response = await this.client.get("/csrf");
      this.csrfToken = response.data.csrf_token;
    } catch (error) {
      console.error("Failed to fetch CSRF token:", error);
      this.csrfToken = null;
    }
  }

  private async request<T>(
    method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
    endpoint: string,
    data?: unknown,
  ): Promise<T> {
    try {
      const response = await this.client.request<T>({
        method,
        url: endpoint,
        data,
      });
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        const apiError = error.response?.data;
        throw new ApiClientError(
          apiError?.message ||
            apiError?.error ||
            `HTTP ${error.response?.status}`,
          error.response?.status || 500,
          apiError?.code,
        );
      }
      throw new ApiClientError("Unknown error", 500);
    }
  }

  get<T>(endpoint: string): Promise<T> {
    return this.request<T>("GET", endpoint);
  }

  post<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>("POST", endpoint, data);
  }

  put<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>("PUT", endpoint, data);
  }

  patch<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>("PATCH", endpoint, data);
  }

  delete<T>(endpoint: string): Promise<T> {
    return this.request<T>("DELETE", endpoint);
  }
}

export const api = new ApiClient(API_BASE);

export function isApiError(error: unknown): error is ApiClientError {
  return error instanceof ApiClientError;
}
