import { useState, useCallback, useEffect, useRef } from "react";

interface UseTextToSpeechOptions {
  rate?: number;
  pitch?: number;
  volume?: number;
  voice?: string;
  onStart?: () => void;
  onEnd?: () => void;
  onError?: (error: Error) => void;
}

interface UseTextToSpeechReturn {
  speak: (text: string) => void;
  cancel: () => void;
  isSpeaking: boolean;
  isPaused: boolean;
  voices: SpeechSynthesisVoice[];
  supported: boolean;
  pause: () => void;
  resume: () => void;
}

export function useTextToSpeech(
  options: UseTextToSpeechOptions = {},
): UseTextToSpeechReturn {
  const {
    rate = 1,
    pitch = 1,
    volume = 1,
    voice,
    onStart,
    onEnd,
    onError,
  } = options;

  const [isSpeaking, setIsSpeaking] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const [voices, setVoices] = useState<SpeechSynthesisVoice[]>([]);
  const utteranceRef = useRef<SpeechSynthesisUtterance | null>(null);
  const pausePositionRef = useRef<number>(0);

  const supported =
    typeof window !== "undefined" && "speechSynthesis" in window;

  useEffect(() => {
    if (!supported) return;

    const loadVoices = () => {
      const synthesis = window.speechSynthesis;
      setVoices(synthesis.getVoices());
    };

    loadVoices();
    window.speechSynthesis.onvoiceschanged = loadVoices;

    return () => {
      window.speechSynthesis.onvoiceschanged = null;
    };
  }, [supported]);

  const speak = useCallback(
    (text: string) => {
      if (!supported) return;

      const synthesis = window.speechSynthesis;
      synthesis.cancel();

      const utterance = new SpeechSynthesisUtterance(text);
      utterance.rate = rate;
      utterance.pitch = pitch;
      utterance.volume = volume;

      if (voice) {
        const selectedVoice = voices.find((v) => v.name === voice);
        if (selectedVoice) {
          utterance.voice = selectedVoice;
        }
      }

      utterance.onstart = () => {
        setIsSpeaking(true);
        setIsPaused(false);
        onStart?.();
      };

      utterance.onend = () => {
        setIsSpeaking(false);
        setIsPaused(false);
        utteranceRef.current = null;
        onEnd?.();
      };

      utterance.onerror = (event) => {
        setIsSpeaking(false);
        setIsPaused(false);
        onError?.(new Error(event.error));
      };

      utteranceRef.current = utterance;
      synthesis.speak(utterance);
    },
    [supported, rate, pitch, volume, voice, voices, onStart, onEnd, onError],
  );

  const cancel = useCallback(() => {
    if (!supported) return;
    window.speechSynthesis.cancel();
    setIsSpeaking(false);
    setIsPaused(false);
    pausePositionRef.current = 0;
  }, [supported]);

  const pause = useCallback(() => {
    if (!supported || !isSpeaking) return;
    window.speechSynthesis.pause();
    setIsPaused(true);
  }, [supported, isSpeaking]);

  const resume = useCallback(() => {
    if (!supported || !isPaused) return;
    window.speechSynthesis.resume();
    setIsPaused(false);
  }, [supported, isPaused]);

  return {
    speak,
    cancel,
    isSpeaking,
    isPaused,
    voices,
    supported,
    pause,
    resume,
  };
}

export function useSpeechRecognition() {
  const [transcript, setTranscript] = useState("");
  const [isListening, setIsListening] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const recognitionRef = useRef<any>(null);

  const supported =
    typeof window !== "undefined" &&
    ((window as any).SpeechRecognition ||
      (window as any).webkitSpeechRecognition);

  useEffect(() => {
    if (!supported) return;

    const SpeechRecognition =
      (window as any).SpeechRecognition ||
      (window as any).webkitSpeechRecognition;
    const recognition = new SpeechRecognition();

    recognition.continuous = true;
    recognition.interimResults = true;
    recognition.lang = "en-US";

    recognition.onresult = (event: any) => {
      let interimTranscript = "";
      let finalTranscript = "";

      for (let i = event.resultIndex; i < event.results.length; i++) {
        const result = event.results[i];
        if (result.isFinal) {
          finalTranscript += result[0].transcript;
        } else {
          interimTranscript += result[0].transcript;
        }
      }

      setTranscript(finalTranscript || interimTranscript);
    };

    recognition.onerror = (event: any) => {
      setError(event.error);
      setIsListening(false);
    };

    recognition.onend = () => {
      setIsListening(false);
    };

    recognitionRef.current = recognition;
  }, [supported]);

  const startListening = useCallback(() => {
    if (!supported || !recognitionRef.current) return;
    setTranscript("");
    setError(null);
    setIsListening(true);
    recognitionRef.current.start();
  }, [supported]);

  const stopListening = useCallback(() => {
    if (!supported || !recognitionRef.current) return;
    recognitionRef.current.stop();
    setIsListening(false);
  }, [supported]);

  return {
    transcript,
    isListening,
    error,
    supported,
    startListening,
    stopListening,
  };
}
