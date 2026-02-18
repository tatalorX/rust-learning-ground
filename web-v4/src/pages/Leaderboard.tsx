import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import {
  Trophy,
  Medal,
  Award,
  Crown,
  TrendingUp,
  Users,
  Target,
} from "lucide-react";
import { useAuthStore } from "../stores/auth";
import { useEngagementStore } from "../stores/engagement";

interface LeaderboardEntry {
  rank: number;
  username: string;
  xp: number;
  exercisesCompleted: number;
  currentStreak: number;
  isCurrentUser?: boolean;
}

export default function LeaderboardPage() {
  const { user, isAuthenticated } = useAuthStore();
  const engagementStore = useEngagementStore();
  const [timeRange, setTimeRange] = useState<"week" | "month" | "all">("all");
  const [entries, setEntries] = useState<LeaderboardEntry[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Simulate loading leaderboard data
    // In production, this would fetch from backend
    const mockData: LeaderboardEntry[] = [
      {
        rank: 1,
        username: "rustacean_master",
        xp: 15250,
        exercisesCompleted: 142,
        currentStreak: 45,
      },
      {
        rank: 2,
        username: "code_warrior",
        xp: 12800,
        exercisesCompleted: 120,
        currentStreak: 32,
      },
      {
        rank: 3,
        username: "cargo_cultist",
        xp: 11500,
        exercisesCompleted: 108,
        currentStreak: 28,
      },
      {
        rank: 4,
        username: "borrow_checker",
        xp: 9800,
        exercisesCompleted: 95,
        currentStreak: 21,
      },
      {
        rank: 5,
        username: "memory_safe",
        xp: 8750,
        exercisesCompleted: 82,
        currentStreak: 18,
      },
      {
        rank: 6,
        username: "ferris_fan",
        xp: 7200,
        exercisesCompleted: 68,
        currentStreak: 15,
      },
      {
        rank: 7,
        username: "trait_bound",
        xp: 6500,
        exercisesCompleted: 61,
        currentStreak: 12,
      },
      {
        rank: 8,
        username: "lifetime_learner",
        xp: 5800,
        exercisesCompleted: 54,
        currentStreak: 10,
      },
      {
        rank: 9,
        username: "generic_dev",
        xp: 4200,
        exercisesCompleted: 40,
        currentStreak: 7,
      },
      {
        rank: 10,
        username: user?.username || "you",
        xp: engagementStore.xp,
        exercisesCompleted: Math.floor(engagementStore.xp / 100),
        currentStreak: engagementStore.streak,
        isCurrentUser: true,
      },
    ];

    setTimeout(() => {
      setEntries(
        mockData
          .sort((a, b) => b.xp - a.xp)
          .map((e, i) => ({ ...e, rank: i + 1 })),
      );
      setIsLoading(false);
    }, 500);
  }, [timeRange, user, engagementStore.xp, engagementStore.streak]);

  const getRankIcon = (rank: number) => {
    if (rank === 1) return <Crown className="w-6 h-6 text-yellow-500" />;
    if (rank === 2) return <Medal className="w-6 h-6 text-gray-400" />;
    if (rank === 3) return <Medal className="w-6 h-6 text-orange-600" />;
    return (
      <span className="w-6 h-6 flex items-center justify-center font-bold text-text-muted">
        {rank}
      </span>
    );
  };

  const getRankColor = (rank: number) => {
    if (rank === 1)
      return "from-yellow-500/20 to-yellow-600/20 border-yellow-500/50";
    if (rank === 2) return "from-gray-400/20 to-gray-500/20 border-gray-400/50";
    if (rank === 3)
      return "from-orange-600/20 to-orange-700/20 border-orange-600/50";
    if (rank <= 10) return "from-primary/10 to-primary/20 border-primary/30";
    return "border-border-subtle";
  };

  return (
    <div className="space-y-8">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-3xl font-display font-bold text-text-primary flex items-center gap-3">
            <Trophy className="w-8 h-8 text-primary" />
            Leaderboard
          </h1>
          <p className="text-text-secondary mt-1">
            See how you rank against other Rust learners
          </p>
        </div>

        <div className="flex items-center gap-2 p-1 rounded-xl bg-surface border border-border-subtle">
          {(["week", "month", "all"] as const).map((range) => (
            <button
              key={range}
              onClick={() => setTimeRange(range)}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-all capitalize ${
                timeRange === range
                  ? "bg-primary text-white"
                  : "text-text-secondary hover:text-text-primary"
              }`}
            >
              {range === "all" ? "All Time" : range}
            </button>
          ))}
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid sm:grid-cols-3 gap-4">
        <div className="p-6 rounded-2xl bg-surface border border-border-subtle">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center">
              <Users className="w-5 h-5 text-primary" />
            </div>
            <span className="text-text-secondary">Total Learners</span>
          </div>
          <p className="text-3xl font-display font-bold text-text-primary">
            2,547
          </p>
        </div>

        <div className="p-6 rounded-2xl bg-surface border border-border-subtle">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-10 h-10 rounded-xl bg-success/10 flex items-center justify-center">
              <Target className="w-5 h-5 text-success" />
            </div>
            <span className="text-text-secondary">Exercises Solved</span>
          </div>
          <p className="text-3xl font-display font-bold text-text-primary">
            45,892
          </p>
        </div>

        <div className="p-6 rounded-2xl bg-surface border border-border-subtle">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-10 h-10 rounded-xl bg-orange-500/10 flex items-center justify-center">
              <TrendingUp className="w-5 h-5 text-orange-500" />
            </div>
            <span className="text-text-secondary">Your Rank</span>
          </div>
          <p className="text-3xl font-display font-bold text-text-primary">
            {entries.find((e) => e.isCurrentUser)?.rank || "--"}
          </p>
        </div>
      </div>

      {/* Leaderboard List */}
      <div className="bg-surface border border-border-subtle rounded-2xl overflow-hidden">
        <div className="grid grid-cols-12 gap-4 p-4 border-b border-border-subtle text-sm font-medium text-text-secondary">
          <div className="col-span-1">#</div>
          <div className="col-span-4">User</div>
          <div className="col-span-3 text-right">XP</div>
          <div className="col-span-2 text-right">Solved</div>
          <div className="col-span-2 text-right">Streak</div>
        </div>

        {isLoading ? (
          <div className="p-8 text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto"></div>
            <p className="text-text-secondary mt-4">Loading leaderboard...</p>
          </div>
        ) : (
          <div className="divide-y divide-border-subtle">
            {entries.map((entry) => (
              <motion.div
                key={entry.username}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: entry.rank * 0.05 }}
                className={`grid grid-cols-12 gap-4 p-4 items-center hover:bg-surface-hover transition-colors ${
                  entry.isCurrentUser ? "bg-primary/5" : ""
                }`}
              >
                <div className="col-span-1 flex justify-center">
                  {getRankIcon(entry.rank)}
                </div>
                <div className="col-span-4 flex items-center gap-3">
                  <div
                    className={`w-10 h-10 rounded-full bg-gradient-to-br ${getRankColor(
                      entry.rank,
                    )} flex items-center justify-center text-white font-bold text-sm border-2`}
                  >
                    {entry.username.charAt(0).toUpperCase()}
                  </div>
                  <div>
                    <p className="font-medium text-text-primary">
                      {entry.username}
                      {entry.isCurrentUser && (
                        <span className="ml-2 text-xs text-primary">(You)</span>
                      )}
                    </p>
                    {entry.rank <= 3 && (
                      <p className="text-xs text-text-muted">
                        {entry.rank === 1
                          ? "Champion"
                          : entry.rank === 2
                            ? "Runner-up"
                            : "Third Place"}
                      </p>
                    )}
                  </div>
                </div>
                <div className="col-span-3 text-right">
                  <span className="font-bold text-text-primary">
                    {entry.xp.toLocaleString()}
                  </span>
                  <span className="text-text-muted text-sm ml-1">XP</span>
                </div>
                <div className="col-span-2 text-right text-text-secondary">
                  {entry.exercisesCompleted}
                </div>
                <div className="col-span-2 text-right">
                  <span
                    className={`inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs ${
                      entry.currentStreak >= 7
                        ? "bg-orange-500/20 text-orange-400"
                        : entry.currentStreak >= 3
                          ? "bg-yellow-500/20 text-yellow-400"
                          : "bg-surface-hover text-text-secondary"
                    }`}
                  >
                    {entry.currentStreak >= 7 && <Award className="w-3 h-3" />}
                    {entry.currentStreak}d
                  </span>
                </div>
              </motion.div>
            ))}
          </div>
        )}
      </div>

      {!isAuthenticated && (
        <div className="p-6 rounded-2xl bg-gradient-to-br from-primary/10 to-purple-500/10 border border-primary/20 text-center">
          <Award className="w-12 h-12 text-primary mx-auto mb-4" />
          <h3 className="text-xl font-bold text-text-primary mb-2">
            Join the Leaderboard
          </h3>
          <p className="text-text-secondary mb-4">
            Sign in to track your progress and compete with other learners
          </p>
        </div>
      )}
    </div>
  );
}
