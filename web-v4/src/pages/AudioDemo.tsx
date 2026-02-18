import { useState } from "react";
import { motion } from "framer-motion";
import { Volume2, Headphones, BookOpen, Sparkles } from "lucide-react";
import {
  AudioGuide,
  AudioGuideList,
  VoiceSelector,
  useTextToSpeech,
} from "../components/audio";

const exerciseGuides = [
  {
    id: "1",
    title: "Variables in Rust",
    content:
      "Welcome to Rust! In Rust, variables are immutable by default, which means once you assign a value to a variable, you can't change it. This is Rust's way of helping you write safe code. But don't worry, you can make a variable mutable by using the 'mut' keyword. This gives you flexibility when you need it.",
    duration: "0:45",
  },
  {
    id: "2",
    title: "Functions in Rust",
    content:
      "Functions in Rust are declared using the 'fn' keyword. They can take parameters and return values. Rust functions are great for organizing your code into reusable blocks. Remember, the last expression in a function is its return value if there's no semicolon!",
    duration: "0:38",
  },
  {
    id: "3",
    title: "Ownership Explained",
    content:
      "Ownership is Rust's most unique feature! Every value has a single owner. When the owner goes out of scope, the value is dropped. This means Rust can guarantee memory safety without a garbage collector. It's like having a personal manager for your memory!",
    duration: "0:52",
  },
];

const bookGuides = [
  {
    id: "b1",
    title: "Chapter 1: Getting Started",
    content:
      "Welcome to the Rust Learning Ground! This chapter will guide you through setting up your development environment, installing Rust, and writing your first program. By the end of this chapter, you'll have Rust installed on your computer and be ready to start your Rust journey.",
    duration: "2:15",
  },
  {
    id: "b2",
    title: "Chapter 2: Programming a Guessing Game",
    content:
      "Let's build a fun guessing game! You'll learn about variables, loops, and user input. We'll use the 'rand' crate to generate random numbers and the 'std::io' module to read from the console. This is a classic programming exercise that teaches you the basics of Rust programming.",
    duration: "3:42",
  },
];

export default function AudioDemo() {
  const { speak: testSpeak, isSpeaking, voices, supported } = useTextToSpeech();
  const [selectedVoice, setSelectedVoice] = useState<string | null>(null);

  const testText =
    "Welcome to Rust Learning Ground! Click play to hear this text spoken aloud.";

  return (
    <div className="min-h-screen bg-canvas p-8">
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="max-w-4xl mx-auto"
      >
        <div className="flex items-center gap-3 mb-8">
          <Headphones className="w-10 h-10 text-primary" />
          <h1 className="text-4xl font-display font-bold text-text-primary">
            AI Voice Guides Demo
          </h1>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
          <div className="bg-surface border border-border-subtle rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <Volume2 className="w-6 h-6 text-primary" />
              <h2 className="text-xl font-bold text-text-primary">
                Voice Settings
              </h2>
            </div>

            {supported ? (
              <div className="space-y-4">
                <VoiceSelector
                  voices={voices}
                  selectedVoice={selectedVoice}
                  onSelect={setSelectedVoice}
                />

                <button
                  onClick={() => testSpeak(testText)}
                  className="w-full px-4 py-3 bg-primary text-white rounded-lg font-medium hover:bg-primary-hover transition-colors"
                >
                  Test Voice
                </button>

                <div className="text-sm text-text-muted">
                  {isSpeaking ? "🔊 Playing..." : "Ready to speak"}
                </div>
              </div>
            ) : (
              <p className="text-text-secondary">
                Text-to-speech is not supported in your browser.
              </p>
            )}
          </div>

          <div className="bg-surface border border-border-subtle rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <BookOpen className="w-6 h-6 text-primary" />
              <h2 className="text-xl font-bold text-text-primary">
                Exercise Guides
              </h2>
            </div>
            <p className="text-text-secondary text-sm mb-4">
              Listen to exercise explanations while you learn.
            </p>
            <AudioGuideList guides={exerciseGuides} />
          </div>
        </div>

        <div className="bg-surface border border-border-subtle rounded-xl p-6">
          <div className="flex items-center gap-2 mb-4">
            <Sparkles className="w-6 h-6 text-purple-500" />
            <h2 className="text-xl font-bold text-text-primary">
              Book Chapter Guides
            </h2>
          </div>
          <p className="text-text-secondary text-sm mb-4">
            Listen to full chapter summaries and explanations.
          </p>
          <AudioGuideList guides={bookGuides} />
        </div>

        <div className="mt-8 p-4 rounded-lg bg-surface-hover text-text-secondary text-sm">
          <h3 className="font-bold mb-2">💡 How to use Voice Guides:</h3>
          <ul className="list-disc list-inside space-y-1">
            <li>Select your preferred voice from the dropdown</li>
            <li>Click the play button to hear the narration</li>
            <li>Perfect for learning while multitasking!</li>
            <li>Works with screen readers and accessibility tools</li>
          </ul>
        </div>
      </motion.div>
    </div>
  );
}
