export interface User {
  id: number;
  email: string;
  username: string;
  displayName: string;
  avatarUrl?: string;
  createdAt: string;
}

export interface RegisterData {
  email: string;
  password: string;
  username: string;
  displayName?: string;
}

export interface ApiAuthResponse {
  success: boolean;
  message?: string;
  access_token?: string;
  token_type?: string;
  expires_in?: number;
  user?: {
    username: string;
    email: string;
    display_name: string;
    id: number;
    is_active: boolean;
    is_verified: boolean;
    created_at: string;
    last_login: string | null;
    github_username: string | null;
    github_avatar_url: string | null;
  };
}

export interface AuthResponse {
  access_token: string;
  user: User;
}

export interface Exercise {
  id: number;
  title: string;
  description: string;
  difficulty: 1 | 2 | 3 | 4 | 5;
  category: string;
  concepts: string[];
  template_code?: string;
  hints?: string[];
  bonus?: string;
}

export interface ExerciseDetail extends Exercise {
  template_code: string;
  hints: string[];
  prerequisites?: number[];
}

export interface ExerciseProgress {
  status: "not_started" | "in_progress" | "completed";
  attempts: number;
  completedAt?: string;
  bestExecutionTimeMs?: number;
  xpEarned: number;
}

export interface RunResult {
  success: boolean;
  output: string;
  error: string | null;
  executionTimeMs: number;
  timedOut?: boolean;
  compilationError?: boolean;
}

export interface SubmitResult {
  success: boolean;
  xpEarned: number;
  output: string;
  error?: string;
  executionTimeMs: number;
}

export interface Category {
  id: string;
  name: string;
  count: number;
  exercises: { id: number; title: string }[];
}

export interface Difficulty {
  level: number;
  count: number;
  label: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    cursor?: string | null;
    next_cursor?: string | number | null;
    limit: number;
    has_more: boolean;
    total: number;
  };
  filters?: {
    difficulty?: number;
    category?: string;
    search?: string;
  };
}

export interface ConsoleEntry {
  id: string;
  type: "info" | "success" | "error" | "warning" | "output";
  message: string;
  timestamp: number;
}

export interface Rank {
  tier: string;
  name: string;
  icon: string;
  color: string;
  minXp: number;
  maxXp?: number;
}

export interface Achievement {
  id: number;
  key: string;
  name: string;
  description: string;
  icon: string;
  earnedAt?: string;
}

export interface UserProfile {
  user: User;
  totalXp: number;
  currentRank: Rank;
  currentTitle?: string;
  streakDays: number;
  longestStreak: number;
  treats: number;
  problemsSolved: number;
  achievements: Achievement[];
}

export interface ApiError {
  message?: string;
  error?: string;
  code?: string;
  status?: number;
}
