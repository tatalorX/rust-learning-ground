import { create } from "zustand";
import { persist } from "zustand/middleware";

interface BookProgress {
  chapterId: string;
  sectionId: string;
  completedSections: string[];
  totalSections: number;
  lastReadAt: string;
  quizScores: Record<string, number>; // quizId -> score
}

interface BookState {
  progress: Record<string, BookProgress>; // chapterId -> progress
  bookmarks: string[]; // sectionIds
  currentChapter: string | null;
  currentSection: string | null;

  // Actions
  startChapter: (chapterId: string, totalSections: number) => void;
  completeSection: (chapterId: string, sectionId: string) => void;
  saveQuizScore: (chapterId: string, quizId: string, score: number) => void;
  toggleBookmark: (sectionId: string) => void;
  setCurrentPosition: (chapterId: string, sectionId: string) => void;
  getChapterProgress: (chapterId: string) => number;
  getOverallProgress: () => number;
  isSectionCompleted: (chapterId: string, sectionId: string) => boolean;
}

export const useBookStore = create<BookState>()(
  persist(
    (set, get) => ({
      progress: {},
      bookmarks: [],
      currentChapter: null,
      currentSection: null,

      startChapter: (chapterId, totalSections) => {
        set((state) => ({
          progress: {
            ...state.progress,
            [chapterId]: {
              chapterId,
              sectionId: "",
              completedSections: [],
              totalSections,
              lastReadAt: new Date().toISOString(),
              quizScores: {},
            },
          },
        }));
      },

      completeSection: (chapterId, sectionId) => {
        set((state) => {
          const chapter = state.progress[chapterId];
          if (!chapter) return state;

          const alreadyCompleted =
            chapter.completedSections.includes(sectionId);
          return {
            progress: {
              ...state.progress,
              [chapterId]: {
                ...chapter,
                completedSections: alreadyCompleted
                  ? chapter.completedSections
                  : [...chapter.completedSections, sectionId],
                lastReadAt: new Date().toISOString(),
              },
            },
          };
        });
      },

      saveQuizScore: (chapterId, quizId, score) => {
        set((state) => {
          const chapter = state.progress[chapterId];
          if (!chapter) return state;

          return {
            progress: {
              ...state.progress,
              [chapterId]: {
                ...chapter,
                quizScores: {
                  ...chapter.quizScores,
                  [quizId]: score,
                },
              },
            },
          };
        });
      },

      toggleBookmark: (sectionId) => {
        set((state) => ({
          bookmarks: state.bookmarks.includes(sectionId)
            ? state.bookmarks.filter((id) => id !== sectionId)
            : [...state.bookmarks, sectionId],
        }));
      },

      setCurrentPosition: (chapterId, sectionId) => {
        set({
          currentChapter: chapterId,
          currentSection: sectionId,
        });
      },

      getChapterProgress: (chapterId) => {
        const chapter = get().progress[chapterId];
        if (!chapter || chapter.totalSections === 0) return 0;
        return Math.round(
          (chapter.completedSections.length / chapter.totalSections) * 100,
        );
      },

      getOverallProgress: () => {
        const chapters = Object.values(get().progress);
        if (chapters.length === 0) return 0;

        const totalSections = chapters.reduce(
          (sum, c) => sum + c.totalSections,
          0,
        );
        const completedSections = chapters.reduce(
          (sum, c) => sum + c.completedSections.length,
          0,
        );

        return totalSections > 0
          ? Math.round((completedSections / totalSections) * 100)
          : 0;
      },

      isSectionCompleted: (chapterId, sectionId) => {
        return (
          get().progress[chapterId]?.completedSections.includes(sectionId) ||
          false
        );
      },
    }),
    {
      name: "book-progress",
    },
  ),
);
