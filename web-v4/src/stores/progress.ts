import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface ExerciseProgress {
  exerciseId: number;
  status: "not_started" | "in_progress" | "completed";
  attempts: number;
  completedAt?: string;
  bestExecutionTimeMs?: number;
  xpEarned: number;
  savedCode?: string;
  lastAttemptAt?: string;
}

interface ProgressState {
  exercises: Record<number, ExerciseProgress>;
  totalCompleted: number;
  totalAttempts: number;
  totalXPEarned: number;
  streakDays: number;
  lastActiveDate: string | null;

  // Actions
  startExercise: (exerciseId: number) => void;
  completeExercise: (
    exerciseId: number,
    xpEarned: number,
    executionTimeMs?: number,
  ) => void;
  saveCode: (exerciseId: number, code: string) => void;
  getProgress: (exerciseId: number) => ExerciseProgress | undefined;
  getCompletedExerciseIds: () => number[];
  getRecentExercises: (limit?: number) => ExerciseProgress[];
  getStats: () => {
    totalCompleted: number;
    totalAttempts: number;
    averageExecutionTime: number;
    completionRate: number;
  };
}

export const useProgressStore = create<ProgressState>()(
  persist(
    (set, get) => ({
      exercises: {},
      totalCompleted: 0,
      totalAttempts: 0,
      totalXPEarned: 0,
      streakDays: 0,
      lastActiveDate: null,

      startExercise: (exerciseId) => {
        set((state) => {
          const existing = state.exercises[exerciseId];
          if (existing) {
            return {
              exercises: {
                ...state.exercises,
                [exerciseId]: {
                  ...existing,
                  status:
                    existing.status === "completed"
                      ? "completed"
                      : "in_progress",
                  lastAttemptAt: new Date().toISOString(),
                },
              },
            };
          }
          return {
            exercises: {
              ...state.exercises,
              [exerciseId]: {
                exerciseId,
                status: "in_progress",
                attempts: 0,
                xpEarned: 0,
                lastAttemptAt: new Date().toISOString(),
              },
            },
          };
        });
      },

      completeExercise: (exerciseId, xpEarned, executionTimeMs) => {
        set((state) => {
          const existing = state.exercises[exerciseId];
          const wasCompleted = existing?.status === "completed";

          const newProgress: ExerciseProgress = {
            exerciseId,
            status: "completed",
            attempts: (existing?.attempts || 0) + 1,
            completedAt: new Date().toISOString(),
            xpEarned: wasCompleted ? existing?.xpEarned || 0 : xpEarned,
            bestExecutionTimeMs:
              existing?.bestExecutionTimeMs && executionTimeMs
                ? Math.min(existing.bestExecutionTimeMs, executionTimeMs)
                : executionTimeMs,
            savedCode: existing?.savedCode,
            lastAttemptAt: new Date().toISOString(),
          };

          const newCompletedCount = wasCompleted
            ? state.totalCompleted
            : state.totalCompleted + 1;

          return {
            exercises: {
              ...state.exercises,
              [exerciseId]: newProgress,
            },
            totalCompleted: newCompletedCount,
            totalAttempts: state.totalAttempts + 1,
            totalXPEarned: wasCompleted
              ? state.totalXPEarned
              : state.totalXPEarned + xpEarned,
          };
        });
      },

      saveCode: (exerciseId, code) => {
        set((state) => ({
          exercises: {
            ...state.exercises,
            [exerciseId]: {
              ...(state.exercises[exerciseId] || {
                exerciseId,
                status: "in_progress",
                attempts: 0,
                xpEarned: 0,
              }),
              savedCode: code,
              lastAttemptAt: new Date().toISOString(),
            },
          },
        }));
      },

      getProgress: (exerciseId) => {
        return get().exercises[exerciseId];
      },

      getCompletedExerciseIds: () => {
        return Object.values(get().exercises)
          .filter((p) => p.status === "completed")
          .map((p) => p.exerciseId);
      },

      getRecentExercises: (limit = 5) => {
        return Object.values(get().exercises)
          .sort(
            (a, b) =>
              new Date(b.lastAttemptAt || 0).getTime() -
              new Date(a.lastAttemptAt || 0).getTime(),
          )
          .slice(0, limit);
      },

      getStats: () => {
        const exercises = Object.values(get().exercises);
        const completed = exercises.filter((e) => e.status === "completed");
        const totalAttempts = exercises.reduce((sum, e) => sum + e.attempts, 0);
        const avgTime =
          completed.length > 0
            ? completed.reduce(
                (sum, e) => sum + (e.bestExecutionTimeMs || 0),
                0,
              ) / completed.length
            : 0;

        return {
          totalCompleted: completed.length,
          totalAttempts,
          averageExecutionTime: Math.round(avgTime),
          completionRate:
            exercises.length > 0
              ? Math.round((completed.length / exercises.length) * 100)
              : 0,
        };
      },
    }),
    {
      name: "progress-storage",
    },
  ),
);
