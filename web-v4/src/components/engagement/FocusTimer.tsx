import { useState, useEffect, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Play, Pause, RotateCcw, Coffee } from "lucide-react";

interface FocusTimerProps {
  enabled: boolean;
  onComplete?: () => void;
}

const POMODORO_TIME = 25 * 60; // 25 minutes
const BREAK_TIME = 5 * 60; // 5 minutes

export function FocusTimer({ enabled, onComplete }: FocusTimerProps) {
  const [isActive, setIsActive] = useState(false);
  const [timeLeft, setTimeLeft] = useState(POMODORO_TIME);
  const [isBreak, setIsBreak] = useState(false);
  const [sessions, setSessions] = useState(0);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  };

  useEffect(() => {
    let interval: ReturnType<typeof setInterval> | null = null;

    if (isActive && timeLeft > 0) {
      interval = setInterval(() => {
        setTimeLeft((prev) => prev - 1);
      }, 1000);
    } else if (timeLeft === 0 && isActive) {
      setIsActive(false);
      if (isBreak) {
        setIsBreak(false);
        setTimeLeft(POMODORO_TIME);
      } else {
        setIsBreak(true);
        setTimeLeft(BREAK_TIME);
        setSessions((prev) => prev + 1);
        onComplete?.();
      }
    }

    return () => {
      if (interval) clearInterval(interval);
    };
  }, [isActive, timeLeft, isBreak, onComplete]);

  const toggleTimer = () => setIsActive(!isActive);
  const resetTimer = () => {
    setIsActive(false);
    setIsBreak(false);
    setTimeLeft(POMODORO_TIME);
  };

  if (!enabled) return null;

  const progress = ((POMODORO_TIME - timeLeft) / POMODORO_TIME) * 100;

  return (
    <div className="bg-surface border border-border-subtle rounded-xl p-4 w-64">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-bold text-text-primary flex items-center gap-2">
          <Coffee className="w-5 h-5 text-primary" />
          Focus Time
        </h3>
        <span
          className={`text-xs px-2 py-1 rounded-full ${
            isBreak
              ? "bg-green-500/20 text-green-500"
              : "bg-primary/20 text-primary"
          }`}
        >
          {isBreak ? "Break" : "Focus"}
        </span>
      </div>

      <div className="relative mb-4">
        <svg className="w-40 h-40 mx-auto -rotate-90">
          <circle
            cx="80"
            cy="80"
            r="70"
            fill="none"
            stroke="currentColor"
            strokeWidth="8"
            className="text-surface-hover"
          />
          <motion.circle
            cx="80"
            cy="80"
            r="70"
            fill="none"
            stroke={isBreak ? "#10b981" : "#f74c00"}
            strokeWidth="8"
            strokeLinecap="round"
            strokeDasharray={440}
            initial={{ strokeDashoffset: 440 }}
            animate={{ strokeDashoffset: 440 - (440 * progress) / 100 }}
            transition={{ duration: 0.5 }}
          />
        </svg>
        <div className="absolute inset-0 flex items-center justify-center">
          <span className="text-3xl font-bold text-text-primary font-mono">
            {formatTime(timeLeft)}
          </span>
        </div>
      </div>

      <div className="flex items-center justify-center gap-2">
        <motion.button
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          onClick={toggleTimer}
          className="p-3 rounded-full bg-primary text-white hover:bg-primary-hover transition-colors"
        >
          {isActive ? (
            <Pause className="w-5 h-5" />
          ) : (
            <Play className="w-5 h-5" />
          )}
        </motion.button>
        <motion.button
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          onClick={resetTimer}
          className="p-3 rounded-full bg-surface-hover text-text-secondary hover:bg-border-subtle transition-colors"
        >
          <RotateCcw className="w-5 h-5" />
        </motion.button>
      </div>

      <div className="mt-4 text-center">
        <span className="text-text-muted text-sm">
          Sessions completed:{" "}
          <span className="text-primary font-bold">{sessions}</span>
        </span>
      </div>
    </div>
  );
}

export function MiniFocusTimer({ enabled }: { enabled: boolean }) {
  const [isOpen, setIsOpen] = useState(false);

  if (!enabled) return null;

  return (
    <>
      <motion.button
        whileHover={{ scale: 1.05 }}
        whileTap={{ scale: 0.95 }}
        onClick={() => setIsOpen(true)}
        className="fixed bottom-4 right-4 p-3 rounded-full bg-primary text-white shadow-lg shadow-primary/50 z-40"
      >
        <Coffee className="w-5 h-5" />
      </motion.button>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, scale: 0.9, y: 20 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.9, y: 20 }}
            className="fixed bottom-20 right-4 z-50"
          >
            <div className="relative">
              <button
                onClick={() => setIsOpen(false)}
                className="absolute -top-2 -right-2 p-1 rounded-full bg-surface-hover text-text-muted hover:text-text-primary z-10"
              >
                ✕
              </button>
              <FocusTimer enabled={true} />
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </>
  );
}
