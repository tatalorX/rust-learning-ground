# 🦀 The Book of Rust - Interactive Learning Platform

[![Project Status](https://img.shields.io/badge/status-100%25%20complete-success)](https://github.com/tatalorX/rust-learning-ground)
[![Exercises](https://img.shields.io/badge/exercises-320-blue)](web/exercise_data.json)
[![Pages](https://img.shields.io/badge/book%20pages-314-orange)](web/docs/book/)
[![Languages](https://img.shields.io/badge/languages-6%2B-purple)](web/docs/book/translations/)

> "Who looks outside, dreams; who looks inside, awakes." — C.G. Jung

A comprehensive, philosophical, and interactive platform for learning Rust programming through the lens of Jungian archetypes.

## 🚀 Quick Deploy

Deploy your own instance in minutes:

```bash
# On your server (as root)
export DOMAIN=yourdomain.com
export EMAIL=admin@yourdomain.com
curl -fsSL https://raw.githubusercontent.com/tatalorX/rust-learning-ground/main/deploy.sh | sudo bash
```

Or clone and deploy manually:

```bash
git clone https://github.com/tatalorX/rust-learning-ground.git
cd rust-learning-ground
export DOMAIN=yourdomain.com
sudo -E bash deploy.sh
```

## 🎯 Project Overview

**The Book of Rust** is a unique educational resource that combines:
- **Deep Psychology**: Jungian archetypes as learning frameworks
- **Technical Precision**: Surgical-level systems programming detail
- **Pragmatic Philosophy**: Unix/Linux kernel wisdom
- **Interactive Practice**: 320 hands-on exercises

### 📊 Completion Status: 100%

| Component | Status | Details |
|-----------|--------|---------|
| The Book | ✅ 100% | 28 components, ~319 pages |
| Chapters | ✅ 100% | 18 complete chapters |
| Interludes | ✅ 100% | 5 surgical deep-dives |
| Appendices | ✅ 100% | 4 reference appendices |
| Exercises | ✅ 100% | 320 hands-on problems |
| Templates | ✅ 100% | 330 exercise directories |
| Translations | ✅ 100% | 6 primary + 10+ auto |

## 📚 The Book Structure

### Part I: The Archetypes
*Jungian Psychology meets Rust*

1. **The Call to Individuation** — The journey begins
2. **The Shadow** — Understanding `unsafe`
3. **The Anima** — Memory as soul (ownership)
4. **The Persona** — Traits as masks
5. **The Wise Old Man** — Lifetimes as wisdom
6. **The Great Mother** — Error handling

### Part II: Surgical Precision
*The anatomy of computation*

- **Interlude I**: Stack Frame Anatomy
- **Interlude II**: Heap Anatomy
- **Interlude III**: Pointer Anatomy
7. **The Puer Aeternus** — Iterators
8. **The Trickster** — Closures

### Part III: Kernel Philosophy
*Pragmatic systems thinking*

9. **The Divine Child** — Fearless concurrency
10. **Zero-Cost Abstractions**
11. **The Unix Philosophy**

### Part IV: Collective Systems
*Async, network, distributed*

- **Interlude IV**: Async Internals
12. **The Ruler** — Project structure
13. **The Explorer** — Network programming

### Part V: Mastery
*The path of the crafts-person*

- **Interlude V**: Optimization
14. **The Detective** — Debugging
15. **The Sage** — Testing
16. **The Artist** — API design
17. **The Hero** — Unsafe Rust
18. **The Integrated Self** — Synthesis

### Appendices

- **A**: Glossary
- **B**: Further Reading
- **C**: Exercise Solutions Guide
- **D**: Index

## 🌍 Multilingual Support

### Primary Languages (Manual Translation)
| Flag | Language | Code | Status |
|------|----------|------|--------|
| 🇬🇧 | English | `en` | ✅ Complete |
| 🇷🇴 | Română | `ro` | ✅ Complete |
| 🇮🇹 | Italiano | `it` | ✅ Complete |
| 🇩🇪 | Deutsch | `de` | ✅ Complete |
| 🇷🇺 | Русский | `ru` | ✅ Complete |
| 🇨🇳 | 简体中文 | `zh` | ✅ Complete |

### Secondary Languages (Auto-Translate)
- 🇫🇷 Français
- 🇪🇸 Español
- 🇵🇹 Português
- 🇯🇵 日本語
- 🇰🇷 한국어
- 🇸🇦 العربية
- 🇮🇳 हिन्दी
- 🇹🇷 Türkçe
- 🇵🇱 Polski
- 🇳🇱 Nederlands

## 🎨 Features

### Reading Experience
- **🌙 Dark Mode Toggle** — Easy on the eyes for night reading
- **🔍 Content Search** — Press `Ctrl+K` to search all chapters
- **🖨️ Print-Friendly** — Optimized styles for printing
- **📱 Responsive Design** — Works on all devices
- **🌐 Language Switcher** — Fixed position for easy access

### Interactive Elements
- **320 Exercises** — From basics to real-world systems
- **Progress Tracking** — Visual progress indicators
- **Hint System** — Helpful nudges when stuck
- **Solution Viewing** — Learn from reference implementations

### Pedagogical Approach
Each chapter contains three special box types:

1. **🎭 Archetype Boxes** — Jungian psychological insights
2. **⚙️ Kernel Wisdom** — Pragmatic systems thinking
3. **🔬 Surgical Notes** — Byte-level technical precision

## 🚀 Quick Start

### Prerequisites
- Python 3.8+
- Rust toolchain
- Node.js (optional, for web development)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-learning-ground.git
cd rust-learning-ground

# Install Python dependencies
pip install -r requirements.txt

# Start the servers
python start_servers.py
```

### Access the Platform
- **Main Dashboard**: http://localhost:3000
- **The Book**: http://localhost:3000/docs/book/toc.html
- **API Documentation**: http://localhost:8000/docs

## 📖 Reading The Book

Start with the [Table of Contents](web/docs/book/toc.html) for the full navigation structure.

### Suggested Reading Paths

**Beginner Path** (Exercises 001-100):
1. Chapters 1-6 (Foundations)
2. Interlude I (Stack Anatomy)
3. Chapter 7-8 (Iterators, Closures)
4. Appendix A (Glossary)

**Intermediate Path** (Exercises 101-200):
1. Chapters 9-11 (Concurrency, Abstractions)
2. Interlude IV (Async Internals)
3. Chapter 12-13 (Modules, Networking)

**Advanced Path** (Exercises 201-320):
1. All Interludes
2. Chapters 14-18 (Mastery)
3. All Appendices

## 📝 Exercise Categories

| Category | Count | Range | Focus |
|----------|-------|-------|-------|
| Basics | 100 | 001-100 | Syntax, ownership, borrowing |
| Advanced | 50 | 101-150 | Algorithms, data structures |
| Projects | 20 | 151-170 | Real applications |
| Real-World | 150 | 171-320 | Systems, networking, async |

## 🛠️ Technical Stack

### Backend
- **FastAPI** — Modern Python web framework
- **Pydantic** — Data validation
- **Uvicorn** — ASGI server

### Frontend
- **Vanilla JavaScript** — No framework dependencies
- **CSS3** — Custom properties, grid, flexbox
- **Google Fonts** — Crimson Text, JetBrains Mono

### Infrastructure
- **CORS** — Configured for remote access
- **Hot Reload** — Development convenience
- **Modular Config** — Environment-based settings

## 📁 Project Structure

```
rust-learning-ground/
├── web/                          # Frontend & book
│   ├── docs/book/               # The Book of Rust
│   │   ├── chapter-*.html       # 18 chapters
│   │   ├── interlude-*.html     # 5 interludes
│   │   ├── appendix-*.html      # 4 appendices
│   │   ├── translations/        # i18n files
│   │   ├── dark-mode.*          # Theme system
│   │   ├── search.js            # Search functionality
│   │   └── print.css            # Print styles
│   ├── index.html               # Main dashboard
│   └── exercise_data.json       # 320 exercises
├── server/                       # Backend
│   └── app/
│       ├── main.py              # FastAPI app
│       └── config.py            # Settings
├── problems/                     # Exercise templates
│   ├── *_exercise/              # 280 exercises
│   └── *_project/               # 50 projects
├── start_servers.py             # Launch script
└── README.md                    # This file
```

## 🤝 Contributing

Contributions are welcome! Areas for contribution:

- **Translations** — Additional primary languages
- **Exercises** — More practice problems
- **Content** — Additional examples and explanations
- **Accessibility** — Improved a11y support
- **Features** — New learning tools

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📜 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

The Book of Rust content is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/).

## 🙏 Acknowledgments

- **Carl Jung** — For the archetypal framework
- **Linus Torvalds** — For pragmatic wisdom
- **The Rust Team** — For an incredible language
- **The Rust Community** — For inspiration and support

## 📬 Contact

- **Issues**: [GitHub Issues](https://github.com/yourusername/rust-learning-ground/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/rust-learning-ground/discussions)

---

<p align="center">
  <strong>Start your journey.</strong><br>
  <a href="web/docs/book/toc.html">📖 Read The Book</a> •
  <a href="web/index.html">💻 Launch Platform</a>
</p>

<p align="center">
  <em>"The privilege of a lifetime is to become who you truly are."</em><br>
  — C.G. Jung
</p>
