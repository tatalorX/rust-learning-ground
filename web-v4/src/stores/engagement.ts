import { create } from "zustand";
import { persist } from "zustand/middleware";

interface EngagementSettings {
  xpEnabled: boolean;
  streakEnabled: boolean;
  soundsEnabled: boolean;
  animationsEnabled: boolean;
  focusTimerEnabled: boolean;
  brainrotMode: boolean;
}

interface EngagementState extends EngagementSettings {
  xp: number;
  streak: number;
  lastActiveDate: string | null;
  focusSessionsCompleted: number;

  // Actions
  setXPSettings: (enabled: boolean) => void;
  setStreakSettings: (enabled: boolean) => void;
  setSoundsSettings: (enabled: boolean) => void;
  setAnimationsSettings: (enabled: boolean) => void;
  setFocusTimerSettings: (enabled: boolean) => void;
  setBrainrotMode: (enabled: boolean) => void;

  // Gamification actions
  addXP: (amount: number) => void;
  incrementStreak: () => void;
  resetStreak: () => void;
  completeFocusSession: () => void;
  getLevel: () => number;
}

const DEFAULT_SETTINGS: EngagementSettings = {
  xpEnabled: true,
  streakEnabled: true,
  soundsEnabled: false,
  animationsEnabled: true,
  focusTimerEnabled: true,
  brainrotMode: false,
};

export const useEngagementStore = create<EngagementState>()(
  persist(
    (set, get) => ({
      ...DEFAULT_SETTINGS,
      xp: 0,
      streak: 0,
      lastActiveDate: null,
      focusSessionsCompleted: 0,

      // Toggle actions
      setXPSettings: (enabled) => set({ xpEnabled: enabled }),
      setStreakSettings: (enabled) => set({ streakEnabled: enabled }),
      setSoundsSettings: (enabled) => set({ soundsEnabled: enabled }),
      setAnimationsSettings: (enabled) => set({ animationsEnabled: enabled }),
      setFocusTimerSettings: (enabled) => set({ focusTimerEnabled: enabled }),
      setBrainrotMode: (enabled) => set({ brainrotMode: enabled }),

      // Gamification
      addXP: (amount) => set((state) => ({ xp: state.xp + amount })),

      incrementStreak: () => {
        const today = new Date().toISOString().split("T")[0];
        const state = get();

        if (state.lastActiveDate !== today) {
          const yesterday = new Date();
          yesterday.setDate(yesterday.getDate() - 1);
          const yesterdayStr = yesterday.toISOString().split("T")[0];

          if (state.lastActiveDate === yesterdayStr) {
            set({ streak: state.streak + 1, lastActiveDate: today });
          } else if (state.lastActiveDate === null) {
            set({ streak: 1, lastActiveDate: today });
          } else {
            set({ streak: 1, lastActiveDate: today });
          }
        }
      },

      resetStreak: () => set({ streak: 0 }),

      completeFocusSession: () =>
        set((state) => ({
          focusSessionsCompleted: state.focusSessionsCompleted + 1,
        })),

      getLevel: () => {
        const xp = get().xp;
        return Math.floor(Math.sqrt(xp / 100)) + 1;
      },
    }),
    {
      name: "engagement-storage",
      partialize: (state) => ({
        xp: state.xp,
        streak: state.streak,
        lastActiveDate: state.lastActiveDate,
        focusSessionsCompleted: state.focusSessionsCompleted,
        xpEnabled: state.xpEnabled,
        streakEnabled: state.streakEnabled,
        soundsEnabled: state.soundsEnabled,
        animationsEnabled: state.animationsEnabled,
        focusTimerEnabled: state.focusTimerEnabled,
        brainrotMode: state.brainrotMode,
      }),
    },
  ),
);
