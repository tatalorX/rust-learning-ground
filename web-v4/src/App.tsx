import { useState, useEffect } from "react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useAuthStore } from "./stores/auth";
import { useAchievementsStore } from "./stores/achievements";
import {
  useNotificationsStore,
  createAchievementNotification,
  createLevelUpNotification,
} from "./stores/notifications";
import { useEngagementStore } from "./stores/engagement";

import Layout from "./components/layout/Layout";
import HomePage from "./pages/Home";
import ExercisesPage from "./pages/Exercises";
import ExercisePage from "./pages/Exercise";
import ProfilePage from "./pages/Profile";
import AuthPage from "./pages/Auth";
import EngagementDemoPage from "./pages/EngagementDemo";
import BrainrotDemoPage from "./pages/BrainrotDemo";
import AudioDemoPage from "./pages/AudioDemo";
import SettingsPage from "./pages/Settings";
import LeaderboardPage from "./pages/Leaderboard";
import AchievementsPage from "./pages/Achievements";
import RustBookPage from "./pages/RustBook";
import DocumentationPage from "./pages/Documentation";
import EventsPage from "./pages/Events";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60,
      retry: 1,
    },
  },
});

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, isLoading } = useAuthStore();

  if (isLoading) {
    return (
      <div className="min-h-screen bg-canvas flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary"></div>
      </div>
    );
  }

  if (!isAuthenticated) {
    return <Navigate to="/auth" replace />;
  }

  return <>{children}</>;
}

function App() {
  const { checkAuth, isAuthenticated } = useAuthStore();
  const engagementStore = useEngagementStore();
  const achievementsStore = useAchievementsStore();
  const notificationsStore = useNotificationsStore();
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    checkAuth().finally(() => setInitialized(true));
  }, [checkAuth]);

  // Check achievements when stats change
  useEffect(() => {
    if (!initialized) return;

    const previousCount = achievementsStore.achievements.filter(
      (a) => a.unlockedAt,
    ).length;

    achievementsStore.checkAchievements({
      exercisesCompleted: achievementsStore.exercisesCompleted,
      streak: engagementStore.streak,
      level: engagementStore.getLevel(),
      xp: engagementStore.xp,
    });

    // Check for newly unlocked achievements
    const newCount = achievementsStore.achievements.filter(
      (a) => a.unlockedAt,
    ).length;
    if (newCount > previousCount) {
      const newlyUnlocked = achievementsStore.recentlyUnlocked;
      if (newlyUnlocked) {
        notificationsStore.addNotification(
          createAchievementNotification(newlyUnlocked.name, newlyUnlocked.icon),
        );
        achievementsStore.clearRecentUnlock();
      }
    }
  }, [
    initialized,
    engagementStore.xp,
    engagementStore.streak,
    achievementsStore.exercisesCompleted,
  ]);

  if (!initialized) {
    return (
      <div className="min-h-screen bg-canvas flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-16 w-16 border-4 border-primary border-t-transparent mx-auto mb-4"></div>
          <p className="text-text-secondary">Loading Rust Learning Ground...</p>
        </div>
      </div>
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/auth" element={<AuthPage />} />

          <Route path="/" element={<Layout />}>
            <Route index element={<HomePage />} />
            <Route path="exercises" element={<ExercisesPage />} />
            <Route path="exercises/:id" element={<ExercisePage />} />
            <Route path="demo" element={<EngagementDemoPage />} />
            <Route path="brainrot" element={<BrainrotDemoPage />} />
            <Route path="audio" element={<AudioDemoPage />} />
            <Route path="settings" element={<SettingsPage />} />
            <Route path="leaderboard" element={<LeaderboardPage />} />
            <Route path="achievements" element={<AchievementsPage />} />
            <Route path="book" element={<RustBookPage />} />
            <Route path="docs" element={<DocumentationPage />} />
            <Route path="events" element={<EventsPage />} />
            <Route
              path="profile"
              element={
                <ProtectedRoute>
                  <ProfilePage />
                </ProtectedRoute>
              }
            />
          </Route>

          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  );
}

export default App;
