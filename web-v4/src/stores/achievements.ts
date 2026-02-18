import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface Achievement {
  id: string;
  name: string;
  description: string;
  icon: string;
  unlockedAt: string | null;
  category: "milestone" | "streak" | "exercise" | "social" | "special";
  requirement: {
    type:
      | "exercises_completed"
      | "streak_days"
      | "xp_earned"
      | "level_reached"
      | "special";
    value: number;
  };
}

const DEFAULT_ACHIEVEMENTS: Achievement[] = [
  {
    id: "first_steps",
    name: "First Steps",
    description: "Complete your first exercise",
    icon: "👣",
    unlockedAt: null,
    category: "milestone",
    requirement: { type: "exercises_completed", value: 1 },
  },
  {
    id: "getting_warm",
    name: "Getting Warm",
    description: "Complete 10 exercises",
    icon: "🔥",
    unlockedAt: null,
    category: "milestone",
    requirement: { type: "exercises_completed", value: 10 },
  },
  {
    id: "problem_solver",
    name: "Problem Solver",
    description: "Complete 50 exercises",
    icon: "🧩",
    unlockedAt: null,
    category: "milestone",
    requirement: { type: "exercises_completed", value: 50 },
  },
  {
    id: "code_master",
    name: "Code Master",
    description: "Complete 100 exercises",
    icon: "👑",
    unlockedAt: null,
    category: "milestone",
    requirement: { type: "exercises_completed", value: 100 },
  },
  {
    id: "week_warrior",
    name: "Week Warrior",
    description: "Maintain a 7-day streak",
    icon: "📅",
    unlockedAt: null,
    category: "streak",
    requirement: { type: "streak_days", value: 7 },
  },
  {
    id: "month_master",
    name: "Month Master",
    description: "Maintain a 30-day streak",
    icon: "🗓️",
    unlockedAt: null,
    category: "streak",
    requirement: { type: "streak_days", value: 30 },
  },
  {
    id: "centurion",
    name: "Centurion",
    description: "Maintain a 100-day streak",
    icon: "💯",
    unlockedAt: null,
    category: "streak",
    requirement: { type: "streak_days", value: 100 },
  },
  {
    id: "novice",
    name: "Novice Rustacean",
    description: "Reach level 5",
    icon: "🦀",
    unlockedAt: null,
    category: "exercise",
    requirement: { type: "level_reached", value: 5 },
  },
  {
    id: "apprentice",
    name: "Apprentice",
    description: "Reach level 10",
    icon: "⚡",
    unlockedAt: null,
    category: "exercise",
    requirement: { type: "level_reached", value: 10 },
  },
  {
    id: "expert",
    name: "Rust Expert",
    description: "Reach level 25",
    icon: "🎯",
    unlockedAt: null,
    category: "exercise",
    requirement: { type: "level_reached", value: 25 },
  },
  {
    id: "master",
    name: "Rust Master",
    description: "Reach level 50",
    icon: "🏆",
    unlockedAt: null,
    category: "exercise",
    requirement: { type: "level_reached", value: 50 },
  },
  {
    id: "legend",
    name: "Rust Legend",
    description: "Reach level 100",
    icon: "👑",
    unlockedAt: null,
    category: "exercise",
    requirement: { type: "level_reached", value: 100 },
  },
  {
    id: "night_owl",
    name: "Night Owl",
    description: "Solve an exercise after midnight",
    icon: "🦉",
    unlockedAt: null,
    category: "special",
    requirement: { type: "special", value: 1 },
  },
  {
    id: "early_bird",
    name: "Early Bird",
    description: "Solve an exercise before 6 AM",
    icon: "🐦",
    unlockedAt: null,
    category: "special",
    requirement: { type: "special", value: 1 },
  },
];

interface AchievementsState {
  achievements: Achievement[];
  exercisesCompleted: number;
  recentlyUnlocked: Achievement | null;

  // Actions
  checkAchievements: (stats: {
    exercisesCompleted: number;
    streak: number;
    level: number;
    xp: number;
  }) => void;
  completeExercise: () => void;
  clearRecentUnlock: () => void;
  getProgress: (achievementId: string) => number;
}

export const useAchievementsStore = create<AchievementsState>()(
  persist(
    (set, get) => ({
      achievements: DEFAULT_ACHIEVEMENTS,
      exercisesCompleted: 0,
      recentlyUnlocked: null,

      checkAchievements: (stats) => {
        const { achievements } = get();
        let newUnlock: Achievement | null = null;

        const updatedAchievements = achievements.map((achievement) => {
          if (achievement.unlockedAt) return achievement;

          let unlocked = false;
          switch (achievement.requirement.type) {
            case "exercises_completed":
              unlocked =
                stats.exercisesCompleted >= achievement.requirement.value;
              break;
            case "streak_days":
              unlocked = stats.streak >= achievement.requirement.value;
              break;
            case "level_reached":
              unlocked = stats.level >= achievement.requirement.value;
              break;
            case "xp_earned":
              unlocked = stats.xp >= achievement.requirement.value;
              break;
          }

          if (unlocked) {
            const updated = {
              ...achievement,
              unlockedAt: new Date().toISOString(),
            };
            if (!newUnlock) newUnlock = updated;
            return updated;
          }
          return achievement;
        });

        set({
          achievements: updatedAchievements,
          recentlyUnlocked: newUnlock,
        });
      },

      completeExercise: () => {
        set((state) => ({ exercisesCompleted: state.exercisesCompleted + 1 }));
      },

      clearRecentUnlock: () => set({ recentlyUnlocked: null }),

      getProgress: (achievementId) => {
        const achievement = get().achievements.find(
          (a) => a.id === achievementId,
        );
        if (!achievement || achievement.unlockedAt) return 100;

        const { exercisesCompleted } = get();
        const { requirement } = achievement;

        switch (requirement.type) {
          case "exercises_completed":
            return Math.min(
              100,
              (exercisesCompleted / requirement.value) * 100,
            );
          default:
            return 0;
        }
      },
    }),
    {
      name: "achievements-storage",
    },
  ),
);
