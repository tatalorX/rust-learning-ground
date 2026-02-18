import { Outlet, Link, useLocation } from "react-router-dom";
import { useAuthStore } from "../../stores/auth";
import { useEngagementStore } from "../../stores/engagement";
import { XPPopup, MiniXP, MiniFocusTimer, SoundManager } from "../engagement";
import {
  NotificationToast,
  NotificationBell,
} from "../notifications/NotificationToast";
import Footer from "./Footer";
import {
  Terminal,
  BookOpen,
  User,
  LogOut,
  Menu,
  X,
  Flame,
  Zap,
  Settings,
  Trophy,
  Award,
  Book,
  Calendar,
  FileText,
} from "lucide-react";
import { useState } from "react";
import { LanguageSelector } from "../LanguageSelector";

export default function Layout() {
  const { user, isAuthenticated, logout } = useAuthStore();
  const engagementStore = useEngagementStore();
  const location = useLocation();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const navItems = [
    { path: "/", label: "Home", icon: Terminal },
    { path: "/exercises", label: "Exercises", icon: BookOpen },
    { path: "/book", label: "Rust Book", icon: Book },
    { path: "/events", label: "Events", icon: Calendar },
    { path: "/leaderboard", label: "Leaderboard", icon: Trophy },
    { path: "/achievements", label: "Achievements", icon: Award },
    { path: "/docs", label: "Docs", icon: FileText },
  ];

  const isActive = (path: string) => {
    if (path === "/") return location.pathname === "/";
    return location.pathname.startsWith(path);
  };

  return (
    <div className="min-h-screen bg-canvas">
      <SoundManager enabled={engagementStore.soundsEnabled} />
      <header className="sticky top-0 z-50 bg-surface/80 backdrop-blur-lg border-b border-border-subtle">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center gap-8">
              <Link to="/" className="flex items-center gap-2 group">
                <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-primary to-primary-hover flex items-center justify-center shadow-glow transition-all group-hover:shadow-glow-lg">
                  <Terminal className="w-6 h-6 text-white" />
                </div>
                <span className="hidden sm:block font-display font-bold text-lg text-text-primary">
                  Rust Learning Ground
                </span>
              </Link>

              <nav className="hidden md:flex items-center gap-1">
                {navItems.map((item) => (
                  <Link
                    key={item.path}
                    to={item.path}
                    className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                      isActive(item.path)
                        ? "bg-primary/10 text-primary"
                        : "text-text-secondary hover:text-text-primary hover:bg-surface-hover"
                    }`}
                  >
                    <item.icon className="w-4 h-4" />
                    {item.label}
                  </Link>
                ))}
              </nav>
            </div>

            <div className="flex items-center gap-4">
              {isAuthenticated ? (
                <>
                  {engagementStore.xpEnabled && (
                    <MiniXP amount={engagementStore.xp} />
                  )}

                  {engagementStore.streakEnabled && (
                    <div className="hidden sm:flex items-center gap-3 px-3 py-1.5 rounded-full bg-gradient-to-r from-orange-500/20 to-red-500/20 border border-orange-500/30">
                      <Flame className="w-4 h-4 text-orange-500" />
                      <span className="text-sm font-medium text-orange-400">
                        {engagementStore.streak} day streak
                      </span>
                    </div>
                  )}

                  <NotificationBell />

                  <LanguageSelector />

                  <Link
                    to="/settings"
                    className="p-2 rounded-lg text-text-secondary hover:text-text-primary hover:bg-surface-hover transition-all"
                    title="Settings"
                  >
                    <Settings className="w-5 h-5" />
                  </Link>

                  <Link
                    to="/profile"
                    className="flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium text-text-secondary hover:text-text-primary hover:bg-surface-hover transition-all"
                  >
                    <div className="w-8 h-8 rounded-full bg-gradient-to-br from-primary to-purple-500 flex items-center justify-center text-white font-bold text-sm">
                      {user?.username?.charAt(0).toUpperCase() || "U"}
                    </div>
                    <span className="hidden lg:block">{user?.username}</span>
                  </Link>

                  <button
                    onClick={() => logout()}
                    className="p-2 rounded-lg text-text-secondary hover:text-error hover:bg-error/10 transition-all"
                    title="Logout"
                  >
                    <LogOut className="w-5 h-5" />
                  </button>
                </>
              ) : (
                <Link
                  to="/auth"
                  className="px-4 py-2 rounded-lg bg-primary text-white text-sm font-medium hover:bg-primary-hover transition-all shadow-glow hover:shadow-glow-lg"
                >
                  Sign In
                </Link>
              )}

              <button
                className="md:hidden p-2 rounded-lg text-text-secondary hover:text-text-primary hover:bg-surface-hover"
                onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              >
                {mobileMenuOpen ? (
                  <X className="w-5 h-5" />
                ) : (
                  <Menu className="w-5 h-5" />
                )}
              </button>
            </div>
          </div>
        </div>

        {mobileMenuOpen && (
          <div className="md:hidden border-t border-border-subtle bg-surface animate-in">
            <nav className="px-4 py-4 space-y-2">
              {navItems.map((item) => (
                <Link
                  key={item.path}
                  to={item.path}
                  onClick={() => setMobileMenuOpen(false)}
                  className={`flex items-center gap-3 px-4 py-3 rounded-lg text-base font-medium ${
                    isActive(item.path)
                      ? "bg-primary/10 text-primary"
                      : "text-text-secondary hover:text-text-primary hover:bg-surface-hover"
                  }`}
                >
                  <item.icon className="w-5 h-5" />
                  {item.label}
                </Link>
              ))}
            </nav>
          </div>
        )}
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <Outlet />
      </main>

      <Footer />

      {/* Fixed position focus timer - only shown when enabled */}
      {engagementStore.focusTimerEnabled && <MiniFocusTimer enabled={true} />}

      {/* Notification toasts */}
      <NotificationToast />
    </div>
  );
}
