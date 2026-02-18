import { api } from "./api";
import type {
  Exercise,
  ExerciseDetail,
  RunResult,
  SubmitResult,
  PaginatedResponse,
  Category,
  Difficulty,
} from "../types";

export interface ExerciseListParams {
  cursor?: string;
  limit?: number;
  difficulty?: number;
  category?: string;
  search?: string;
}

export const exerciseApi = {
  list: async (
    params?: ExerciseListParams,
  ): Promise<PaginatedResponse<Exercise>> => {
    const queryParams = new URLSearchParams();

    if (params?.cursor) queryParams.set("cursor", params.cursor);
    if (params?.limit) queryParams.set("limit", params.limit.toString());
    if (params?.difficulty)
      queryParams.set("difficulty", params.difficulty.toString());
    if (params?.category) queryParams.set("category", params.category);
    if (params?.search) queryParams.set("search", params.search);

    const query = queryParams.toString();
    return api.get<PaginatedResponse<Exercise>>(
      `/exercises${query ? `?${query}` : ""}`,
    );
  },

  get: async (id: number): Promise<ExerciseDetail> => {
    return api.get<ExerciseDetail>(`/exercises/${id}`);
  },

  run: async (id: number, code: string): Promise<RunResult> => {
    return api.post<RunResult>(`/exercises/${id}/run`, { code });
  },

  submit: async (id: number, code: string): Promise<SubmitResult> => {
    return api.post<SubmitResult>(`/exercises/${id}/submit`, { code });
  },

  getCategories: async (): Promise<{
    categories: Category[];
    total: number;
  }> => {
    return api.get<{ categories: Category[]; total: number }>(
      "/exercises/categories",
    );
  },

  getDifficulties: async (): Promise<{ difficulties: Difficulty[] }> => {
    return api.get<{ difficulties: Difficulty[] }>("/exercises/difficulties");
  },
};
