import { motion } from "framer-motion";
import { Flame } from "lucide-react";

interface StreakFlameProps {
  streak: number;
  enabled: boolean;
}

export function StreakFlame({ streak, enabled }: StreakFlameProps) {
  if (!enabled || streak === 0) return null;

  const getFlameSize = () => {
    if (streak >= 30) return "w-8 h-8";
    if (streak >= 14) return "w-7 h-7";
    if (streak >= 7) return "w-6 h-6";
    return "w-5 h-5";
  };

  const getFlameIntensity = () => {
    if (streak >= 30) return 3;
    if (streak >= 14) return 2;
    if (streak >= 7) return 1.5;
    return 1;
  };

  return (
    <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-gradient-to-r from-orange-500/20 to-red-500/20 border border-orange-500/30">
      <motion.div
        animate={
          enabled
            ? {
                scale: [1, 1.2, 1],
                rotate: [0, 5, -5, 0],
              }
            : {}
        }
        transition={{
          duration: 0.5,
          repeat: Infinity,
          repeatType: "reverse",
        }}
      >
        <Flame
          className={`${getFlameSize()} text-orange-500`}
          style={{
            filter: `drop-shadow(0 0 ${getFlameIntensity() * 4}px rgba(255, 100, 0, 0.6))`,
          }}
        />
      </motion.div>
      <span className="text-orange-500 font-bold text-sm">
        {streak} day{streak > 1 ? "s" : ""}
      </span>
    </div>
  );
}

export function StreakCard({ streak }: { streak: number }) {
  const milestones = [7, 14, 30, 60, 100];
  const nextMilestone =
    milestones.find((m) => m > streak) || milestones[milestones.length - 1];
  const progress = Math.min((streak / nextMilestone) * 100, 100);

  return (
    <div className="bg-surface border border-border-subtle rounded-xl p-4">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-bold text-text-primary">Streak</h3>
        <Flame className="w-6 h-6 text-orange-500" />
      </div>

      <div className="text-4xl font-bold text-orange-500 mb-2">{streak}</div>
      <div className="text-text-secondary text-sm mb-4">days in a row</div>

      <div className="mb-2 flex justify-between text-xs text-text-muted">
        <span>0</span>
        <span>{nextMilestone}</span>
      </div>
      <div className="h-2 bg-surface-hover rounded-full overflow-hidden">
        <motion.div
          className="h-full bg-gradient-to-r from-orange-500 to-red-500"
          initial={{ width: 0 }}
          animate={{ width: `${progress}%` }}
          transition={{ duration: 0.5 }}
        />
      </div>

      <div className="mt-4 flex gap-2 flex-wrap">
        {milestones.map((m) => (
          <span
            key={m}
            className={`text-xs px-2 py-1 rounded-full ${
              streak >= m
                ? "bg-orange-500/20 text-orange-500"
                : "bg-surface-hover text-text-muted"
            }`}
          >
            {m}d
          </span>
        ))}
      </div>
    </div>
  );
}
