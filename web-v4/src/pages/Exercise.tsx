import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import { useParams, Link } from "react-router-dom";
import { useExercisesStore } from "../stores/exercises";
import { useEditorStore } from "../stores/editor";
import { useEngagementStore } from "../stores/engagement";
import { useAchievementsStore } from "../stores/achievements";
import { useProgressStore } from "../stores/progress";
import {
  useNotificationsStore,
  createLevelUpNotification,
} from "../stores/notifications";
import { exerciseApi } from "../services/exercises";
import Editor from "@monaco-editor/react";
import {
  ArrowLeft,
  Play,
  RotateCcw,
  CheckCircle2,
  XCircle,
  Loader2,
  ChevronRight,
  Sparkles,
  Clock,
  Zap,
  Volume2,
} from "lucide-react";
import type { ExerciseDetail, ConsoleEntry } from "../types";
import { XPPopup } from "../components/engagement/XPPopup";
import { AudioGuide } from "../components/audio/AudioGuide";

const difficultyColors = {
  1: "from-green-500 to-emerald-500",
  2: "from-lime-500 to-yellow-500",
  3: "from-yellow-500 to-orange-500",
  4: "from-orange-500 to-red-500",
  5: "from-red-500 to-pink-500",
};

const difficultyLabels = {
  1: "Beginner",
  2: "Easy",
  3: "Medium",
  4: "Hard",
  5: "Expert",
};

