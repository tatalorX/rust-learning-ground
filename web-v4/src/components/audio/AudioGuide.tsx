import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Play, Pause, RotateCcw, Volume2, VolumeX, Clock } from "lucide-react";
import { useTextToSpeech } from "../../hooks/useTextToSpeech";

interface AudioGuideProps {
  title: string;
  content: string;
  duration?: string;
}

export function AudioGuide({ title, content, duration }: AudioGuideProps) {
  const { speak, cancel, isSpeaking, isPaused, pause, resume, supported } =
    useTextToSpeech({
      rate: 0.9,
      pitch: 1,
      volume: 1,
      onStart: () => console.log("Audio guide started"),
      onEnd: () => console.log("Audio guide ended"),
    });

  const [progress, setProgress] = useState(0);

  if (!supported) {
    return (
      <div className="bg-surface border border-border-subtle rounded-xl p-4">
        <p className="text-text-secondary text-sm">
          Text-to-speech is not supported in your browser.
        </p>
      </div>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="bg-gradient-to-br from-primary/10 to-purple-500/10 border border-primary/20 rounded-2xl p-6"
    >
      <div className="flex items-start gap-4">
        <div className="flex-shrink-0">
          <motion.button
            whileHover={{ scale: 1.1 }}
            whileTap={{ scale: 0.9 }}
            onClick={() => {
              if (isSpeaking) {
                cancel();
              } else {
                speak(content);
              }
            }}
            className="w-16 h-16 rounded-full bg-gradient-to-br from-primary to-primary-hover flex items-center justify-center shadow-lg shadow-primary/30"
          >
            {isSpeaking ? (
              <Pause className="w-8 h-8 text-white" />
            ) : (
              <Play className="w-8 h-8 text-white fill-white ml-1" />
            )}
          </motion.button>
        </div>

        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <h3 className="text-lg font-bold text-text-primary">{title}</h3>
            {duration && (
              <span className="flex items-center gap-1 text-xs text-text-muted bg-surface px-2 py-1 rounded-full">
                <Clock className="w-3 h-3" />
                {duration}
              </span>
            )}
          </div>

          <p className="text-text-secondary text-sm mb-4 line-clamp-2">
            {content}
          </p>

          {isSpeaking && (
            <div className="flex items-center gap-4">
              <button
                onClick={isPaused ? resume : pause}
                className="p-2 rounded-full bg-surface hover:bg-surface-hover transition-colors"
              >
                {isPaused ? (
                  <Play className="w-4 h-4 text-text-primary" />
                ) : (
                  <Pause className="w-4 h-4 text-text-primary" />
                )}
              </button>

              <button
                onClick={cancel}
                className="p-2 rounded-full bg-surface hover:bg-surface-hover transition-colors"
              >
                <RotateCcw className="w-4 h-4 text-text-primary" />
              </button>

              <div className="flex-1 h-2 bg-surface-hover rounded-full overflow-hidden">
                <motion.div
                  className="h-full bg-gradient-to-r from-primary to-purple-500"
                  initial={{ width: 0 }}
                  animate={{ width: `${progress}%` }}
                />
              </div>

              <span className="text-xs text-text-muted">
                {isSpeaking ? "Playing..." : "Ready"}
              </span>
            </div>
          )}
        </div>
      </div>
    </motion.div>
  );
}

interface AudioGuideListProps {
  guides: Array<{
    id: string;
    title: string;
    content: string;
    duration?: string;
  }>;
}

export function AudioGuideList({ guides }: AudioGuideListProps) {
  return (
    <div className="space-y-4">
      {guides.map((guide) => (
        <AudioGuide key={guide.id} {...guide} />
      ))}
    </div>
  );
}

interface VoiceSelectorProps {
  voices: SpeechSynthesisVoice[];
  selectedVoice: string | null;
  onSelect: (voiceName: string) => void;
}

export function VoiceSelector({
  voices,
  selectedVoice,
  onSelect,
}: VoiceSelectorProps) {
  const [isOpen, setIsOpen] = useState(false);

  const selected = voices.find((v) => v.name === selectedVoice);

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 px-4 py-2 bg-surface border border-border-subtle rounded-lg text-text-primary hover:border-primary/30 transition-colors"
      >
        <Volume2 className="w-4 h-4" />
        <span className="text-sm">{selected?.name || "Select Voice"}</span>
      </button>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            className="absolute top-full left-0 mt-2 w-64 bg-surface border border-border-subtle rounded-xl shadow-xl z-50 max-h-64 overflow-y-auto"
          >
            {voices.map((voice) => (
              <button
                key={voice.name}
                onClick={() => {
                  onSelect(voice.name);
                  setIsOpen(false);
                }}
                className={`w-full px-4 py-2 text-left text-sm hover:bg-surface-hover transition-colors ${
                  voice.name === selectedVoice
                    ? "text-primary bg-primary/10"
                    : "text-text-primary"
                }`}
              >
                {voice.name} ({voice.lang})
                {voice.default && (
                  <span className="text-xs text-text-muted ml-2">Default</span>
                )}
              </button>
            ))}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
