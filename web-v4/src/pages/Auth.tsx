import { useState } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useAuthStore } from "../stores/auth";
import {
  Terminal,
  Mail,
  Lock,
  User,
  ArrowRight,
  Loader2,
  Github,
} from "lucide-react";

const loginSchema = z.object({
  email: z.string().email("Invalid email address"),
  password: z.string().min(8, "Password must be at least 8 characters"),
});

const registerSchema = z
  .object({
    username: z
      .string()
      .min(3, "Username must be at least 3 characters")
      .max(20, "Username must be less than 20 characters"),
    email: z.string().email("Invalid email address"),
    password: z.string().min(8, "Password must be at least 8 characters"),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ["confirmPassword"],
  });

type LoginForm = z.infer<typeof loginSchema>;
type RegisterForm = z.infer<typeof registerSchema>;

export default function AuthPage() {
  const [isLogin, setIsLogin] = useState(true);
  const [generalError, setGeneralError] = useState("");
  const { login, register: registerUser, isLoading } = useAuthStore();
  const navigate = useNavigate();
  const location = useLocation();

  // SECURITY FIX: Validate redirect URL to prevent open redirect (H-04)
  const getSafeRedirect = (url: string): string => {
    // Only allow relative URLs or same-origin URLs
    if (!url || url.startsWith('http') || url.startsWith('//')) {
      return "/";
    }
    // Ensure URL starts with /
    if (!url.startsWith('/')) {
      return "/";
    }
    return url;
  };

  const from = getSafeRedirect(location.state?.from?.pathname || "/");

  const loginForm = useForm<LoginForm>({
    resolver: zodResolver(loginSchema),
  });

  const registerForm = useForm<RegisterForm>({
    resolver: zodResolver(registerSchema),
  });

  const handleLogin = async (data: LoginForm) => {
    setGeneralError("");
    try {
      await login(data.email, data.password);
      navigate(from, { replace: true });
    } catch (error) {
      setGeneralError(error instanceof Error ? error.message : "Login failed");
    }
  };

  const handleRegister = async (data: RegisterForm) => {
    setGeneralError("");
    try {
      await registerUser(data);
      navigate(from, { replace: true });
    } catch (error) {
      setGeneralError(
        error instanceof Error ? error.message : "Registration failed",
      );
    }
  };

  return (
    <div className="min-h-screen bg-canvas flex">
      <div className="hidden lg:flex lg:w-1/2 bg-gradient-to-br from-primary/20 via-surface to-purple-500/10 p-12 flex-col justify-between">
        <div>
          <div className="flex items-center gap-3">
            <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-primary to-primary-hover flex items-center justify-center shadow-glow">
              <Terminal className="w-7 h-7 text-white" />
            </div>
            <span className="font-display font-bold text-xl text-text-primary">
              Rust Learning Ground
            </span>
          </div>
        </div>

        <div className="space-y-8">
          <h1 className="text-4xl lg:text-5xl font-display font-bold text-text-primary leading-tight">
            Master Rust
            <br />
            <span className="bg-gradient-to-r from-primary to-orange-400 bg-clip-text text-transparent">
              The Right Way
            </span>
          </h1>
          <p className="text-lg text-text-secondary max-w-md">
            Interactive exercises, instant feedback, and a gamified learning
            experience designed for modern developers.
          </p>

          <div className="grid grid-cols-2 gap-4">
            {[
              { label: "280+ Exercises", icon: "📚" },
              { label: "Instant Feedback", icon: "⚡" },
              { label: "Streak System", icon: "🔥" },
              { label: "Achievements", icon: "🏆" },
            ].map((item, i) => (
              <div
                key={i}
                className="p-4 rounded-xl bg-surface/50 backdrop-blur border border-border-subtle"
              >
                <span className="text-2xl mb-2 block">{item.icon}</span>
                <span className="text-text-secondary">{item.label}</span>
              </div>
            ))}
          </div>
        </div>

        <p className="text-text-muted text-sm">
          © 2024 Rust Learning Ground. All rights reserved.
        </p>
      </div>

      <div className="flex-1 flex items-center justify-center p-8">
        <div className="w-full max-w-md space-y-8">
          <div className="text-center lg:hidden mb-8">
            <div className="flex items-center justify-center gap-3 mb-4">
              <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-primary to-primary-hover flex items-center justify-center">
                <Terminal className="w-7 h-7 text-white" />
              </div>
              <span className="font-display font-bold text-xl text-text-primary">
                Rust Learning Ground
              </span>
            </div>
          </div>

          <div className="text-center">
            <h2 className="text-2xl font-display font-bold text-text-primary">
              {isLogin ? "Welcome back" : "Create your account"}
            </h2>
            <p className="text-text-secondary mt-2">
              {isLogin
                ? "Sign in to continue your Rust journey"
                : "Start your Rust learning adventure today"}
            </p>
          </div>

          {generalError && (
            <div className="p-4 rounded-xl bg-error/10 border border-error/20 text-error text-sm">
              {generalError}
            </div>
          )}

          <form
            onSubmit={
              isLogin
                ? loginForm.handleSubmit(handleLogin)
                : registerForm.handleSubmit(handleRegister)
            }
            className="space-y-4"
          >
            {!isLogin && (
              <div>
                <label className="block text-sm font-medium text-text-secondary mb-2">
                  Username
                </label>
                <div className="relative">
                  <User className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-text-muted" />
                  <input
                    {...registerForm.register("username")}
                    type="text"
                    placeholder="your_username"
                    className="w-full pl-10 pr-4 py-3 rounded-xl bg-surface border border-border-subtle text-text-primary placeholder-text-muted focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all"
                  />
                </div>
                {registerForm.formState.errors.username && (
                  <p className="mt-1 text-sm text-error">
                    {registerForm.formState.errors.username.message}
                  </p>
                )}
              </div>
            )}

            <div>
              <label className="block text-sm font-medium text-text-secondary mb-2">
                Email
              </label>
              <div className="relative">
                <Mail className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-text-muted" />
                <input
                  {...registerForm.register(isLogin ? "email" : "email")}
                  type="email"
                  placeholder="you@example.com"
                  className="w-full pl-10 pr-4 py-3 rounded-xl bg-surface border border-border-subtle text-text-primary placeholder-text-muted focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all"
                />
              </div>
              {(loginForm.formState.errors.email ||
                registerForm.formState.errors.email) && (
                <p className="mt-1 text-sm text-error">
                  {loginForm.formState.errors.email?.message ||
                    registerForm.formState.errors.email?.message}
                </p>
              )}
            </div>

            <div>
              <label className="block text-sm font-medium text-text-secondary mb-2">
                Password
              </label>
              <div className="relative">
                <Lock className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-text-muted" />
                <input
                  {...registerForm.register(isLogin ? "password" : "password")}
                  type="password"
                  placeholder="••••••••"
                  className="w-full pl-10 pr-4 py-3 rounded-xl bg-surface border border-border-subtle text-text-primary placeholder-text-muted focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all"
                />
              </div>
              {(loginForm.formState.errors.password ||
                registerForm.formState.errors.password) && (
                <p className="mt-1 text-sm text-error">
                  {loginForm.formState.errors.password?.message ||
                    registerForm.formState.errors.password?.message}
                </p>
              )}
            </div>

            {!isLogin && (
              <div>
                <label className="block text-sm font-medium text-text-secondary mb-2">
                  Confirm Password
                </label>
                <div className="relative">
                  <Lock className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-text-muted" />
                  <input
                    {...registerForm.register("confirmPassword")}
                    type="password"
                    placeholder="••••••••"
                    className="w-full pl-10 pr-4 py-3 rounded-xl bg-surface border border-border-subtle text-text-primary placeholder-text-muted focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all"
                  />
                </div>
                {registerForm.formState.errors.confirmPassword && (
                  <p className="mt-1 text-sm text-error">
                    {registerForm.formState.errors.confirmPassword.message}
                  </p>
                )}
              </div>
            )}

            <button
              type="submit"
              disabled={isLoading}
              className="w-full flex items-center justify-center gap-2 py-3 px-4 rounded-xl bg-primary text-white font-semibold hover:bg-primary-hover transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-glow hover:shadow-glow-lg"
            >
              {isLoading ? (
                <Loader2 className="w-5 h-5 animate-spin" />
              ) : (
                <>
                  {isLogin ? "Sign In" : "Create Account"}
                  <ArrowRight className="w-5 h-5" />
                </>
              )}
            </button>
          </form>

          <div className="relative">
            <div className="absolute inset-0 flex items-center">
              <div className="w-full border-t border-border-subtle"></div>
            </div>
            <div className="relative flex justify-center text-sm">
              <span className="px-4 bg-canvas text-text-muted">
                Or continue with
              </span>
            </div>
          </div>

          <button className="w-full flex items-center justify-center gap-2 py-3 px-4 rounded-xl bg-surface border border-border-subtle text-text-primary font-medium hover:bg-surface-hover transition-all">
            <Github className="w-5 h-5" />
            GitHub
          </button>

          <p className="text-center text-text-secondary">
            {isLogin ? "Don't have an account?" : "Already have an account?"}{" "}
            <button
              onClick={() => {
                setIsLogin(!isLogin);
                setGeneralError("");
                loginForm.reset();
                registerForm.reset();
              }}
              className="text-primary font-medium hover:text-primary-hover transition-colors"
            >
              {isLogin ? "Sign up" : "Sign in"}
            </button>
          </p>
        </div>
      </div>
    </div>
  );
}
