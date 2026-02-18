import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { useBookStore } from "../stores/book";
import { RUST_BOOK } from "../data/rustBook";
import Editor from "@monaco-editor/react";
import DOMPurify from "dompurify";
import {
  Book,
  ChevronLeft,
  ChevronRight,
  CheckCircle,
  Circle,
  Clock,
  Play,
  Bookmark,
  Award,
  BarChart3,
} from "lucide-react";

export default function RustBookPage() {
  const [selectedChapter, setSelectedChapter] = useState<string | null>(null);
  const [selectedSection, setSelectedSection] = useState<string | null>(null);
  const [showQuiz, setShowQuiz] = useState(false);
  const [currentQuiz, setCurrentQuiz] = useState(0);
  const [quizAnswers, setQuizAnswers] = useState<number[]>([]);
  const [showQuizResults, setShowQuizResults] = useState(false);

  const {
    progress,
    bookmarks,
    completeSection,
    saveQuizScore,
    toggleBookmark,
    getChapterProgress,
    getOverallProgress,
    isSectionCompleted,
  } = useBookStore();

  const overallProgress = getOverallProgress();

  const handleChapterSelect = (chapterId: string) => {
    setSelectedChapter(chapterId);
    setSelectedSection(null);
    setShowQuiz(false);
  };

  const handleSectionSelect = (sectionId: string) => {
    setSelectedSection(sectionId);
    setShowQuiz(false);
  };

  const handleSectionComplete = () => {
    if (selectedChapter && selectedSection) {
      completeSection(selectedChapter, selectedSection);
    }
  };

  const handleQuizSubmit = () => {
    if (selectedChapter && selectedSection) {
      const section = RUST_BOOK.find(
        (c) => c.id === selectedChapter,
      )?.sections.find((s) => s.id === selectedSection);

      if (section?.quiz) {
        const score = quizAnswers.reduce((acc, answer, idx) => {
          return acc + (answer === section.quiz![idx].correctAnswer ? 1 : 0);
        }, 0);

        saveQuizScore(selectedChapter, selectedSection, score);
        setShowQuizResults(true);
      }
    }
  };

  const renderChapterList = () => (
    <div className="space-y-4">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-3xl font-display font-bold text-text-primary flex items-center gap-3">
            <Book className="w-8 h-8 text-primary" />
            The Rust Book
          </h1>
          <p className="text-text-secondary mt-1">
            Learn Rust from the official book, integrated with exercises
          </p>
        </div>
        <div className="text-right">
          <div className="text-2xl font-bold text-text-primary">
            {overallProgress}%
          </div>
          <div className="text-sm text-text-secondary">Complete</div>
        </div>
      </div>

      <div className="h-3 bg-surface rounded-full overflow-hidden mb-8">
        <motion.div
          initial={{ width: 0 }}
          animate={{ width: `${overallProgress}%` }}
          className="h-full bg-gradient-to-r from-primary to-purple-500"
        />
      </div>

      <div className="grid gap-4">
        {RUST_BOOK.map((chapter, index) => {
          const chapterProgress = getChapterProgress(chapter.id);
          const isCompleted = chapterProgress === 100;

          return (
            <motion.div
              key={chapter.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.05 }}
              onClick={() => handleChapterSelect(chapter.id)}
              className={`p-6 rounded-2xl border cursor-pointer transition-all hover:shadow-lg ${
                isCompleted
                  ? "bg-success/5 border-success/30"
                  : chapterProgress > 0
                    ? "bg-primary/5 border-primary/30"
                    : "bg-surface border-border-subtle hover:border-primary/30"
              }`}
            >
              <div className="flex items-start gap-4">
                <div
                  className={`w-12 h-12 rounded-xl flex items-center justify-center ${
                    isCompleted
                      ? "bg-success text-white"
                      : chapterProgress > 0
                        ? "bg-primary text-white"
                        : "bg-surface-hover"
                  }`}
                >
                  {isCompleted ? (
                    <CheckCircle className="w-6 h-6" />
                  ) : (
                    <span className="font-bold">{index + 1}</span>
                  )}
                </div>

                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    <h3 className="text-lg font-semibold text-text-primary">
                      {chapter.title}
                    </h3>
                    <span
                      className={`px-2 py-1 rounded-full text-xs ${
                        chapter.difficulty === "beginner"
                          ? "bg-green-500/20 text-green-400"
                          : chapter.difficulty === "intermediate"
                            ? "bg-yellow-500/20 text-yellow-400"
                            : "bg-red-500/20 text-red-400"
                      }`}
                    >
                      {chapter.difficulty}
                    </span>
                  </div>
                  <p className="text-text-secondary text-sm mb-3">
                    {chapter.description}
                  </p>
                  <div className="flex items-center gap-4 text-sm text-text-muted">
                    <span className="flex items-center gap-1">
                      <Clock className="w-4 h-4" />
                      {chapter.estimatedReadTime} min
                    </span>
                    <span className="flex items-center gap-1">
                      <BarChart3 className="w-4 h-4" />
                      {chapter.sections.length} sections
                    </span>
                    <span className="flex items-center gap-1">
                      {chapterProgress}%
                    </span>
                  </div>
                </div>

                <div className="w-16 h-16 relative">
                  <svg className="w-full h-full -rotate-90">
                    <circle
                      cx="32"
                      cy="32"
                      r="28"
                      fill="none"
                      stroke="currentColor"
                      strokeWidth="4"
                      className="text-surface-hover"
                    />
                    <circle
                      cx="32"
                      cy="32"
                      r="28"
                      fill="none"
                      stroke="currentColor"
                      strokeWidth="4"
                      strokeDasharray={`${chapterProgress * 1.76} 176`}
                      className={`${
                        isCompleted ? "text-success" : "text-primary"
                      } transition-all`}
                    />
                  </svg>
                </div>
              </div>
            </motion.div>
          );
        })}
      </div>
    </div>
  );

  const renderChapterDetail = () => {
    if (!selectedChapter) return null;

    const chapter = RUST_BOOK.find((c) => c.id === selectedChapter);
    if (!chapter) return null;

    return (
      <div className="space-y-6">
        <div className="flex items-center gap-4">
          <button
            onClick={() => setSelectedChapter(null)}
            className="p-2 rounded-lg bg-surface hover:bg-surface-hover transition-colors"
          >
            <ChevronLeft className="w-5 h-5" />
          </button>
          <div>
            <h2 className="text-2xl font-bold text-text-primary">
              {chapter.title}
            </h2>
            <p className="text-text-secondary">{chapter.description}</p>
          </div>
        </div>

        <div className="grid gap-3">
          {chapter.sections.map((section, index) => {
            const isCompleted = isSectionCompleted(chapter.id, section.id);
            const isBookmarked = bookmarks.includes(section.id);

            return (
              <motion.div
                key={section.id}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: index * 0.05 }}
                onClick={() => handleSectionSelect(section.id)}
                className={`p-4 rounded-xl border cursor-pointer transition-all ${
                  isCompleted
                    ? "bg-success/5 border-success/30"
                    : "bg-surface border-border-subtle hover:border-primary/30"
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    {isCompleted ? (
                      <CheckCircle className="w-5 h-5 text-success" />
                    ) : (
                      <Circle className="w-5 h-5 text-text-muted" />
                    )}
                    <span className="font-medium text-text-primary">
                      {index + 1}. {section.title}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    {section.quiz && <Award className="w-4 h-4 text-primary" />}
                    {section.codeExamples &&
                      section.codeExamples.length > 0 && (
                        <Play className="w-4 h-4 text-text-muted" />
                      )}
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleBookmark(section.id);
                      }}
                      className={`p-1 rounded transition-colors ${
                        isBookmarked
                          ? "text-primary"
                          : "text-text-muted hover:text-primary"
                      }`}
                    >
                      <Bookmark
                        className={`w-4 h-4 ${isBookmarked ? "fill-current" : ""}`}
                      />
                    </button>
                    <ChevronRight className="w-4 h-4 text-text-muted" />
                  </div>
                </div>
              </motion.div>
            );
          })}
        </div>
      </div>
    );
  };

  const renderSectionDetail = () => {
    if (!selectedChapter || !selectedSection) return null;

    const chapter = RUST_BOOK.find((c) => c.id === selectedChapter);
    const section = chapter?.sections.find((s) => s.id === selectedSection);
    if (!chapter || !section) return null;

    const isCompleted = isSectionCompleted(chapter.id, section.id);

    return (
      <div className="space-y-6">
        <div className="flex items-center gap-4">
          <button
            onClick={() => setSelectedSection(null)}
            className="p-2 rounded-lg bg-surface hover:bg-surface-hover transition-colors"
          >
            <ChevronLeft className="w-5 h-5" />
          </button>
          <div>
            <h2 className="text-2xl font-bold text-text-primary">
              {section.title}
            </h2>
            <p className="text-text-secondary text-sm">
              {chapter.title} • Section{" "}
              {chapter.sections.findIndex((s) => s.id === section.id) + 1}
            </p>
          </div>
        </div>

        <div className="prose prose-invert max-w-none bg-surface p-8 rounded-2xl border border-border-subtle">
          <div
            dangerouslySetInnerHTML={{
              __html: DOMPurify.sanitize(
                section.content.replace(/\n/g, "<br/>"),
              ),
            }}
          />
        </div>

        {section.codeExamples && section.codeExamples.length > 0 && (
          <div className="space-y-4">
            <h3 className="text-lg font-semibold text-text-primary">
              Code Examples
            </h3>
            {section.codeExamples.map((example) => (
              <div
                key={example.id}
                className="bg-surface rounded-xl border border-border-subtle overflow-hidden"
              >
                <div className="px-4 py-2 bg-surface-hover border-b border-border-subtle flex items-center justify-between">
                  <span className="font-medium text-text-primary">
                    {example.title}
                  </span>
                  {example.runnable && (
                    <span className="text-xs text-success flex items-center gap-1">
                      <Play className="w-3 h-3" />
                      Runnable
                    </span>
                  )}
                </div>
                <div className="h-48">
                  <Editor
                    height="100%"
                    defaultLanguage="rust"
                    value={example.code}
                    theme="vs-dark"
                    options={{
                      readOnly: true,
                      minimap: { enabled: false },
                      lineNumbers: "on",
                    }}
                  />
                </div>
                <div className="p-4 bg-surface-hover text-sm text-text-secondary">
                  {example.explanation}
                </div>
              </div>
            ))}
          </div>
        )}

        {section.quiz && !showQuiz && !showQuizResults && (
          <button
            onClick={() => {
              setShowQuiz(true);
              setCurrentQuiz(0);
              setQuizAnswers([]);
              setShowQuizResults(false);
            }}
            className="w-full py-4 rounded-xl bg-primary text-white font-medium hover:bg-primary-hover transition-all flex items-center justify-center gap-2"
          >
            <Award className="w-5 h-5" />
            Take Quiz ({section.quiz.length} questions)
          </button>
        )}

        {showQuiz && section.quiz && (
          <div className="bg-surface p-6 rounded-2xl border border-border-subtle space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="text-lg font-semibold text-text-primary">
                Question {currentQuiz + 1} of {section.quiz.length}
              </h3>
              <div className="w-32 h-2 bg-surface-hover rounded-full overflow-hidden">
                <div
                  className="h-full bg-primary transition-all"
                  style={{
                    width: `${((currentQuiz + 1) / section.quiz.length) * 100}%`,
                  }}
                />
              </div>
            </div>

            <p className="text-text-primary">
              {section.quiz[currentQuiz].question}
            </p>

            <div className="space-y-2">
              {section.quiz[currentQuiz].options.map((option, idx) => (
                <button
                  key={idx}
                  onClick={() => {
                    const newAnswers = [...quizAnswers];
                    newAnswers[currentQuiz] = idx;
                    setQuizAnswers(newAnswers);
                  }}
                  className={`w-full p-4 rounded-xl border text-left transition-all ${
                    quizAnswers[currentQuiz] === idx
                      ? "border-primary bg-primary/10"
                      : "border-border-subtle hover:border-primary/30"
                  }`}
                >
                  {option}
                </button>
              ))}
            </div>

            <div className="flex justify-end">
              <button
                onClick={() => {
                  if (currentQuiz < section.quiz!.length - 1) {
                    setCurrentQuiz(currentQuiz + 1);
                  } else {
                    handleQuizSubmit();
                  }
                }}
                disabled={quizAnswers[currentQuiz] === undefined}
                className="px-6 py-2 rounded-lg bg-primary text-white font-medium disabled:opacity-50 disabled:cursor-not-allowed hover:bg-primary-hover transition-colors"
              >
                {currentQuiz < section.quiz.length - 1 ? "Next" : "Submit"}
              </button>
            </div>
          </div>
        )}

        {showQuizResults && section.quiz && (
          <div className="bg-surface p-6 rounded-2xl border border-border-subtle text-center">
            <Award className="w-12 h-12 text-primary mx-auto mb-4" />
            <h3 className="text-xl font-bold text-text-primary mb-2">
              Quiz Complete!
            </h3>
            <p className="text-text-secondary mb-4">
              You scored{" "}
              {quizAnswers.reduce(
                (acc, answer, idx) =>
                  acc + (answer === section.quiz![idx].correctAnswer ? 1 : 0),
                0,
              )}{" "}
              out of {section.quiz.length}
            </p>
            <button
              onClick={() => {
                setShowQuiz(false);
                setShowQuizResults(false);
              }}
              className="px-6 py-2 rounded-lg bg-primary text-white font-medium hover:bg-primary-hover transition-colors"
            >
              Back to Section
            </button>
          </div>
        )}

        {!isCompleted && (
          <button
            onClick={handleSectionComplete}
            className="w-full py-4 rounded-xl bg-success text-white font-medium hover:bg-success-hover transition-all flex items-center justify-center gap-2"
          >
            <CheckCircle className="w-5 h-5" />
            Mark as Complete
          </button>
        )}
      </div>
    );
  };

  return (
    <div className="max-w-4xl mx-auto">
      <AnimatePresence mode="wait">
        {selectedSection ? (
          <motion.div
            key="section"
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
          >
            {renderSectionDetail()}
          </motion.div>
        ) : selectedChapter ? (
          <motion.div
            key="chapter"
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
          >
            {renderChapterDetail()}
          </motion.div>
        ) : (
          <motion.div
            key="list"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
          >
            {renderChapterList()}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
