import { useState } from "react";
import { motion } from "framer-motion";
import {
  FileText,
  Code,
  Terminal,
  Book,
  ExternalLink,
  Search,
  ChevronRight,
  Copy,
  Check,
} from "lucide-react";

interface DocSection {
  id: string;
  title: string;
  icon: React.ElementType;
  content: React.ReactNode;
}

const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text);
};

const CodeBlock = ({
  code,
  language = "bash",
}: {
  code: string;
  language?: string;
}) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    copyToClipboard(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="relative group my-4">
      <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
        <button
          onClick={handleCopy}
          className="p-2 rounded-lg bg-surface-hover text-text-secondary hover:text-text-primary"
        >
          {copied ? (
            <Check className="w-4 h-4" />
          ) : (
            <Copy className="w-4 h-4" />
          )}
        </button>
      </div>
      <pre className="bg-canvas border border-border-subtle rounded-xl p-4 overflow-x-auto">
        <code className="text-sm text-text-primary font-mono">{code}</code>
      </pre>
    </div>
  );
};

export default function DocumentationPage() {
  const [activeSection, setActiveSection] = useState("getting-started");
  const [searchQuery, setSearchQuery] = useState("");

  const sections: DocSection[] = [
    {
      id: "getting-started",
      title: "Getting Started",
      icon: Terminal,
      content: (
        <div className="space-y-6">
          <h2 className="text-2xl font-bold text-text-primary">
            Getting Started
          </h2>
          <p className="text-text-secondary">
            Welcome to Rust Learning Ground! This guide will help you get up and
            running with our platform.
          </p>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Quick Start
            </h3>
            <p className="text-text-secondary">
              No installation required! You can start learning Rust immediately
              in your browser.
            </p>
            <ol className="list-decimal list-inside space-y-2 text-text-secondary">
              <li>
                Navigate to the <strong>Exercises</strong> section
              </li>
              <li>Choose an exercise matching your skill level</li>
              <li>Read the instructions and write your solution</li>
              <li>Click "Run" to test your code</li>
              <li>Click "Submit" when you're satisfied</li>
            </ol>
          </div>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Keyboard Shortcuts
            </h3>
            <div className="grid grid-cols-2 gap-4">
              <div className="flex items-center justify-between p-3 bg-surface rounded-lg">
                <span className="text-text-secondary">Run Code</span>
                <kbd className="px-2 py-1 bg-canvas rounded text-sm">
                  Ctrl + Enter
                </kbd>
              </div>
              <div className="flex items-center justify-between p-3 bg-surface rounded-lg">
                <span className="text-text-secondary">Reset Code</span>
                <kbd className="px-2 py-1 bg-canvas rounded text-sm">
                  Ctrl + R
                </kbd>
              </div>
              <div className="flex items-center justify-between p-3 bg-surface rounded-lg">
                <span className="text-text-secondary">Save Code</span>
                <kbd className="px-2 py-1 bg-canvas rounded text-sm">
                  Ctrl + S
                </kbd>
              </div>
            </div>
          </div>
        </div>
      ),
    },
    {
      id: "exercises",
      title: "How Exercises Work",
      icon: Code,
      content: (
        <div className="space-y-6">
          <h2 className="text-2xl font-bold text-text-primary">
            How Exercises Work
          </h2>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Difficulty Levels
            </h3>
            <div className="space-y-2">
              <div className="flex items-center gap-3 p-3 bg-surface rounded-lg">
                <div className="w-3 h-3 rounded-full bg-green-500" />
                <span className="font-medium text-text-primary">Beginner</span>
                <span className="text-text-secondary text-sm">
                  - Basic syntax and concepts
                </span>
              </div>
              <div className="flex items-center gap-3 p-3 bg-surface rounded-lg">
                <div className="w-3 h-3 rounded-full bg-lime-500" />
                <span className="font-medium text-text-primary">Easy</span>
                <span className="text-text-secondary text-sm">
                  - Simple problems
                </span>
              </div>
              <div className="flex items-center gap-3 p-3 bg-surface rounded-lg">
                <div className="w-3 h-3 rounded-full bg-yellow-500" />
                <span className="font-medium text-text-primary">Medium</span>
                <span className="text-text-secondary text-sm">
                  - Intermediate concepts
                </span>
              </div>
              <div className="flex items-center gap-3 p-3 bg-surface rounded-lg">
                <div className="w-3 h-3 rounded-full bg-orange-500" />
                <span className="font-medium text-text-primary">Hard</span>
                <span className="text-text-secondary text-sm">
                  - Advanced patterns
                </span>
              </div>
              <div className="flex items-center gap-3 p-3 bg-surface rounded-lg">
                <div className="w-3 h-3 rounded-full bg-red-500" />
                <span className="font-medium text-text-primary">Expert</span>
                <span className="text-text-secondary text-sm">
                  - Complex algorithms
                </span>
              </div>
            </div>
          </div>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Exercise Structure
            </h3>
            <p className="text-text-secondary">Each exercise includes:</p>
            <ul className="list-disc list-inside space-y-2 text-text-secondary">
              <li>
                <strong>Description</strong> - Problem statement and
                requirements
              </li>
              <li>
                <strong>Template Code</strong> - Starting point for your
                solution
              </li>
              <li>
                <strong>Hints</strong> - Helpful tips if you get stuck (click to
                reveal)
              </li>
              <li>
                <strong>Concepts</strong> - Rust concepts you'll practice
              </li>
              <li>
                <strong>Bonus</strong> - Optional challenges for extra practice
              </li>
            </ul>
          </div>
        </div>
      ),
    },
    {
      id: "gamification",
      title: "Gamification",
      icon: Book,
      content: (
        <div className="space-y-6">
          <h2 className="text-2xl font-bold text-text-primary">
            Gamification Features
          </h2>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              XP System
            </h3>
            <p className="text-text-secondary">
              Earn XP by completing exercises:
            </p>
            <ul className="list-disc list-inside space-y-2 text-text-secondary">
              <li>Beginner exercises: 10-15 XP</li>
              <li>Easy exercises: 15-20 XP</li>
              <li>Medium exercises: 20-30 XP</li>
              <li>Hard exercises: 30-45 XP</li>
              <li>Expert exercises: 45-60 XP</li>
            </ul>
          </div>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Streak System
            </h3>
            <p className="text-text-secondary">
              Maintain your daily learning streak by completing at least one
              exercise each day. Your streak is displayed in the header. Streak
              milestones:
            </p>
            <ul className="list-disc list-inside space-y-2 text-text-secondary">
              <li>7 days - Week Warrior</li>
              <li>30 days - Month Master</li>
              <li>100 days - Centurion</li>
            </ul>
          </div>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Achievements
            </h3>
            <p className="text-text-secondary">
              Unlock achievements as you progress:
            </p>
            <ul className="list-disc list-inside space-y-2 text-text-secondary">
              <li>First Steps - Complete your first exercise</li>
              <li>Problem Solver - Complete 50 exercises</li>
              <li>Code Master - Complete 100 exercises</li>
              <li>And many more...</li>
            </ul>
          </div>
        </div>
      ),
    },
    {
      id: "rust-installation",
      title: "Installing Rust Locally",
      icon: Terminal,
      content: (
        <div className="space-y-6">
          <h2 className="text-2xl font-bold text-text-primary">
            Installing Rust Locally
          </h2>
          <p className="text-text-secondary">
            While you can practice entirely in the browser, you may want to
            install Rust locally for larger projects.
          </p>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Installing rustup
            </h3>
            <p className="text-text-secondary">
              rustup is the Rust installer and version management tool.
            </p>

            <p className="text-text-secondary font-medium">
              On Linux or macOS:
            </p>
            <CodeBlock
              code={`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`}
            />

            <p className="text-text-secondary font-medium">On Windows:</p>
            <p className="text-text-secondary">
              Download and run the installer from{" "}
              <a
                href="https://rustup.rs"
                target="_blank"
                rel="noopener noreferrer"
                className="text-primary hover:underline"
              >
                rustup.rs
              </a>
            </p>
          </div>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Verifying Installation
            </h3>
            <CodeBlock
              code={`rustc --version
cargo --version`}
            />
          </div>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Creating Your First Project
            </h3>
            <CodeBlock
              code={`cargo new hello_world
cd hello_world
cargo run`}
            />
          </div>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Useful Cargo Commands
            </h3>
            <div className="grid gap-3">
              <div className="p-3 bg-surface rounded-lg">
                <code className="text-primary font-mono">cargo build</code>
                <span className="text-text-secondary ml-2">
                  - Compile the project
                </span>
              </div>
              <div className="p-3 bg-surface rounded-lg">
                <code className="text-primary font-mono">cargo run</code>
                <span className="text-text-secondary ml-2">
                  - Build and run
                </span>
              </div>
              <div className="p-3 bg-surface rounded-lg">
                <code className="text-primary font-mono">cargo test</code>
                <span className="text-text-secondary ml-2">- Run tests</span>
              </div>
              <div className="p-3 bg-surface rounded-lg">
                <code className="text-primary font-mono">cargo check</code>
                <span className="text-text-secondary ml-2">
                  - Check for errors without building
                </span>
              </div>
            </div>
          </div>
        </div>
      ),
    },
    {
      id: "troubleshooting",
      title: "Troubleshooting",
      icon: FileText,
      content: (
        <div className="space-y-6">
          <h2 className="text-2xl font-bold text-text-primary">
            Troubleshooting
          </h2>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Common Issues
            </h3>

            <div className="space-y-4">
              <div className="p-4 bg-surface rounded-xl border border-border-subtle">
                <h4 className="font-medium text-text-primary mb-2">
                  Code Won't Run
                </h4>
                <p className="text-text-secondary text-sm">
                  Make sure your code has a main function and compiles without
                  errors. Check the console output for specific error messages.
                </p>
              </div>

              <div className="p-4 bg-surface rounded-xl border border-border-subtle">
                <h4 className="font-medium text-text-primary mb-2">
                  Submission Fails
                </h4>
                <p className="text-text-secondary text-sm">
                  Read the test failure messages carefully. They usually tell
                  you exactly what's wrong. Check edge cases and input
                  validation.
                </p>
              </div>

              <div className="p-4 bg-surface rounded-xl border border-border-subtle">
                <h4 className="font-medium text-text-primary mb-2">
                  Browser Issues
                </h4>
                <p className="text-text-secondary text-sm">
                  We recommend using the latest version of Chrome, Firefox, or
                  Safari. If you experience issues, try clearing your browser
                  cache or disabling extensions.
                </p>
              </div>
            </div>
          </div>

          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-text-primary">
              Getting Help
            </h3>
            <p className="text-text-secondary">
              If you're stuck on an exercise:
            </p>
            <ol className="list-decimal list-inside space-y-2 text-text-secondary">
              <li>Read the exercise hints carefully</li>
              <li>Review the relevant Rust Book chapter</li>
              <li>Check the official Rust documentation</li>
              <li>Visit the Rust community forums</li>
              <li>Ask in the Rust Discord or subreddit</li>
            </ol>
          </div>
        </div>
      ),
    },
  ];

  const filteredSections = sections.filter(
    (section) =>
      section.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (typeof section.content === "string" &&
        section.content.toLowerCase().includes(searchQuery.toLowerCase())),
  );

  const activeContent = sections.find((s) => s.id === activeSection);

  return (
    <div className="max-w-6xl mx-auto">
      <div className="flex flex-col lg:flex-row gap-8">
        {/* Sidebar */}
        <aside className="lg:w-64 flex-shrink-0">
          <div className="sticky top-24 space-y-6">
            <div>
              <h1 className="text-2xl font-display font-bold text-text-primary mb-4">
                Documentation
              </h1>
              <div className="relative">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-muted" />
                <input
                  type="text"
                  placeholder="Search docs..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 rounded-lg bg-surface border border-border-subtle text-text-primary placeholder-text-muted focus:outline-none focus:border-primary/50"
                />
              </div>
            </div>

            <nav className="space-y-1">
              {filteredSections.map((section) => {
                const Icon = section.icon;
                const isActive = activeSection === section.id;

                return (
                  <button
                    key={section.id}
                    onClick={() => setActiveSection(section.id)}
                    className={`w-full flex items-center gap-3 px-4 py-3 rounded-xl text-left transition-all ${
                      isActive
                        ? "bg-primary/10 text-primary"
                        : "text-text-secondary hover:text-text-primary hover:bg-surface"
                    }`}
                  >
                    <Icon className="w-5 h-5" />
                    <span className="font-medium">{section.title}</span>
                    {isActive && <ChevronRight className="w-4 h-4 ml-auto" />}
                  </button>
                );
              })}
            </nav>

            <div className="p-4 rounded-xl bg-gradient-to-br from-primary/10 to-purple-500/10 border border-primary/20">
              <h3 className="font-semibold text-text-primary mb-2">
                External Resources
              </h3>
              <ul className="space-y-2">
                <li>
                  <a
                    href="https://doc.rust-lang.org/book/"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-sm text-text-secondary hover:text-primary flex items-center gap-1"
                  >
                    The Rust Book
                    <ExternalLink className="w-3 h-3" />
                  </a>
                </li>
                <li>
                  <a
                    href="https://doc.rust-lang.org/std/"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-sm text-text-secondary hover:text-primary flex items-center gap-1"
                  >
                    Standard Library
                    <ExternalLink className="w-3 h-3" />
                  </a>
                </li>
                <li>
                  <a
                    href="https://crates.io/"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-sm text-text-secondary hover:text-primary flex items-center gap-1"
                  >
                    Crates.io
                    <ExternalLink className="w-3 h-3" />
                  </a>
                </li>
              </ul>
            </div>
          </div>
        </aside>

        {/* Content */}
        <main className="flex-1 min-w-0">
          <motion.div
            key={activeSection}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-surface rounded-2xl border border-border-subtle p-8"
          >
            {activeContent?.content}
          </motion.div>
        </main>
      </div>
    </div>
  );
}
