import { create } from "zustand";
import { exerciseApi } from "../services";
import type { ConsoleEntry, RunResult, SubmitResult } from "../types";

interface EditorState {
  code: string;
  originalCode: string;
  isRunning: boolean;
  isSubmitting: boolean;
  consoleEntries: ConsoleEntry[];
  lastRunResult: RunResult | null;
  lastSubmitResult: SubmitResult | null;

  setCode: (code: string) => void;
  resetCode: () => void;
  runCode: (exerciseId: number) => Promise<void>;
  submitCode: (exerciseId: number) => Promise<void>;
  clearConsole: () => void;
  addConsoleEntry: (entry: Omit<ConsoleEntry, "id" | "timestamp">) => void;
}

const createConsoleEntry = (
  entry: Omit<ConsoleEntry, "id" | "timestamp">,
): ConsoleEntry => ({
  ...entry,
  id: crypto.randomUUID(),
  timestamp: Date.now(),
});

const DEFAULT_CODE = `fn main() {
    println!("Hello, Rust!");
}`;

export const useEditorStore = create<EditorState>((set, get) => ({
  code: DEFAULT_CODE,
  originalCode: DEFAULT_CODE,
  isRunning: false,
  isSubmitting: false,
  consoleEntries: [],
  lastRunResult: null,
  lastSubmitResult: null,

  setCode: (code: string) => {
    set({ code });
  },

  resetCode: () => {
    set({ code: get().originalCode });
    get().clearConsole();
  },

  runCode: async (exerciseId: number) => {
    const { code } = get();

    get().addConsoleEntry({
      type: "info",
      message: "Running code...",
    });

    set({ isRunning: true });

    try {
      const result = await exerciseApi.run(exerciseId, code);

      set({ lastRunResult: result });

      if (result.output) {
        get().addConsoleEntry({
          type: "output",
          message: result.output,
        });
      }

      if (result.success) {
        get().addConsoleEntry({
          type: "success",
          message: `✓ Execution successful (${result.executionTimeMs}ms)`,
        });
      } else if (result.error) {
        get().addConsoleEntry({
          type: "error",
          message: result.error,
        });
      }
    } catch (error) {
      get().addConsoleEntry({
        type: "error",
        message: error instanceof Error ? error.message : "Failed to run code",
      });
    } finally {
      set({ isRunning: false });
    }
  },

  submitCode: async (exerciseId: number) => {
    const { code } = get();

    get().addConsoleEntry({
      type: "info",
      message: "Submitting solution...",
    });

    set({ isSubmitting: true });

    try {
      const result = await exerciseApi.submit(exerciseId, code);

      set({ lastSubmitResult: result });

      if (result.success) {
        get().addConsoleEntry({
          type: "success",
          message: `✓ Correct! +${result.xpEarned} XP`,
        });
      } else if (result.error) {
        get().addConsoleEntry({
          type: "error",
          message: result.error,
        });
      }
    } catch (error) {
      get().addConsoleEntry({
        type: "error",
        message: error instanceof Error ? error.message : "Failed to submit",
      });
    } finally {
      set({ isSubmitting: false });
    }
  },

  clearConsole: () => {
    set({ consoleEntries: [], lastRunResult: null, lastSubmitResult: null });
  },

  addConsoleEntry: (entry) => {
    set((state) => ({
      consoleEntries: [...state.consoleEntries, createConsoleEntry(entry)],
    }));
  },
}));
