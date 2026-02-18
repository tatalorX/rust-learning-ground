import { useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import {
  Terminal,
  BookOpen,
  Flame,
  Trophy,
  Zap,
  ArrowRight,
  CheckCircle2,
  Star,
  Play,
  Youtube,
  Gamepad2,
} from "lucide-react";
import { motion } from "framer-motion";

export default function HomePage() {
  const { t } = useTranslation();

  return (
    <div className="space-y-16">
      {/* Hero Section with Video */}
      <section className="text-center space-y-6 animate-in">
        <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-primary/10 border border-primary/20 text-primary text-sm font-medium">
          <Star className="w-4 h-4" />
          <span>{t("home.hero.badge")}</span>
        </div>

        <h1 className="text-4xl sm:text-5xl lg:text-6xl font-display font-bold text-text-primary leading-tight">
          {t("home.hero.title")}{" "}
          <span className="bg-gradient-to-r from-primary to-orange-400 bg-clip-text text-transparent">
            {t("home.hero.titleHighlight")}
          </span>
          <br />
          {t("home.hero.subtitle")}
        </h1>

        <p className="max-w-2xl mx-auto text-lg text-text-secondary">
          {t("home.hero.description")}
        </p>

        <div className="flex flex-col sm:flex-row items-center justify-center gap-4 pt-4">
          <Link
            to="/exercises"
            className="group flex items-center gap-2 px-8 py-4 rounded-xl bg-primary text-white font-semibold text-lg hover:bg-primary-hover transition-all shadow-glow hover:shadow-glow-lg"
          >
            <Terminal className="w-5 h-5" />
            {t("home.hero.startLearning")}
            <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
          </Link>
          <Link
            to="/auth"
            className="flex items-center gap-2 px-8 py-4 rounded-xl bg-surface-hover border border-border text-text-secondary font-semibold text-lg hover:text-text-primary hover:border-border-strong transition-all"
          >
            {t("home.hero.createAccount")}
          </Link>
        </div>
      </section>

      {/* Video Section - Youth Appeal */}
      <section className="relative overflow-hidden rounded-3xl bg-gradient-to-br from-surface via-surface to-surface-hover border border-border-subtle">
        <div className="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=1600&q=80')] opacity-10 bg-cover bg-center" />
        <div className="relative p-8 lg:p-12">
          <div className="grid lg:grid-cols-2 gap-8 items-center">
            <div>
              <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-red-500/10 border border-red-500/20 text-red-400 text-sm font-medium mb-4">
                <Youtube className="w-4 h-4" />
                Watch & Learn
              </div>
              <h2 className="text-3xl font-display font-bold text-text-primary mb-4">
                Learn Rust by Building Games
              </h2>
              <p className="text-text-secondary mb-6">
                Watch our video tutorials and see how Rust powers modern game
                development. From 2D platformers to 3D engines, learn through
                exciting projects.
              </p>
              <div className="flex flex-wrap gap-3">
                <span className="px-3 py-1 rounded-full bg-primary/10 text-primary text-sm">
                  <Gamepad2 className="w-4 h-4 inline mr-1" />
                  Game Dev
                </span>
                <span className="px-3 py-1 rounded-full bg-success/10 text-success text-sm">
                  <Zap className="w-4 h-4 inline mr-1" />
                  High Performance
                </span>
                <span className="px-3 py-1 rounded-full bg-orange-500/10 text-orange-400 text-sm">
                  <Flame className="w-4 h-4 inline mr-1" />
                  Hot Trend
                </span>
              </div>
            </div>

            {/* Video Grid */}
            <div className="grid grid-cols-2 gap-4">
              <motion.a
                href="https://www.youtube.com/watch?v=ygL_xcavzQ4"
                target="_blank"
                rel="noopener noreferrer"
                whileHover={{ scale: 1.02 }}
                className="group relative aspect-video rounded-xl overflow-hidden bg-canvas border border-border-subtle hover:border-primary/50 transition-all"
              >
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <div className="w-12 h-12 rounded-full bg-primary/90 flex items-center justify-center group-hover:scale-110 transition-transform">
                    <Play className="w-5 h-5 text-white ml-1" />
                  </div>
                </div>
                <div className="absolute bottom-3 left-3 right-3">
                  <p className="text-white font-medium text-sm line-clamp-2">
                    Rust in 100 Seconds
                  </p>
                  <p className="text-white/70 text-xs">Fireship • 2:24</p>
                </div>
              </motion.a>

              <motion.a
                href="https://www.youtube.com/watch?v=zF34dRivLOw"
                target="_blank"
                rel="noopener noreferrer"
                whileHover={{ scale: 1.02 }}
                className="group relative aspect-video rounded-xl overflow-hidden bg-canvas border border-border-subtle hover:border-primary/50 transition-all"
              >
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <div className="w-12 h-12 rounded-full bg-primary/90 flex items-center justify-center group-hover:scale-110 transition-transform">
                    <Play className="w-5 h-5 text-white ml-1" />
                  </div>
                </div>
                <div className="absolute bottom-3 left-3 right-3">
                  <p className="text-white font-medium text-sm line-clamp-2">
                    Rust for Beginners
                  </p>
                  <p className="text-white/70 text-xs">
                    Traversy Media • 1:12:45
                  </p>
                </div>
              </motion.a>

              <motion.a
                href="https://www.youtube.com/watch?v=ygL_xcavzQ4"
                target="_blank"
                rel="noopener noreferrer"
                whileHover={{ scale: 1.02 }}
                className="group relative aspect-video rounded-xl overflow-hidden bg-canvas border border-border-subtle hover:border-primary/50 transition-all"
              >
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <div className="w-12 h-12 rounded-full bg-primary/90 flex items-center justify-center group-hover:scale-110 transition-transform">
                    <Play className="w-5 h-5 text-white ml-1" />
                  </div>
                </div>
                <div className="absolute bottom-3 left-3 right-3">
                  <p className="text-white font-medium text-sm line-clamp-2">
                    Build a Game in Rust
                  </p>
                  <p className="text-white/70 text-xs">
                    Let's Get Rusty • 45:30
                  </p>
                </div>
              </motion.a>

              <motion.a
                href="https://www.youtube.com/watch?v=5C_HPTP5ueU"
                target="_blank"
                rel="noopener noreferrer"
                whileHover={{ scale: 1.02 }}
                className="group relative aspect-video rounded-xl overflow-hidden bg-canvas border border-border-subtle hover:border-primary/50 transition-all"
              >
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-black/20 to-transparent" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <div className="w-12 h-12 rounded-full bg-primary/90 flex items-center justify-center group-hover:scale-110 transition-transform">
                    <Play className="w-5 h-5 text-white ml-1" />
                  </div>
                </div>
                <div className="absolute bottom-3 left-3 right-3">
                  <p className="text-white font-medium text-sm line-clamp-2">
                    Rust Crash Course
                  </p>
                  <p className="text-white/70 text-xs">
                    freeCodeCamp • 2:30:15
                  </p>
                </div>
              </motion.a>
            </div>
          </div>
        </div>
      </section>

      {/* Features Grid */}
      <section className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
        {[
          {
            icon: BookOpen,
            label: t("home.features.exercises.title"),
            desc: t("home.features.exercises.description"),
            color: "from-blue-500 to-cyan-500",
          },
          {
            icon: Terminal,
            label: t("home.features.feedback.title"),
            desc: t("home.features.feedback.description"),
            color: "from-primary to-orange-500",
          },
          {
            icon: Flame,
            label: t("home.features.streak.title"),
            desc: t("home.features.streak.description"),
            color: "from-red-500 to-orange-500",
          },
          {
            icon: Trophy,
            label: t("home.features.achievements.title"),
            desc: t("home.features.achievements.description"),
            color: "from-yellow-500 to-amber-500",
          },
        ].map((feature, i) => (
          <div
            key={i}
            className="group p-6 rounded-2xl bg-surface border border-border-subtle hover:border-primary/50 transition-all hover:shadow-glow"
          >
            <div
              className={`w-12 h-12 rounded-xl bg-gradient-to-br ${feature.color} flex items-center justify-center mb-4`}
            >
              <feature.icon className="w-6 h-6 text-white" />
            </div>
            <h3 className="text-lg font-semibold text-text-primary mb-2">
              {feature.label}
            </h3>
            <p className="text-text-secondary">{feature.desc}</p>
          </div>
        ))}
      </section>

      {/* Interactive Code Section */}
      <section className="grid lg:grid-cols-2 gap-12 items-center">
        <div className="space-y-6">
          <h2 className="text-3xl font-display font-bold text-text-primary">
            {t("home.editor.title")}
          </h2>
          <p className="text-text-secondary text-lg">
            {t("home.editor.description")}
          </p>
          <ul className="space-y-3">
            {(
              t("home.editor.features", { returnObjects: true }) as string[]
            ).map((item: string, i: number) => (
              <li
                key={i}
                className="flex items-center gap-3 text-text-secondary"
              >
                <CheckCircle2 className="w-5 h-5 text-success" />
                {item}
              </li>
            ))}
          </ul>
          <Link
            to="/exercises/1"
            className="inline-flex items-center gap-2 text-primary font-medium hover:text-primary-hover transition-colors"
          >
            {t("home.editor.tryExercise")} <ArrowRight className="w-4 h-4" />
          </Link>
        </div>

        <div className="rounded-2xl overflow-hidden border border-border-subtle shadow-2xl">
          <div className="bg-surface p-4 border-b border-border-subtle flex items-center gap-2">
            <div className="w-3 h-3 rounded-full bg-red-500" />
            <div className="w-3 h-3 rounded-full bg-yellow-500" />
            <div className="w-3 h-3 rounded-full bg-green-500" />
            <span className="ml-2 text-sm text-text-muted font-mono">
              main.rs
            </span>
          </div>
          <div className="bg-canvas p-6 font-mono text-sm overflow-x-auto">
            <pre className="text-text-secondary">
              {`fn main() {
    println!("Hello, Rust!");

    // Variables and mutability
    let x = 5;
    let mut y = 10;
    
    y = x + y;
    println!("Result: {}", y);
}`}
            </pre>
          </div>
        </div>
      </section>

      {/* Why Rust Section */}
      <section className="bg-gradient-to-br from-primary/5 to-purple-500/5 rounded-3xl p-8 lg:p-12 border border-primary/10">
        <div className="text-center mb-8">
          <h2 className="text-3xl font-display font-bold text-text-primary mb-4">
            {t("home.whyRust.title")}
          </h2>
          <p className="text-text-secondary max-w-2xl mx-auto">
            {t("home.whyRust.description")}
          </p>
        </div>

        <div className="grid sm:grid-cols-3 gap-8">
          {(
            t("home.whyRust.features", { returnObjects: true }) as {
              title: string;
              description: string;
            }[]
          ).map(
            (feature: { title: string; description: string }, i: number) => (
              <div key={i} className="text-center">
                <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-primary/20 to-purple-500/20 flex items-center justify-center mx-auto mb-4">
                  <Zap className="w-8 h-8 text-primary" />
                </div>
                <h3 className="text-lg font-semibold text-text-primary mb-2">
                  {feature.title}
                </h3>
                <p className="text-text-secondary">{feature.description}</p>
              </div>
            ),
          )}
        </div>
      </section>

      {/* CTA Section */}
      <section className="text-center">
        <h2 className="text-3xl font-display font-bold text-text-primary mb-4">
          {t("home.cta.title")}
        </h2>
        <p className="text-text-secondary mb-8">{t("home.cta.description")}</p>
        <Link
          to="/exercises"
          className="inline-flex items-center gap-2 px-8 py-4 rounded-xl bg-primary text-white font-semibold text-lg hover:bg-primary-hover transition-all shadow-glow hover:shadow-glow-lg"
        >
          <Terminal className="w-5 h-5" />
          {t("home.cta.button")}
        </Link>
      </section>
    </div>
  );
}
