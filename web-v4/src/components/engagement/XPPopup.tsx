import { useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import confetti from "canvas-confetti";
import { Star, Zap, Trophy } from "lucide-react";

interface XPPopupProps {
  show: boolean;
  amount: number;
  onComplete: () => void;
  levelUp?: boolean;
}

export function XPPopup({
  show,
  amount,
  onComplete,
  levelUp = false,
}: XPPopupProps) {
  const hasTriggered = useRef(false);

  useEffect(() => {
    if (show && !hasTriggered.current) {
      hasTriggered.current = true;

      // Trigger confetti
      const duration = 3 * 1000;
      const animationEnd = Date.now() + duration;
      const defaults = { startVelocity: 30, spread: 360, ticks: 60, zIndex: 0 };

      const randomInRange = (min: number, max: number) =>
        Math.random() * (max - min) + min;

      const interval = setInterval(function () {
        const timeLeft = animationEnd - Date.now();

        if (timeLeft <= 0) {
          clearInterval(interval);
          return;
        }

        const particleCount = 50 * (timeLeft / duration);
        confetti({
          ...defaults,
          particleCount,
          origin: {
            x: randomInRange(0.1, 0.3),
            y: randomInRange(0.1, 0.3),
          },
        });
        confetti({
          ...defaults,
          particleCount,
          origin: {
            x: randomInRange(0.7, 0.9),
            y: randomInRange(0.1, 0.3),
          },
        });
      }, 250);

      // Complete after animation
      setTimeout(() => {
        hasTriggered.current = false;
        onComplete();
      }, 3000);
    }
  }, [show, amount, onComplete, levelUp]);

  return (
    <AnimatePresence>
      {show && (
        <motion.div
          initial={{ scale: 0, opacity: 0, y: 50 }}
          animate={{ scale: 1, opacity: 1, y: 0 }}
          exit={{ scale: 0, opacity: 0, y: -50 }}
          className="fixed inset-0 pointer-events-none z-50 flex items-center justify-center"
        >
          <div className="bg-gradient-to-br from-primary to-orange-500 text-white px-8 py-6 rounded-2xl shadow-2xl shadow-primary/50 flex flex-col items-center gap-2">
            {levelUp ? (
              <>
                <Trophy className="w-12 h-12" />
                <span className="text-2xl font-bold">LEVEL UP!</span>
              </>
            ) : (
              <>
                <Star className="w-12 h-12" />
                <span className="text-2xl font-bold">+{amount} XP</span>
              </>
            )}
            <span className="text-white/80 text-sm">Keep going!</span>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

export function MiniXP({ amount }: { amount: number }) {
  return (
    <div className="inline-flex items-center gap-1 px-2 py-1 rounded-full bg-primary/20 text-primary text-sm font-medium">
      <Zap className="w-4 h-4" />
      <span>{amount} XP</span>
    </div>
  );
}
