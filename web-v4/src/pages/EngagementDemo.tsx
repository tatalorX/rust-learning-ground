import { useState } from "react";
import { motion } from "framer-motion";
import { useEngagementStore } from "../stores/engagement";
import {
  XPPopup,
  StreakCard,
  FocusTimer,
  SoundManager,
} from "../components/engagement";
import { Settings, Zap, Flame, Volume2, Coffee, Sparkles } from "lucide-react";

export default function EngagementDemo() {
  const store = useEngagementStore();
  const [showXPPopup, setShowXPPopup] = useState(false);
  const [levelUp, setLevelUp] = useState(false);

  const handleEarnXP = () => {
    setShowXPPopup(true);
    store.addXP(25);
    setTimeout(() => setShowXPPopup(false), 3000);
  };

  const handleLevelUp = () => {
    setShowXPPopup(true);
    setLevelUp(true);
    store.addXP(100);
    setTimeout(() => {
      setShowXPPopup(false);
      setLevelUp(false);
    }, 3000);
  };

  return (
    <div className="min-h-screen bg-canvas p-8">
      <XPPopup
        show={showXPPopup}
        amount={25}
        onComplete={() => setShowXPPopup(false)}
        levelUp={levelUp}
      />

      <SoundManager enabled={store.soundsEnabled} />

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="max-w-4xl mx-auto"
      >
        <h1 className="text-4xl font-display font-bold text-text-primary mb-8">
          🎮 Engagement Features Demo
        </h1>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
          <div className="bg-surface border border-border-subtle rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <Zap className="w-6 h-6 text-primary" />
              <h2 className="text-xl font-bold text-text-primary">
                XP & Levels
              </h2>
            </div>
            <div className="text-3xl font-bold text-primary mb-2">
              {store.xp} XP
            </div>
            <div className="text-text-secondary mb-4">
              Level {store.getLevel()}
            </div>
            <div className="h-3 bg-surface-hover rounded-full overflow-hidden mb-4">
              <motion.div
                className="h-full bg-gradient-to-r from-primary to-orange-500"
                style={{
                  width: `${((store.xp % 100) / 100) * 100}%`,
                }}
                transition={{ duration: 0.5 }}
              />
            </div>
            <div className="flex gap-2">
              <motion.button
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                onClick={handleEarnXP}
                className="px-4 py-2 bg-primary text-white rounded-lg"
              >
                +25 XP
              </motion.button>
              <motion.button
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                onClick={handleLevelUp}
                className="px-4 py-2 bg-gradient-to-r from-primary to-orange-500 text-white rounded-lg"
              >
                Level Up!
              </motion.button>
            </div>
          </div>

          <div className="bg-surface border border-border-subtle rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <Flame className="w-6 h-6 text-orange-500" />
              <h2 className="text-xl font-bold text-text-primary">Streak</h2>
            </div>
            <StreakCard streak={store.streak} />
          </div>

          <div className="bg-surface border border-border-subtle rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <Coffee className="w-6 h-6 text-primary" />
              <h2 className="text-xl font-bold text-text-primary">
                Focus Timer
              </h2>
            </div>
            <FocusTimer
              enabled={store.focusTimerEnabled}
              onComplete={() => store.completeFocusSession()}
            />
          </div>
        </div>

        <div className="bg-surface border border-border-subtle rounded-xl p-6">
          <div className="flex items-center gap-2 mb-4">
            <Settings className="w-6 h-6 text-text-secondary" />
            <h2 className="text-xl font-bold text-text-primary">Settings</h2>
          </div>

          <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
            <label className="flex items-center gap-3 p-3 rounded-lg bg-surface-hover cursor-pointer">
              <input
                type="checkbox"
                checked={store.xpEnabled}
                onChange={(e) => store.setXPSettings(e.target.checked)}
                className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary"
              />
              <Zap className="w-5 h-5 text-primary" />
              <span className="text-text-primary">XP System</span>
            </label>

            <label className="flex items-center gap-3 p-3 rounded-lg bg-surface-hover cursor-pointer">
              <input
                type="checkbox"
                checked={store.streakEnabled}
                onChange={(e) => store.setStreakSettings(e.target.checked)}
                className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary"
              />
              <Flame className="w-5 h-5 text-orange-500" />
              <span className="text-text-primary">Streak</span>
            </label>

            <label className="flex items-center gap-3 p-3 rounded-lg bg-surface-hover cursor-pointer">
              <input
                type="checkbox"
                checked={store.soundsEnabled}
                onChange={(e) => store.setSoundsSettings(e.target.checked)}
                className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary"
              />
              <Volume2 className="w-5 h-5 text-text-secondary" />
              <span className="text-text-primary">Sounds</span>
            </label>

            <label className="flex items-center gap-3 p-3 rounded-lg bg-surface-hover cursor-pointer">
              <input
                type="checkbox"
                checked={store.focusTimerEnabled}
                onChange={(e) => store.setFocusTimerSettings(e.target.checked)}
                className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary"
              />
              <Coffee className="w-5 h-5 text-primary" />
              <span className="text-text-primary">Focus Timer</span>
            </label>

            <label className="flex items-center gap-3 p-3 rounded-lg bg-surface-hover cursor-pointer">
              <input
                type="checkbox"
                checked={store.animationsEnabled}
                onChange={(e) => store.setAnimationsSettings(e.target.checked)}
                className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary"
              />
              <Sparkles className="w-5 h-5 text-text-secondary" />
              <span className="text-text-primary">Animations</span>
            </label>

            <label className="flex items-center gap-3 p-3 rounded-lg bg-surface-hover cursor-pointer">
              <input
                type="checkbox"
                checked={store.brainrotMode}
                onChange={(e) => store.setBrainrotMode(e.target.checked)}
                className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary"
              />
              <Sparkles className="w-5 h-5 text-purple-500" />
              <span className="text-text-primary">Brainrot Mode</span>
            </label>
          </div>
        </div>

        <div className="mt-8 p-4 rounded-lg bg-surface-hover text-text-secondary text-sm">
          <h3 className="font-bold mb-2">How to use:</h3>
          <ul className="list-disc list-inside space-y-1">
            <li>Click "+25 XP" to earn XP and see the popup</li>
            <li>Click "Level Up!" to see the celebration animation</li>
            <li>Toggle settings to enable/disable features</li>
            <li>Use the Focus Timer for Pomodoro sessions</li>
            <li>Streak tracks your daily activity</li>
          </ul>
        </div>
      </motion.div>
    </div>
  );
}