export default function ExercisePage() {
  const { id } = useParams<{ id: string }>();
  const exerciseId = parseInt(id || "1");

  const { currentExercise, isLoadingDetail, fetchExercise, clearCurrent } =
    useExercisesStore();
  const {
    code,
    setCode,
    resetCode,
    runCode,
    submitCode,
    isRunning,
    isSubmitting,
    consoleEntries,
    clearConsole,
    lastSubmitResult,
  } = useEditorStore();
  const engagementStore = useEngagementStore();
  const achievementsStore = useAchievementsStore();
  const notificationsStore = useNotificationsStore();
  const progressStore = useProgressStore();

  const [activeTab, setActiveTab] = useState<
    "description" | "hints" | "solutions"
  >("description");
  const [showHints, setShowHints] = useState(false);
  const [showXPPopup, setShowXPPopup] = useState(false);
  const [levelUp, setLevelUp] = useState(false);
  const [lastXP, setLastXP] = useState(0);
  const [audioPlaying, setAudioPlaying] = useState(false);
  const [showEmojis, setShowEmojis] = useState(false);
  const [recentEmojis, setRecentEmojis] = useState<
    Array<{ id: number; emoji: string; x: number }>
  >([]);

  const progress = progressStore.getProgress(exerciseId);
  const isCompleted = progress?.status === "completed";

  useEffect(() => {
    const loadExercise = async () => {
      clearCurrent();
      clearConsole();
      try {
        const exercise = await exerciseApi.get(exerciseId);
        // Load saved code or use template
        const savedCode = progressStore.getProgress(exerciseId)?.savedCode;
        setCode(
          savedCode ||
            exercise.template_code ||
            "fn main() {\n    // Your code here\n}",
        );
        // Track that user started this exercise
        progressStore.startExercise(exerciseId);
      } catch (error) {
        console.error("Failed to load exercise:", error);
      }
    };
    loadExercise();
  }, [exerciseId, clearCurrent, clearConsole, setCode, progressStore]);

  const handleRun = async () => {
    await runCode(exerciseId);
  };

  const handleSubmit = async () => {
    await submitCode(exerciseId);

    if (lastSubmitResult?.success) {
      const executionTime = lastSubmitResult.executionTimeMs;

      if (engagementStore.xpEnabled) {
        const xpEarned = 10 + Math.floor(Math.random() * 5);
        const currentLevel = engagementStore.getLevel();

        engagementStore.addXP(xpEarned);
        engagementStore.incrementStreak();

        // Track exercise completion for achievements
        achievementsStore.completeExercise();

        // Track in progress store
        progressStore.completeExercise(exerciseId, xpEarned, executionTime);

        const newLevel = engagementStore.getLevel();
        setLastXP(xpEarned);

        if (newLevel > currentLevel) {
          setLevelUp(true);
          setShowEmojis(true);
          setTimeout(() => setShowEmojis(false), 2000);

          // Send level up notification
          notificationsStore.addNotification(
            createLevelUpNotification(newLevel),
          );
        }

        if (engagementStore.brainrotMode) {
          setShowEmojis(true);
          const emojis = [
            "🎉",
            "🔥",
            "💯",
            "🚀",
            "⭐",
            "⚡",
            "🎮",
            "💪",
            "🤩",
            "🥳",
          ];
          const newEmojis = Array.from({ length: 8 }, (_, i) => ({
            id: Date.now() + i,
            emoji: emojis[Math.floor(Math.random() * emojis.length)],
            x: 20 + Math.random() * 60,
          }));
          setRecentEmojis(newEmojis);
          setTimeout(() => setShowEmojis(false), 2000);
        }

        setShowXPPopup(true);
      } else {
        // Still track completion even if XP is disabled
        progressStore.completeExercise(exerciseId, 0, executionTime);
      }
    }
  };

  const handleReset = () => {
    if (currentExercise) {
      setCode(
        currentExercise.template_code ||
          "fn main() {\n    // Your code here\n}",
      );
    }
    clearConsole();
  };

  // Debounced code save
  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (code && code.trim()) {
        progressStore.saveCode(exerciseId, code);
      }
    }, 1000);

    return () => clearTimeout(timeoutId);
  }, [code, exerciseId, progressStore]);

  const getConsoleColor = (type: ConsoleEntry["type"]) => {
    switch (type) {
      case "success":
        return "text-success";
      case "error":
        return "text-error";
      case "warning":
        return "text-warning";
      case "output":
        return "text-text-primary";
      default:
        return "text-text-secondary";
    }
  };

  return (
    <div className="h-[calc(100vh-8rem)]">
      <XPPopup
        show={showXPPopup}
        amount={lastXP}
        onComplete={() => {
          setShowXPPopup(false);
          setLevelUp(false);
        }}
        levelUp={levelUp}
      />

      <div className="flex items-center gap-4 mb-6">
        <Link
          to="/exercises"
          className="p-2 rounded-lg bg-surface border border-border-subtle text-text-secondary hover:text-text-primary hover:border-primary/30 transition-all"
        >
          <ArrowLeft className="w-5 h-5" />
        </Link>
        <div className="flex-1">
          <div className="flex items-center gap-3">
            <span className="text-sm text-text-muted">
              Exercise {exerciseId}
            </span>
            {currentExercise && (
              <span
                className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-gradient-to-r ${difficultyColors[currentExercise.difficulty as keyof typeof difficultyColors]} text-white`}
              >
                {
                  difficultyLabels[
                    currentExercise.difficulty as keyof typeof difficultyLabels
                  ]
                }
              </span>
            )}
          </div>
          <h1 className="text-xl font-display font-bold text-text-primary">
            {currentExercise?.title || `Exercise ${exerciseId}`}
          </h1>
        </div>
      </div>

      <div className="grid lg:grid-cols-2 gap-6 h-[calc(100%-5rem)]">
        <div className="flex flex-col h-full">
          <div className="flex items-center gap-2 p-2 rounded-t-xl bg-surface border border-border-subtle border-b-0">
            <button
              onClick={() => setActiveTab("description")}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                activeTab === "description"
                  ? "bg-primary/10 text-primary"
                  : "text-text-secondary hover:text-text-primary"
              }`}
            >
              Description
            </button>
            <button
              onClick={() => setActiveTab("hints")}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                activeTab === "hints"
                  ? "bg-primary/10 text-primary"
                  : "text-text-secondary hover:text-text-primary"
              }`}
            >
              Hints
            </button>
            {currentExercise?.concepts && (
              <div className="flex items-center gap-2 ml-auto">
                {currentExercise.concepts.slice(0, 3).map((concept, i) => (
                  <span
                    key={i}
                    className="flex items-center gap-1 px-2 py-1 rounded-md bg-surface-hover text-xs text-text-secondary"
                  >
                    <Sparkles className="w-3 h-3" />
                    {concept}
                  </span>
                ))}
              </div>
            )}
          </div>

          <div className="flex-1 p-6 rounded-b-xl bg-surface border border-border-subtle overflow-auto">
            {isLoadingDetail ? (
              <div className="flex items-center justify-center h-full">
                <Loader2 className="w-8 h-8 animate-spin text-primary" />
              </div>
            ) : currentExercise ? (
              <div className="prose prose-invert max-w-none">
                <div className="flex items-center justify-between mb-4">
                  <p className="text-text-secondary">
                    {currentExercise.description}
                  </p>
                  {engagementStore.brainrotMode && (
                    <button
                      onClick={() => setAudioPlaying(!audioPlaying)}
                      className="p-2 rounded-lg bg-primary/10 text-primary hover:bg-primary/20 transition-colors"
                      title="Listen to explanation"
                    >
                      <Volume2 className="w-5 h-5" />
                    </button>
                  )}
                </div>

                {audioPlaying && engagementStore.brainrotMode && (
                  <div className="mb-4">
                    <AudioGuide
                      title="Exercise Guide"
                      content={currentExercise.description}
                      duration="2:00"
                    />
                  </div>
                )}

                {currentExercise.hints && currentExercise.hints.length > 0 && (
                  <div className="mt-6">
                    <button
                      onClick={() => setShowHints(!showHints)}
                      className="flex items-center gap-2 text-primary font-medium hover:text-primary-hover transition-colors"
                    >
                      <Zap className="w-4 h-4" />
                      {showHints ? "Hide Hints" : "Show Hints"}
                    </button>
                    {showHints && (
                      <ul className="mt-4 space-y-2">
                        {currentExercise.hints.map((hint, i) => (
                          <li
                            key={i}
                            className="flex items-start gap-2 text-text-secondary text-sm"
                          >
                            <span className="text-primary font-mono text-xs mt-0.5">
                              {i + 1}.
                            </span>
                            {hint}
                          </li>
                        ))}
                      </ul>
                    )}
                  </div>
                )}

                {currentExercise.bonus && (
                  <div className="mt-6 p-4 rounded-lg bg-gradient-to-br from-primary/10 to-purple-500/10 border border-primary/20">
                    <h4 className="text-sm font-medium text-primary mb-2">
                      Bonus Challenge
                    </h4>
                    <p className="text-text-secondary text-sm">
                      {currentExercise.bonus}
                    </p>
                  </div>
                )}
              </div>
            ) : (
              <p className="text-text-muted">Loading exercise...</p>
            )}
          </div>
        </div>

        <div className="flex flex-col h-full">
          <div className="flex-1 rounded-xl overflow-hidden border border-border-subtle bg-canvas">
            <Editor
              height="100%"
              defaultLanguage="rust"
              value={code}
              onChange={(value) => setCode(value || "")}
              theme="vs-dark"
              options={{
                fontSize: 14,
                fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
                minimap: { enabled: false },
                lineNumbers: "on",
                roundedSelection: true,
                scrollBeyondLastLine: false,
                automaticLayout: true,
                tabSize: 4,
                wordWrap: "on",
                padding: { top: 16, bottom: 16 },
              }}
            />
          </div>

          <div className="mt-4 h-48 rounded-xl bg-surface border border-border-subtle flex flex-col overflow-hidden">
            <div className="flex items-center justify-between px-4 py-2 border-b border-border-subtle bg-surface-hover">
              <span className="text-sm font-medium text-text-secondary">
                Console
              </span>
              <button
                onClick={clearConsole}
                className="text-xs text-text-muted hover:text-text-primary transition-colors"
              >
                Clear
              </button>
            </div>

            <div className="flex-1 overflow-auto p-4 font-mono text-sm scrollbar-thin">
              {consoleEntries.length === 0 ? (
                <p className="text-text-muted">Run your code to see output</p>
              ) : (
                consoleEntries.map((entry) => (
                  <div
                    key={entry.id}
                    className={`${getConsoleColor(entry.type)} mb-1`}
                  >
                    {entry.type === "output" && "> "}
                    {entry.message}
                  </div>
                ))
              )}
            </div>

            <div className="flex items-center gap-3 px-4 py-3 border-t border-border-subtle bg-surface-hover">
              <button
                onClick={handleRun}
                disabled={isRunning || isSubmitting}
                className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg bg-success text-white font-medium hover:bg-success-hover transition-all disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isRunning ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <Play className="w-4 h-4" />
                )}
                Run
              </button>

              <button
                onClick={handleSubmit}
                disabled={isRunning || isSubmitting}
                className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg bg-primary text-white font-medium hover:bg-primary-hover transition-all disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isSubmitting ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <CheckCircle2 className="w-4 h-4" />
                )}
                Submit
              </button>

              <button
                onClick={handleReset}
                className="p-2 rounded-lg bg-surface-hover text-text-secondary hover:text-text-primary transition-all"
                title="Reset code"
              >
                <RotateCcw className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>
      </div>

      {showEmojis && (
        <div className="fixed inset-0 pointer-events-none z-50">
          {recentEmojis.map((emoji) => (
            <motion.div
              key={emoji.id}
              initial={{ opacity: 1, y: 0, scale: 0.5 }}
              animate={{ opacity: 0, y: -200, scale: 1.5 }}
              transition={{ duration: 1.5, ease: "easeOut" }}
              className="absolute bottom-20 text-4xl"
              style={{ left: `${emoji.x}%` }}
            >
              {emoji.emoji}
            </motion.div>
          ))}
        </div>
      )}
    </div>
  );
}
