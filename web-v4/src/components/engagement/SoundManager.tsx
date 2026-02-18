import { useCallback, useRef, useEffect, useState } from "react";
import { Volume2, VolumeX, Speaker } from "lucide-react";
import { useEngagementStore } from "../../stores/engagement";

interface SoundManagerProps {
  enabled: boolean;
}

export function SoundManager({ enabled }: SoundManagerProps) {
  const soundsEnabled = useEngagementStore((s) => s.soundsEnabled);
  const audioContext = useRef<AudioContext | null>(null);

  useEffect(() => {
    if (soundsEnabled && !audioContext.current) {
      audioContext.current = new (
        window.AudioContext || (window as any).webkitAudioContext
      )();
    }
  }, [soundsEnabled]);

  const playTone = useCallback(
    (frequency: number, duration: number, type: OscillatorType = "sine") => {
      if (!soundsEnabled || !audioContext.current) return;

      const ctx = audioContext.current;
      const oscillator = ctx.createOscillator();
      const gainNode = ctx.createGain();

      oscillator.connect(gainNode);
      gainNode.connect(ctx.destination);

      oscillator.type = type;
      oscillator.frequency.setValueAtTime(frequency, ctx.currentTime);

      gainNode.gain.setValueAtTime(0.3, ctx.currentTime);
      gainNode.gain.exponentialRampToValueAtTime(
        0.01,
        ctx.currentTime + duration,
      );

      oscillator.start(ctx.currentTime);
      oscillator.stop(ctx.currentTime + duration);
    },
    [soundsEnabled],
  );

  const playSuccess = useCallback(() => {
    playTone(523.25, 0.1); // C5
    setTimeout(() => playTone(659.25, 0.1), 100); // E5
    setTimeout(() => playTone(783.99, 0.15), 200); // G5
  }, [playTone]);

  const playError = useCallback(() => {
    playTone(200, 0.15, "sawtooth");
    setTimeout(() => playTone(150, 0.15, "sawtooth"), 150);
  }, [playTone]);

  const playClick = useCallback(() => {
    playTone(800, 0.05, "sine");
  }, [playTone]);

  const playLevelUp = useCallback(() => {
    const notes = [523.25, 587.33, 659.25, 698.46, 783.99, 880, 987.77, 1046.5];
    notes.forEach((freq, i) => {
      setTimeout(() => playTone(freq, 0.2), i * 100);
    });
  }, [playTone]);

  const playTimerEnd = useCallback(() => {
    playTone(880, 0.3);
    setTimeout(() => playTone(1100, 0.3), 200);
    setTimeout(() => playTone(1320, 0.4), 400);
  }, [playTone]);

  // Expose play functions globally for easy access
  useEffect(() => {
    if (typeof window !== "undefined") {
      (window as any).__soundManager = {
        success: playSuccess,
        error: playError,
        click: playClick,
        levelUp: playLevelUp,
        timerEnd: playTimerEnd,
      };
    }
  }, [playSuccess, playError, playClick, playLevelUp, playTimerEnd]);

  return null;
}

export function SoundToggle() {
  const soundsEnabled = useEngagementStore((s) => s.soundsEnabled);
  const setSoundsSettings = useEngagementStore((s) => s.setSoundsSettings);

  return (
    <button
      onClick={() => setSoundsSettings(!soundsEnabled)}
      className="p-2 rounded-lg bg-surface border border-border-subtle text-text-secondary hover:text-text-primary hover:border-primary/30 transition-all"
      title={soundsEnabled ? "Sound ON" : "Sound OFF"}
    >
      {soundsEnabled ? (
        <Volume2 className="w-5 h-5" />
      ) : (
        <VolumeX className="w-5 h-5" />
      )}
    </button>
  );
}

export function SoundButton({
  type,
}: {
  type: "success" | "error" | "levelUp";
}) {
  const soundsEnabled = useEngagementStore((s) => s.soundsEnabled);

  const play = useCallback(() => {
    if (!soundsEnabled || !(window as any).__soundManager) return;
    (window as any).__soundManager[type]();
  }, [soundsEnabled, type]);

  return (
    <button
      onClick={play}
      className="p-2 rounded-lg bg-surface border border-border-subtle text-text-secondary hover:text-primary hover:border-primary/30 transition-all"
      title={`Play ${type} sound`}
    >
      <Speaker className="w-5 h-5" />
    </button>
  );
}
