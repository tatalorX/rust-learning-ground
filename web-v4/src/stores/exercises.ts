import { create } from "zustand";
import type { Exercise, ExerciseDetail } from "../types";
import { exerciseApi } from "../services/exercises";

interface ExercisesState {
  exercises: Exercise[];
  currentExercise: ExerciseDetail | null;
  isLoadingList: boolean;
  isLoadingDetail: boolean;
  hasMore: boolean;
  nextCursor?: string;
  filters: {
    difficulty?: number;
    category?: string;
    search?: string;
  };

  fetchExercises: (reset?: boolean) => Promise<void>;
  fetchExercise: (id: number) => Promise<void>;
  setFilters: (filters: Partial<ExercisesState["filters"]>) => void;
  clearCurrent: () => void;
}

export const useExercisesStore = create<ExercisesState>((set, get) => ({
  exercises: [],
  currentExercise: null,
  isLoadingList: false,
  isLoadingDetail: false,
  hasMore: true,
  nextCursor: undefined,
  filters: {},

  fetchExercises: async (reset = false) => {
    const { filters, nextCursor } = get();
    if (reset) {
      set({ exercises: [], hasMore: true, nextCursor: undefined });
    }

    set({ isLoadingList: true });

    try {
      const response = await exerciseApi.list({
        ...filters,
        cursor: reset ? undefined : nextCursor,
        limit: 20,
      });

      set({
        exercises: reset
          ? response.data
          : [...get().exercises, ...response.data],
        hasMore: response.pagination.has_more,
        nextCursor: response.pagination.next_cursor?.toString(),
        isLoadingList: false,
      });
    } catch (error) {
      console.error("Failed to fetch exercises:", error);
      set({ isLoadingList: false });
    }
  },

  fetchExercise: async (id: number) => {
    set({ isLoadingDetail: true, currentExercise: null });

    try {
      const exercise = await exerciseApi.get(id);
      set({ currentExercise: exercise, isLoadingDetail: false });
    } catch (error) {
      console.error("Failed to fetch exercise:", error);
      set({ isLoadingDetail: false });
    }
  },

  setFilters: (newFilters) => {
    set((state) => ({
      filters: { ...state.filters, ...newFilters },
    }));
    get().fetchExercises(true);
  },

  clearCurrent: () => {
    set({ currentExercise: null });
  },
}));
