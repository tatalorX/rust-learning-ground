import { useAuthStore } from "../stores/auth";
import { useEngagementStore } from "../stores/engagement";
import {
  User,
  Trophy,
  Flame,
  Star,
  Target,
  Calendar,
  TrendingUp,
  Award,
  Zap,
  ChevronRight,
} from "lucide-react";

export default function ProfilePage() {
  const { user } = useAuthStore();
  const engagementStore = useEngagementStore();

  const currentLevel = engagementStore.getLevel();
  const xpForNextLevel = Math.pow(currentLevel, 2) * 100;
  const xpProgress = ((engagementStore.xp % 100) / 100) * 100;

  const stats = [
    {
      label: "Total XP",
      value: engagementStore.xp.toLocaleString(),
      icon: Zap,
      color: "text-primary",
    },
    {
      label: "Level",
      value: currentLevel,
      icon: Trophy,
      color: "text-yellow-500",
    },
    {
      label: "Streak",
      value: `${engagementStore.streak} days`,
      icon: Flame,
      color: "text-orange-500",
    },
    {
      label: "Focus Sessions",
      value: engagementStore.focusSessionsCompleted,
      icon: Target,
      color: "text-success",
    },
  ];

  const recentActivity = [
    {
      type: "solved",
      exercise: "Exercise 001",
      xp: "+10 XP",
      time: "2 hours ago",
    },
    { type: "achievement", exercise: "First Steps", xp: "", time: "1 day ago" },
    {
      type: "solved",
      exercise: "Exercise 025",
      xp: "+20 XP",
      time: "2 days ago",
    },
    {
      type: "streak",
      exercise: "7 day streak!",
      xp: "+50 XP",
      time: "3 days ago",
    },
  ];

  const achievements = [
    {
      name: "First Steps",
      description: "Solve your first exercise",
      unlocked: true,
      icon: "👣",
    },
    {
      name: "Week Warrior",
      description: "7 day streak",
      unlocked: true,
      icon: "🔥",
    },
    {
      name: "Rustacean",
      description: "Solve 10 exercises",
      unlocked: true,
      icon: "🦀",
    },
    {
      name: "Master",
      description: "Solve 100 exercises",
      unlocked: false,
      icon: "⭐",
    },
  ];

  return (
    <div className="space-y-8">
      <div className="flex flex-col sm:flex-row items-start sm:items-center gap-6">
        <div className="w-24 h-24 rounded-2xl bg-gradient-to-br from-primary to-purple-500 flex items-center justify-center text-white text-3xl font-bold shadow-glow">
          {user?.username?.charAt(0).toUpperCase() || "U"}
        </div>
        <div className="flex-1">
          <h1 className="text-2xl font-display font-bold text-text-primary">
            {user?.displayName || user?.username || "User"}
          </h1>
          <p className="text-text-secondary">@{user?.username}</p>
          <div className="flex items-center gap-4 mt-3">
            <span className="flex items-center gap-1.5 px-3 py-1 rounded-full bg-gradient-to-r from-primary/20 to-purple-500/20 border border-primary/30">
              <Trophy className="w-4 h-4 text-yellow-500" />
              <span className="text-sm font-medium text-primary">
                Level {currentLevel}
              </span>
            </span>
            <span className="flex items-center gap-1.5 text-text-secondary text-sm">
              <Zap className="w-4 h-4" />
              {engagementStore.xp.toLocaleString()} XP
            </span>
            <span className="flex items-center gap-1.5 text-text-secondary text-sm">
              <Flame className="w-4 h-4 text-orange-500" />
              {engagementStore.streak} day streak
            </span>
          </div>
        </div>
        <button className="px-4 py-2 rounded-xl bg-surface border border-border-subtle text-text-secondary font-medium hover:text-text-primary hover:border-primary/30 transition-all">
          Edit Profile
        </button>
      </div>

      <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {stats.map((stat, i) => (
          <div
            key={i}
            className="p-6 rounded-2xl bg-surface border border-border-subtle hover:border-primary/30 transition-all"
          >
            <div className={`mb-3 ${stat.color}`}>
              <stat.icon className="w-6 h-6" />
            </div>
            <p className="text-2xl font-display font-bold text-text-primary">
              {stat.value}
            </p>
            <p className="text-text-secondary text-sm">{stat.label}</p>
          </div>
        ))}
      </div>

      <div className="grid lg:grid-cols-2 gap-8">
        <div className="space-y-6">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-display font-bold text-text-primary">
              Recent Activity
            </h2>
            <button className="text-primary text-sm font-medium hover:text-primary-hover transition-colors">
              View All
            </button>
          </div>

          <div className="p-6 rounded-2xl bg-surface border border-border-subtle space-y-4">
            {recentActivity.map((activity, i) => (
              <div key={i} className="flex items-center gap-4">
                <div
                  className={`w-10 h-10 rounded-xl flex items-center justify-center ${
                    activity.type === "solved"
                      ? "bg-success/10 text-success"
                      : activity.type === "achievement"
                        ? "bg-yellow-500/10 text-yellow-500"
                        : "bg-orange-500/10 text-orange-500"
                  }`}
                >
                  {activity.type === "solved" ? (
                    <Target className="w-5 h-5" />
                  ) : activity.type === "achievement" ? (
                    <Award className="w-5 h-5" />
                  ) : (
                    <Flame className="w-5 h-5" />
                  )}
                </div>
                <div className="flex-1">
                  <p className="text-text-primary font-medium">
                    {activity.exercise}
                  </p>
                  <p className="text-text-muted text-sm">{activity.time}</p>
                </div>
                {activity.xp && (
                  <span className="text-success font-medium">
                    {activity.xp}
                  </span>
                )}
              </div>
            ))}
          </div>
        </div>

        <div className="space-y-6">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-display font-bold text-text-primary">
              Achievements
            </h2>
            <span className="text-text-secondary text-sm">3/10 unlocked</span>
          </div>

          <div className="grid grid-cols-2 gap-4">
            {achievements.map((achievement, i) => (
              <div
                key={i}
                className={`p-4 rounded-xl border transition-all ${
                  achievement.unlocked
                    ? "bg-surface border-border-subtle hover:border-primary/30"
                    : "bg-surface/50 border-border-subtle opacity-50"
                }`}
              >
                <div className="text-3xl mb-3">{achievement.icon}</div>
                <h3 className="font-medium text-text-primary">
                  {achievement.name}
                </h3>
                <p className="text-text-muted text-sm mt-1">
                  {achievement.description}
                </p>
                {achievement.unlocked && (
                  <div className="flex items-center gap-1 mt-2 text-success text-xs">
                    <Award className="w-3 h-3" />
                    Unlocked
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      </div>

      <div className="p-6 rounded-2xl bg-gradient-to-br from-primary/10 to-purple-500/10 border border-primary/20">
        <div className="flex items-center gap-4 mb-4">
          <div className="w-12 h-12 rounded-xl bg-primary/20 flex items-center justify-center">
            <TrendingUp className="w-6 h-6 text-primary" />
          </div>
          <div>
            <h3 className="font-semibold text-text-primary">Your Progress</h3>
            <p className="text-text-secondary text-sm">
              Keep learning to advance!
            </p>
          </div>
        </div>

        <div className="space-y-3">
          <div className="flex items-center justify-between text-sm">
            <span className="text-text-secondary">
              Level {currentLevel} → Level {currentLevel + 1}
            </span>
            <span className="text-text-muted">
              {engagementStore.xp} / {xpForNextLevel} XP
            </span>
          </div>
          <div className="h-3 rounded-full bg-surface overflow-hidden">
            <div
              className="h-full rounded-full bg-gradient-to-r from-primary to-orange-400 transition-all"
              style={{ width: `${xpProgress}%` }}
            />
          </div>
          <p className="text-text-muted text-xs">
            {xpForNextLevel - engagementStore.xp} XP needed for Level{" "}
            {currentLevel + 1}
          </p>
        </div>
      </div>
    </div>
  );
}
