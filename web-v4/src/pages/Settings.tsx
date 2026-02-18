import { useEngagementStore } from "../stores/engagement";
import { useAuthStore } from "../stores/auth";
import {
  Settings as SettingsIcon,
  Zap,
  Flame,
  Volume2,
  Sparkles,
  Coffee,
  Brain,
  Save,
} from "lucide-react";
import { motion } from "framer-motion";
import { useState } from "react";

export default function SettingsPage() {
  const engagementStore = useEngagementStore();
  const { user } = useAuthStore();
  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  return (
    <div className="min-h-screen bg-canvas">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="max-w-4xl mx-auto p-8"
      >
        <div className="flex items-center gap-3 mb-8">
          <SettingsIcon className="w-10 h-10 text-primary" />
          <h1 className="text-4xl font-display font-bold text-text-primary">
            Settings
          </h1>
        </div>

        {user && (
          <div className="mb-8 p-4 rounded-xl bg-surface border border-border-subtle">
            <p className="text-text-secondary">
              Signed in as{" "}
              <span className="text-primary font-medium">{user.username}</span>
            </p>
          </div>
        )}

        <div className="space-y-6">
          <div className="bg-surface border border-border-subtle rounded-xl p-6">
            <div className="flex items-center gap-2 mb-6">
              <Zap className="w-6 h-6 text-primary" />
              <h2 className="text-xl font-bold text-text-primary">
                Engagement Features
              </h2>
            </div>

            <div className="space-y-4">
              <label className="flex items-center justify-between p-4 rounded-lg bg-surface-hover cursor-pointer">
                <div className="flex items-center gap-3">
                  <Zap className="w-5 h-5 text-primary" />
                  <div>
                    <p className="text-text-primary font-medium">XP System</p>
                    <p className="text-text-muted text-sm">
                      Earn XP for completing exercises
                    </p>
                  </div>
                </div>
                <input
                  type="checkbox"
                  checked={engagementStore.xpEnabled}
                  onChange={(e) =>
                    engagementStore.setXPSettings(e.target.checked)
                  }
                  className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary cursor-pointer"
                />
              </label>

              <label className="flex items-center justify-between p-4 rounded-lg bg-surface-hover cursor-pointer">
                <div className="flex items-center gap-3">
                  <Flame className="w-5 h-5 text-orange-500" />
                  <div>
                    <p className="text-text-primary font-medium">
                      Streak Tracking
                    </p>
                    <p className="text-text-muted text-sm">
                      Track your daily learning streak
                    </p>
                  </div>
                </div>
                <input
                  type="checkbox"
                  checked={engagementStore.streakEnabled}
                  onChange={(e) =>
                    engagementStore.setStreakSettings(e.target.checked)
                  }
                  className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary cursor-pointer"
                />
              </label>

              <label className="flex items-center justify-between p-4 rounded-lg bg-surface-hover cursor-pointer">
                <div className="flex items-center gap-3">
                  <Volume2 className="w-5 h-5 text-text-secondary" />
                  <div>
                    <p className="text-text-primary font-medium">Sounds</p>
                    <p className="text-text-muted text-sm">
                      Play sounds for achievements
                    </p>
                  </div>
                </div>
                <input
                  type="checkbox"
                  checked={engagementStore.soundsEnabled}
                  onChange={(e) =>
                    engagementStore.setSoundsSettings(e.target.checked)
                  }
                  className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary cursor-pointer"
                />
              </label>

              <label className="flex items-center justify-between p-4 rounded-lg bg-surface-hover cursor-pointer">
                <div className="flex items-center gap-3">
                  <Sparkles className="w-5 h-5 text-text-secondary" />
                  <div>
                    <p className="text-text-primary font-medium">Animations</p>
                    <p className="text-text-muted text-sm">
                      Show celebration animations
                    </p>
                  </div>
                </div>
                <input
                  type="checkbox"
                  checked={engagementStore.animationsEnabled}
                  onChange={(e) =>
                    engagementStore.setAnimationsSettings(e.target.checked)
                  }
                  className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary cursor-pointer"
                />
              </label>

              <label className="flex items-center justify-between p-4 rounded-lg bg-surface-hover cursor-pointer">
                <div className="flex items-center gap-3">
                  <Coffee className="w-5 h-5 text-primary" />
                  <div>
                    <p className="text-text-primary font-medium">Focus Timer</p>
                    <p className="text-text-muted text-sm">
                      Enable Pomodoro timer
                    </p>
                  </div>
                </div>
                <input
                  type="checkbox"
                  checked={engagementStore.focusTimerEnabled}
                  onChange={(e) =>
                    engagementStore.setFocusTimerSettings(e.target.checked)
                  }
                  className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary cursor-pointer"
                />
              </label>

              <label className="flex items-center justify-between p-4 rounded-lg bg-surface-hover cursor-pointer">
                <div className="flex items-center gap-3">
                  <Brain className="w-5 h-5 text-purple-500" />
                  <div>
                    <p className="text-text-primary font-medium">
                      Brainrot Mode
                    </p>
                    <p className="text-text-muted text-sm">
                      ADHD-friendly fun learning experience
                    </p>
                  </div>
                </div>
                <input
                  type="checkbox"
                  checked={engagementStore.brainrotMode}
                  onChange={(e) =>
                    engagementStore.setBrainrotMode(e.target.checked)
                  }
                  className="w-5 h-5 rounded border-border-subtle bg-surface text-primary focus:ring-primary cursor-pointer"
                />
              </label>
            </div>
          </div>

          <div className="flex items-center justify-between p-4 rounded-xl bg-surface border border-border-subtle">
            <div className="flex items-center gap-3">
              <Save className="w-5 h-5 text-success" />
              <p className="text-text-secondary">
                Settings are saved automatically to your browser
              </p>
            </div>
            {saved && (
              <motion.span
                initial={{ opacity: 0, y: -10 }}
                animate={{ opacity: 1, y: 0 }}
                className="text-success font-medium"
              >
                Saved!
              </motion.span>
            )}
          </div>

          <div className="mt-8 p-4 rounded-lg bg-surface-hover text-text-secondary text-sm">
            <h3 className="font-bold mb-2">💡 Tips</h3>
            <ul className="list-disc list-inside space-y-1">
              <li>XP and streak are stored locally in your browser</li>
              <li>Brainrot mode adds fun memes and emojis to your learning</li>
              <li>
                Focus timer uses Pomodoro technique (25 min work, 5 min break)
              </li>
            </ul>
          </div>
        </div>
      </motion.div>
    </div>
  );
}
