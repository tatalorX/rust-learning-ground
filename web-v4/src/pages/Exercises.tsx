import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { useExercisesStore } from "../stores/exercises";
import { useProgressStore } from "../stores/progress";
import { exerciseApi } from "../services/exercises";
import {
  Search,
  Filter,
  ChevronRight,
  Loader2,
  Code2,
  Sparkles,
  Lock,
  CheckCircle2,
  Circle,
  Play,
  Clock,
  Trophy,
} from "lucide-react";
import type { Category, Difficulty } from "../types";
import { motion } from "framer-motion";

const difficultyColors = {
  1: "from-green-500 to-emerald-500",
  2: "from-lime-500 to-yellow-500",
  3: "from-yellow-500 to-orange-500",
  4: "from-orange-500 to-red-500",
  5: "from-red-500 to-pink-500",
};

const difficultyLabels = {
  1: "Beginner",
  2: "Easy",
  3: "Medium",
  4: "Hard",
  5: "Expert",
};

export default function ExercisesPage() {
  const {
    exercises,
    isLoadingList,
    hasMore,
    filters,
    fetchExercises,
    setFilters,
  } = useExercisesStore();
  const { exercises: progressExercises, getStats } = useProgressStore();

  const [categories, setCategories] = useState<Category[]>([]);
  const [difficulties, setDifficulties] = useState<Difficulty[]>([]);
  const [showFilters, setShowFilters] = useState(false);
  const [searchValue, setSearchValue] = useState("");

  const stats = getStats();

  useEffect(() => {
    const loadData = async () => {
      try {
        const [cats, diffs] = await Promise.all([
          exerciseApi.getCategories(),
          exerciseApi.getDifficulties(),
        ]);
        setCategories(cats.categories);
        setDifficulties(diffs.difficulties);
      } catch (error) {
        console.error("Failed to load filters:", error);
      }
    };
    loadData();
  }, []);

  useEffect(() => {
    fetchExercises(true);
  }, [fetchExercises]);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setFilters({ search: searchValue });
  };

  const handleDifficultyFilter = (level: number | undefined) => {
    setFilters({ difficulty: level });
  };

  const handleCategoryFilter = (categoryId: string | undefined) => {
    setFilters({ category: categoryId });
  };

  return (
    <div className="space-y-8">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-3xl font-display font-bold text-text-primary">
            Exercises
          </h1>
          <p className="text-text-secondary mt-1">
            {exercises.length} exercises available • {stats.totalCompleted}{" "}
            completed ({stats.completionRate}%)
          </p>
        </div>

        <form onSubmit={handleSearch} className="flex items-center gap-2">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-text-muted" />
            <input
              type="text"
              placeholder="Search exercises..."
              value={searchValue}
              onChange={(e) => setSearchValue(e.target.value)}
              className="pl-10 pr-4 py-2.5 rounded-xl bg-surface border border-border-subtle text-text-primary placeholder-text-muted focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all w-64"
            />
          </div>
          <button
            type="button"
            onClick={() => setShowFilters(!showFilters)}
            className={`p-2.5 rounded-xl border transition-all ${
              showFilters
                ? "bg-primary/10 border-primary/50 text-primary"
                : "bg-surface border-border-subtle text-text-secondary hover:text-text-primary"
            }`}
          >
            <Filter className="w-5 h-5" />
          </button>
        </form>
      </div>

      {showFilters && (
        <div className="p-6 rounded-2xl bg-surface border border-border-subtle animate-in">
          <div className="grid sm:grid-cols-2 gap-8">
            <div>
              <h3 className="text-sm font-medium text-text-secondary mb-3">
                Difficulty
              </h3>
              <div className="flex flex-wrap gap-2">
                <button
                  onClick={() => handleDifficultyFilter(undefined)}
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                    !filters.difficulty
                      ? "bg-primary text-white"
                      : "bg-surface-hover text-text-secondary hover:text-text-primary"
                  }`}
                >
                  All
                </button>
                {difficulties.map((diff) => (
                  <button
                    key={diff.level}
                    onClick={() => handleDifficultyFilter(diff.level)}
                    className={`px-4 py-2 rounded-lg text-sm font-medium transition-all flex items-center gap-2 ${
                      filters.difficulty === diff.level
                        ? "bg-primary text-white"
                        : "bg-surface-hover text-text-secondary hover:text-text-primary"
                    }`}
                  >
                    <span
                      className={`w-2 h-2 rounded-full bg-gradient-to-r ${difficultyColors[diff.level as keyof typeof difficultyColors]}`}
                    />
                    {
                      difficultyLabels[
                        diff.level as keyof typeof difficultyLabels
                      ]
                    }
                  </button>
                ))}
              </div>
            </div>

            <div>
              <h3 className="text-sm font-medium text-text-secondary mb-3">
                Category
              </h3>
              <div className="flex flex-wrap gap-2">
                <button
                  onClick={() => handleCategoryFilter(undefined)}
                  className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                    !filters.category
                      ? "bg-primary text-white"
                      : "bg-surface-hover text-text-secondary hover:text-text-primary"
                  }`}
                >
                  All
                </button>
                {categories.slice(0, 6).map((cat) => (
                  <button
                    key={cat.id}
                    onClick={() => handleCategoryFilter(cat.id)}
                    className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                      filters.category === cat.id
                        ? "bg-primary text-white"
                        : "bg-surface-hover text-text-secondary hover:text-text-primary"
                    }`}
                  >
                    {cat.name}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>
      )}

      <div className="grid gap-4">
        {isLoadingList && exercises.length === 0 ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-primary" />
          </div>
        ) : exercises.length > 0 ? (
          <>
            {exercises.map((exercise) => {
              const progress = progressExercises[exercise.id];
              const isCompleted = progress?.status === "completed";
              const isInProgress = progress?.status === "in_progress";

              return (
                <Link
                  key={exercise.id}
                  to={`/exercises/${exercise.id}`}
                  className={`group flex items-center gap-6 p-6 rounded-2xl border transition-all animate-in ${
                    isCompleted
                      ? "bg-success/5 border-success/30 hover:border-success/50"
                      : isInProgress
                        ? "bg-primary/5 border-primary/30 hover:border-primary/50"
                        : "bg-surface border-border-subtle hover:border-primary/30 hover:shadow-glow"
                  }`}
                >
                  <div
                    className={`flex-shrink-0 w-16 h-16 rounded-xl flex items-center justify-center ${
                      isCompleted
                        ? "bg-success/10"
                        : isInProgress
                          ? "bg-primary/10"
                          : "bg-gradient-to-br from-surface-hover to-surface"
                    }`}
                  >
                    {isCompleted ? (
                      <motion.div
                        initial={{ scale: 0 }}
                        animate={{ scale: 1 }}
                        className="w-10 h-10 rounded-full bg-success flex items-center justify-center"
                      >
                        <CheckCircle2 className="w-6 h-6 text-white" />
                      </motion.div>
                    ) : isInProgress ? (
                      <div className="w-10 h-10 rounded-full bg-primary flex items-center justify-center">
                        <Play className="w-5 h-5 text-white" />
                      </div>
                    ) : (
                      <Code2 className="w-8 h-8 text-primary" />
                    )}
                  </div>

                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-3 mb-1">
                      <span className="text-sm text-text-muted">
                        #{exercise.id}
                      </span>
                      <span
                        className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-gradient-to-r ${difficultyColors[exercise.difficulty as keyof typeof difficultyColors]} text-white`}
                      >
                        {
                          difficultyLabels[
                            exercise.difficulty as keyof typeof difficultyLabels
                          ]
                        }
                      </span>
                      <span className="px-2.5 py-1 rounded-full text-xs font-medium bg-surface-hover text-text-secondary capitalize">
                        {exercise.category.replace("_", " ")}
                      </span>
                      {isCompleted && (
                        <span className="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium bg-success/20 text-success">
                          <Trophy className="w-3 h-3" />
                          Completed
                        </span>
                      )}
                      {isInProgress && (
                        <span className="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium bg-primary/20 text-primary">
                          <Clock className="w-3 h-3" />
                          In Progress
                        </span>
                      )}
                    </div>
                    <h3 className="text-lg font-semibold text-text-primary group-hover:text-primary transition-colors">
                      {exercise.title}
                    </h3>
                    <p className="text-text-secondary text-sm mt-1 line-clamp-2">
                      {exercise.description}
                    </p>
                  </div>

                  <div className="flex items-center gap-2 text-text-muted">
                    {exercise.concepts.slice(0, 3).map((concept, i) => (
                      <span
                        key={i}
                        className="hidden sm:inline-flex items-center gap-1 px-2 py-1 rounded-md bg-surface-hover text-xs"
                      >
                        <Sparkles className="w-3 h-3" />
                        {concept}
                      </span>
                    ))}
                  </div>

                  <ChevronRight
                    className={`w-5 h-5 transition-all ${
                      isCompleted
                        ? "text-success group-hover:translate-x-1"
                        : "text-text-muted group-hover:text-primary group-hover:translate-x-1"
                    }`}
                  />
                </Link>
              );
            })}

            {hasMore && (
              <button
                onClick={() => fetchExercises(false)}
                disabled={isLoadingList}
                className="w-full py-4 rounded-xl bg-surface-hover text-text-secondary font-medium hover:text-text-primary transition-all flex items-center justify-center gap-2"
              >
                {isLoadingList ? (
                  <Loader2 className="w-5 h-5 animate-spin" />
                ) : (
                  "Load More"
                )}
              </button>
            )}
          </>
        ) : (
          <div className="text-center py-12">
            <Circle className="w-16 h-16 mx-auto text-text-muted mb-4" />
            <h3 className="text-lg font-semibold text-text-primary mb-2">
              No exercises found
            </h3>
            <p className="text-text-secondary">Try adjusting your filters</p>
          </div>
        )}
      </div>
    </div>
  );
}
