import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { useEngagementStore } from "../stores/engagement";
import {
  MemeCard,
  BrainrotBanner,
  FloatingReaction,
  ReactionBar,
  VideoGuideCard,
  VideoGrid,
} from "../components/brainrot";
import { Brain, Gamepad2, Sparkles, Play } from "lucide-react";

const sampleMemes = [
  {
    title: "Ownership in Minecraft",
    memeText:
      "You can't have that item. It's not YOURS. It's MINE. *Rust ownership goes brrrr*",
    explanation:
      "Rust's ownership model is like having a personal inventory in Minecraft. Once you give something away, it's gone from your inventory!",
    type: "gaming" as const,
  },
  {
    title: "Borrow Checker",
    memeText:
      "Hey bro, can I borrow that variable? NO. YOU CAN'T. It's borrowed already!",
    explanation:
      "The borrow checker is like your roommate who gets mad if you use their stuff without asking. In Rust, you gotta ask nicely (references) and follow the rules!",
    type: "meme" as const,
  },
  {
    title: "Lifetimes",
    memeText:
      "How long does this reference live? IDK, let's ask the compiler. The compiler: 😅",
    explanation:
      "Lifetimes are Rust's way of making sure references don't outlive the data they point to. It's like a warranty period for your references!",
    type: "reference" as const,
  },
];

const sampleVideos = [
  {
    id: "1",
    title: "Variables Explained",
    description: "Learn Rust variables in 60 seconds! No cap, this is fire 🔥",
    thumbnail:
      "https://images.unsplash.com/photo-1515879218367-8466d910aaa4?w=400",
    duration: "0:60",
    onClick: () => console.log("Video 1 clicked"),
  },
  {
    id: "2",
    title: "Functions Quick",
    description: "Functions be like... do the thing, get the result ✨",
    thumbnail:
      "https://images.unsplash.com/photo-1461749280684-dccba630e2f6?w=400",
    duration: "0:45",
    onClick: () => console.log("Video 2 clicked"),
  },
  {
    id: "3",
    title: "Ownership Dance",
    description: "When ownership moves... *chef's kiss* 😘",
    thumbnail:
      "https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=400",
    duration: "1:15",
    onClick: () => console.log("Video 3 clicked"),
  },
  {
    id: "4",
    title: "Error Handling",
    description: "Errors hitting different? Just panic! 💀",
    thumbnail:
      "https://images.unsplash.com/photo-1555099962-4199c345e5dd?w=400",
    duration: "0:55",
    onClick: () => console.log("Video 4 clicked"),
  },
];

const reactions = ["🔥", "💯", "😂", "🤡", "✨", "💀", "👀", "🚀"];

export default function BrainrotDemo() {
  const { brainrotMode, setBrainrotMode } = useEngagementStore();
  const [showReactions, setShowReactions] = useState<string[]>([]);
  const [activeReactions, setActiveReactions] = useState<string[]>([]);

  const handleReaction = (emoji: string) => {
    const id = Math.random().toString();
    setShowReactions((prev) => [...prev, id]);
    setActiveReactions((prev) => [...prev, emoji]);

    setTimeout(() => {
      setShowReactions((prev) => prev.filter((r) => r !== id));
    }, 2000);
  };

  return (
    <div className="min-h-screen bg-canvas p-8">
      <AnimatePresence>
        {showReactions.map((id) => (
          <FloatingReaction key={id} emoji={activeReactions[0] || "🔥"} />
        ))}
      </AnimatePresence>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="max-w-6xl mx-auto"
      >
        <div className="flex items-center justify-between mb-8">
          <div className="flex items-center gap-3">
            <motion.div
              animate={{ rotate: [0, 10, -10, 0] }}
              transition={{ repeat: Infinity, duration: 2 }}
            >
              <Brain className="w-10 h-10 text-purple-500" />
            </motion.div>
            <h1 className="text-4xl font-display font-bold bg-gradient-to-r from-purple-500 to-pink-500 bg-clip-text text-transparent">
              Brainrot Mode Demo
            </h1>
          </div>

          <motion.button
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            onClick={() => setBrainrotMode(!brainrotMode)}
            className={`px-6 py-3 rounded-xl font-bold transition-all ${
              brainrotMode
                ? "bg-gradient-to-r from-purple-500 to-pink-500 text-white shadow-lg shadow-purple-500/30"
                : "bg-surface border border-border-subtle text-text-secondary"
            }`}
          >
            {brainrotMode ? "🧠 BRAINROT: ON" : "💀 Enable Brainrot"}
          </motion.button>
        </div>

        {brainrotMode && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: "auto", opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            className="mb-8"
          >
            <BrainrotBanner />
          </motion.div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
          <div className="bg-surface border border-border-subtle rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <Gamepad2 className="w-6 h-6 text-purple-500" />
              <h2 className="text-xl font-bold text-text-primary">
                🎮 Brainrot Memes
              </h2>
            </div>
            <div className="grid grid-cols-1 gap-4">
              {sampleMemes.map((meme, i) => (
                <MemeCard key={i} {...meme} />
              ))}
            </div>
          </div>

          <div className="bg-surface border border-border-subtle rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <Sparkles className="w-6 h-6 text-pink-500" />
              <h2 className="text-xl font-bold text-text-primary">
                🎬 Quick Video Guides
              </h2>
            </div>
            <VideoGrid videos={sampleVideos} />
          </div>
        </div>

        <div className="bg-surface border border-border-subtle rounded-xl p-6">
          <div className="flex items-center gap-2 mb-4">
            <span className="text-2xl">👇</span>
            <h2 className="text-xl font-bold text-text-primary">
              Drop a Reaction!
            </h2>
          </div>
          <div className="flex justify-center">
            <ReactionBar reactions={reactions} onReact={handleReaction} />
          </div>
        </div>

        <div className="mt-8 p-4 rounded-lg bg-surface-hover text-text-secondary text-sm">
          <h3 className="font-bold mb-2">💡 How Brainrot Mode Works:</h3>
          <ul className="list-disc list-inside space-y-1">
            <li>Toggle Brainrot Mode to enable meme-style explanations</li>
            <li>Gaming references help you understand Rust concepts</li>
            <li>Quick 60-second video guides for fast learning</li>
            <li>React with emojis to show your engagement!</li>
            <li>All content is family-friendly but high-energy!</li>
          </ul>
        </div>
      </motion.div>
    </div>
  );
}
