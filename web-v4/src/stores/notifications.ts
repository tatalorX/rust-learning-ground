import { create } from "zustand";
import { persist } from "zustand/middleware";

export type NotificationType =
  | "achievement"
  | "streak"
  | "level_up"
  | "system"
  | "exercise";

export interface Notification {
  id: string;
  type: NotificationType;
  title: string;
  message: string;
  read: boolean;
  createdAt: string;
  data?: Record<string, unknown>;
}

interface NotificationsState {
  notifications: Notification[];
  unreadCount: number;

  // Actions
  addNotification: (
    notification: Omit<Notification, "id" | "createdAt" | "read">,
  ) => void;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  removeNotification: (id: string) => void;
  clearAll: () => void;
}

export const useNotificationsStore = create<NotificationsState>()(
  persist(
    (set, get) => ({
      notifications: [],
      unreadCount: 0,

      addNotification: (notification) => {
        const newNotification: Notification = {
          ...notification,
          id: crypto.randomUUID(),
          createdAt: new Date().toISOString(),
          read: false,
        };

        set((state) => ({
          notifications: [newNotification, ...state.notifications].slice(0, 50), // Keep only last 50
          unreadCount: state.unreadCount + 1,
        }));

        // Auto-remove notification after 5 seconds for toasts
        if (
          notification.type === "achievement" ||
          notification.type === "level_up"
        ) {
          setTimeout(() => {
            get().removeNotification(newNotification.id);
          }, 5000);
        }
      },

      markAsRead: (id) => {
        set((state) => ({
          notifications: state.notifications.map((n) =>
            n.id === id ? { ...n, read: true } : n,
          ),
          unreadCount: Math.max(0, state.unreadCount - 1),
        }));
      },

      markAllAsRead: () => {
        set((state) => ({
          notifications: state.notifications.map((n) => ({ ...n, read: true })),
          unreadCount: 0,
        }));
      },

      removeNotification: (id) => {
        set((state) => {
          const notification = state.notifications.find((n) => n.id === id);
          return {
            notifications: state.notifications.filter((n) => n.id !== id),
            unreadCount: notification?.read
              ? state.unreadCount
              : Math.max(0, state.unreadCount - 1),
          };
        });
      },

      clearAll: () => set({ notifications: [], unreadCount: 0 }),
    }),
    {
      name: "notifications-storage",
      partialize: (state) => ({
        notifications: state.notifications,
        unreadCount: state.unreadCount,
      }),
    },
  ),
);

// Helper to create common notifications
export const createAchievementNotification = (
  achievementName: string,
  icon: string,
) => ({
  type: "achievement" as const,
  title: "Achievement Unlocked!",
  message: `${icon} ${achievementName}`,
});

export const createStreakNotification = (streakDays: number) => ({
  type: "streak" as const,
  title: "Streak Milestone!",
  message: `You've maintained a ${streakDays}-day streak!`,
});

export const createLevelUpNotification = (level: number) => ({
  type: "level_up" as const,
  title: "Level Up!",
  message: `Congratulations! You reached level ${level}`,
});
