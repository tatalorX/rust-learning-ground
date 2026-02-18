/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // Background colors - Deep space-inspired
        canvas: "#0a0a0f",
        surface: "#12121a",
        surfaceHover: "#1a1a24",
        elevated: "#1e1e2a",
        floating: "#252532",

        // Rust-inspired orange accent
        primary: {
          DEFAULT: "#f74c00",
          hover: "#ff6d24",
          light: "#ff8555",
          dark: "#d43c00",
        },

        // Semantic colors
        success: "#10b981",
        warning: "#f59e0b",
        error: "#ef4444",
        info: "#3b82f6",

        // Text colors
        text: {
          primary: "#f0f0f5",
          secondary: "#a0a0b0",
          muted: "#6b6b7b",
          inverse: "#0a0a0f",
        },

        // Border colors
        border: {
          subtle: "#2a2a3a",
          DEFAULT: "#3a3a4a",
          strong: "#4a4a5a",
        },

        // Glow effects
        glow: {
          primary: "rgba(247, 76, 0, 0.5)",
          success: "rgba(16, 185, 129, 0.5)",
          error: "rgba(239, 68, 68, 0.5)",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        display: ["JetBrains Mono", "monospace"],
        mono: ["JetBrains Mono", "Fira Code", "monospace"],
      },
      animation: {
        "fade-in": "fadeIn 0.2s ease-out",
        "slide-up": "slideUp 0.3s ease-out",
        "slide-down": "slideDown 0.3s ease-out",
        "scale-in": "scaleIn 0.2s ease-out",
        "pulse-glow": "pulseGlow 2s ease-in-out infinite",
        "spin-slow": "spin 3s linear infinite",
      },
      keyframes: {
        fadeIn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
        },
        slideUp: {
          "0%": { transform: "translateY(10px)", opacity: "0" },
          "100%": { transform: "translateY(0)", opacity: "1" },
        },
        slideDown: {
          "0%": { transform: "translateY(-10px)", opacity: "0" },
          "100%": { transform: "translateY(0)", opacity: "1" },
        },
        scaleIn: {
          "0%": { transform: "scale(0.95)", opacity: "0" },
          "100%": { transform: "scale(1)", opacity: "1" },
        },
        pulseGlow: {
          "0%, 100%": { boxShadow: "0 0 20px rgba(247, 76, 0, 0.3)" },
          "50%": { boxShadow: "0 0 40px rgba(247, 76, 0, 0.6)" },
        },
      },
      boxShadow: {
        glow: "0 0 20px rgba(247, 76, 0, 0.3)",
        "glow-sm": "0 0 10px rgba(247, 76, 0, 0.2)",
        "glow-lg": "0 0 40px rgba(247, 76, 0, 0.4)",
      },
    },
  },
  plugins: [],
};
