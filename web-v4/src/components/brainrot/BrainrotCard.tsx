import { motion } from "framer-motion";
import { Sparkles, Zap, Gamepad2, Brain } from "lucide-react";

interface MemeCardProps {
  title: string;
  memeText: string;
  explanation: string;
  type: "gaming" | "meme" | "reference";
}

const memeStyles = {
  gaming: {
    gradient: "from-purple-500 to-pink-500",
    icon: Gamepad2,
    border: "border-purple-500/30",
  },
  meme: {
    gradient: "from-orange-500 to-red-500",
    icon: Sparkles,
    border: "border-orange-500/30",
  },
  reference: {
    gradient: "from-blue-500 to-cyan-500",
    icon: Brain,
    border: "border-blue-500/30",
  },
};

export function MemeCard({
  title,
  memeText,
  explanation,
  type,
}: MemeCardProps) {
  const style = memeStyles[type];
  const Icon = style.icon;

  return (
    <motion.div
      initial={{ scale: 0.9, opacity: 0 }}
      animate={{ scale: 1, opacity: 1 }}
      whileHover={{ scale: 1.02 }}
      className={`bg-gradient-to-br ${style.gradient} p-1 rounded-xl`}
    >
      <div className="bg-canvas rounded-lg p-4">
        <div className="flex items-center gap-2 mb-3">
          <Icon
            className={`w-5 h-5 text-${type === "gaming" ? "purple" : type === "meme" ? "orange" : "blue"}-400`}
          />
          <span className="text-xs font-bold uppercase tracking-wider text-text-muted">
            {type}
          </span>
        </div>

        <h4 className="text-lg font-bold text-text-primary mb-2">{title}</h4>

        <div
          className="bg-surface-hover rounded-lg p-3 mb-3 border-l-4"
          style={{ borderColor: "var(--color)" }}
        >
          <p className="text-text-primary font-medium italic">"{memeText}"</p>
        </div>

        <p className="text-text-secondary text-sm">{explanation}</p>
      </div>
    </motion.div>
  );
}

export function BrainrotBanner() {
  return (
    <motion.div
      initial={{ y: -20, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      className="bg-gradient-to-r from-purple-600 via-pink-500 to-orange-500 text-white px-6 py-3 rounded-xl flex items-center justify-between gap-4 shadow-lg shadow-purple-500/20"
    >
      <div className="flex items-center gap-3">
        <motion.div
          animate={{ rotate: [0, 10, -10, 0] }}
          transition={{ repeat: Infinity, duration: 2 }}
        >
          <Zap className="w-6 h-6" />
        </motion.div>
        <div>
          <h3 className="font-bold text-lg">BRAINROT MODE ACTIVATED</h3>
          <p className="text-white/80 text-sm">
            Gaming references + memes incoming
          </p>
        </div>
      </div>

      <motion.button
        whileHover={{ scale: 1.05 }}
        whileTap={{ scale: 0.95 }}
        className="bg-white/20 hover:bg-white/30 px-4 py-2 rounded-lg text-sm font-medium transition-colors"
      >
        Disable 💀
      </motion.button>
    </motion.div>
  );
}

export function FloatingReaction({ emoji }: { emoji: string }) {
  return (
    <motion.div
      initial={{ y: 0, opacity: 1 }}
      animate={{ y: -100, opacity: 0 }}
      transition={{ duration: 2 }}
      className="fixed text-4xl pointer-events-none z-50"
      style={{ left: `${Math.random() * 80 + 10}%`, bottom: 0 }}
    >
      {emoji}
    </motion.div>
  );
}

export function ReactionButton({
  emoji,
  onClick,
}: {
  emoji: string;
  onClick: () => void;
}) {
  return (
    <motion.button
      whileHover={{ scale: 1.2, rotate: [0, -10, 10, 0] }}
      whileTap={{ scale: 0.9 }}
      onClick={onClick}
      className="text-2xl p-2 hover:bg-surface-hover rounded-full transition-colors"
    >
      {emoji}
    </motion.button>
  );
}

export function ReactionBar({
  reactions,
  onReact,
}: {
  reactions: string[];
  onReact: (emoji: string) => void;
}) {
  return (
    <div className="flex items-center gap-1 p-2 bg-surface rounded-full">
      {reactions.map((emoji, i) => (
        <ReactionButton key={i} emoji={emoji} onClick={() => onReact(emoji)} />
      ))}
    </div>
  );
}
