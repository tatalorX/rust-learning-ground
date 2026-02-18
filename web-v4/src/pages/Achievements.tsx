import { motion } from "framer-motion";
import { useAchievementsStore } from "../stores/achievements";
import { useEngagementStore } from "../stores/engagement";
import { Award, Lock, Trophy, Flame, Target, Star } from "lucide-react";

const categoryIcons = {
  milestone: Trophy,
  streak: Flame,
  exercise: Target,
  social: Star,
  special: Award,
};

const categoryColors = {
  milestone: "from-yellow-500 to-amber-500",
  streak: "from-orange-500 to-red-500",
  exercise: "from-primary to-blue-500",
  social: "from-purple-500 to-pink-500",
  special: "from-emerald-500 to-teal-500",
};

export default function AchievementsPage() {
  const { achievements, exercisesCompleted, getProgress } =
    useAchievementsStore();
  const engagementStore = useEngagementStore();

  const unlockedCount = achievements.filter((a) => a.unlockedAt).length;
  const totalCount = achievements.length;

  const achievementsByCategory = achievements.reduce(
    (acc, achievement) => {
      if (!acc[achievement.category]) {
        acc[achievement.category] = [];
      }
      acc[achievement.category].push(achievement);
      return acc;
    },
    {} as Record<string, typeof achievements>,
  );

  return (
    <div className="space-y-8">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-3xl font-display font-bold text-text-primary flex items-center gap-3">
            <Award className="w-8 h-8 text-primary" />
            Achievements
          </h1>
          <p className="text-text-secondary mt-1">
            {unlockedCount} of {totalCount} achievements unlocked
          </p>
        </div>

        <div className="flex items-center gap-4 p-4 rounded-2xl bg-surface border border-border-subtle">
          <div className="text-center">
            <p className="text-2xl font-bold text-text-primary">
              {exercisesCompleted}
            </p>
            <p className="text-xs text-text-secondary">Exercises Done</p>
          </div>
          <div className="w-px h-10 bg-border-subtle" />
          <div className="text-center">
            <p className="text-2xl font-bold text-text-primary">
              {engagementStore.streak}
            </p>
            <p className="text-xs text-text-secondary">Day Streak</p>
          </div>
          <div className="w-px h-10 bg-border-subtle" />
          <div className="text-center">
            <p className="text-2xl font-bold text-text-primary">
              {engagementStore.getLevel()}
            </p>
            <p className="text-xs text-text-secondary">Current Level</p>
          </div>
        </div>
      </div>

      {/* Progress Bar */}
      <div className="p-6 rounded-2xl bg-gradient-to-br from-primary/10 to-purple-500/10 border border-primary/20">
        <div className="flex items-center justify-between mb-3">
          <span className="text-text-primary font-medium">
            Overall Progress
          </span>
          <span className="text-primary font-bold">
            {Math.round((unlockedCount / totalCount) * 100)}%
          </span>
        </div>
        <div className="h-3 rounded-full bg-surface overflow-hidden">
          <motion.div
            initial={{ width: 0 }}
            animate={{ width: `${(unlockedCount / totalCount) * 100}%` }}
            transition={{ duration: 1, ease: "easeOut" }}
            className="h-full rounded-full bg-gradient-to-r from-primary to-purple-500"
          />
        </div>
      </div>

      {/* Achievements by Category */}
      <div className="space-y-8">
        {Object.entries(achievementsByCategory).map(
          ([category, categoryAchievements]) => {
            const Icon = categoryIcons[category as keyof typeof categoryIcons];
            const colorClass =
              categoryColors[category as keyof typeof categoryColors];

            return (
              <div key={category}>
                <div className="flex items-center gap-3 mb-4">
                  <div
                    className={`w-10 h-10 rounded-xl bg-gradient-to-br ${colorClass} flex items-center justify-center`}
                  >
                    <Icon className="w-5 h-5 text-white" />
                  </div>
                  <h2 className="text-xl font-bold text-text-primary capitalize">
                    {category.replace("_", " ")} Achievements
                  </h2>
                  <span className="text-text-muted text-sm">
                    {categoryAchievements.filter((a) => a.unlockedAt).length} /{" "}
                    {categoryAchievements.length}
                  </span>
                </div>

                <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">
                  {categoryAchievements.map((achievement, index) => (
                    <motion.div
                      key={achievement.id}
                      initial={{ opacity: 0, y: 20 }}
                      animate={{ opacity: 1, y: 0 }}
                      transition={{ delay: index * 0.05 }}
                      className={`p-5 rounded-2xl border transition-all ${
                        achievement.unlockedAt
                          ? "bg-surface border-border-subtle hover:border-primary/30"
                          : "bg-surface/50 border-border-subtle/50 opacity-60"
                      }`}
                    >
                      <div className="flex items-start gap-4">
                        <div
                          className={`w-14 h-14 rounded-2xl flex items-center justify-center text-3xl ${
                            achievement.unlockedAt
                              ? "bg-gradient-to-br " + colorClass
                              : "bg-surface-hover"
                          }`}
                        >
                          {achievement.unlockedAt ? (
                            achievement.icon
                          ) : (
                            <Lock className="w-6 h-6 text-text-muted" />
                          )}
                        </div>

                        <div className="flex-1 min-w-0">
                          <h3 className="font-semibold text-text-primary mb-1">
                            {achievement.name}
                          </h3>
                          <p className="text-sm text-text-secondary mb-2">
                            {achievement.description}
                          </p>

                          {!achievement.unlockedAt && (
                            <div className="space-y-1">
                              <div className="flex items-center justify-between text-xs">
                                <span className="text-text-muted">
                                  Progress
                                </span>
                                <span className="text-text-secondary">
                                  {Math.round(getProgress(achievement.id))}%
                                </span>
                              </div>
                              <div className="h-1.5 rounded-full bg-surface-hover overflow-hidden">
                                <div
                                  className="h-full rounded-full bg-primary/50"
                                  style={{
                                    width: `${getProgress(achievement.id)}%`,
                                  }}
                                />
                              </div>
                            </div>
                          )}

                          {achievement.unlockedAt && (
                            <p className="text-xs text-success">
                              Unlocked on{" "}
                              {new Date(
                                achievement.unlockedAt,
                              ).toLocaleDateString()}
                            </p>
                          )}
                        </div>
                      </div>
                    </motion.div>
                  ))}
                </div>
              </div>
            );
          },
        )}
      </div>
    </div>
  );
}
