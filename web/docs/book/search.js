/**
 * Search functionality for The Book of Rust
 */

class BookSearch {
  constructor() {
    this.index = [];
    this.init();
  }

  async init() {
    this.createSearchUI();
    await this.buildIndex();
  }

  createSearchUI() {
    // Create search container
    const searchContainer = document.createElement('div');
    searchContainer.id = 'book-search';
    searchContainer.className = 'book-search';
    searchContainer.innerHTML = `
      <button class="search-toggle" onclick="bookSearch.toggle()" aria-label="Search">
        🔍
      </button>
      <div class="search-panel" id="search-panel">
        <div class="search-header">
          <input type="text" 
                 class="search-input" 
                 id="search-input" 
                 placeholder="Search the book..."
                 autocomplete="off">
          <button class="search-close" onclick="bookSearch.toggle()">✕</button>
        </div>
        <div class="search-results" id="search-results"></div>
      </div>
    `;

    // Add styles
    const styles = document.createElement('style');
    styles.textContent = `
      .book-search {
        position: fixed;
        top: 20px;
        right: 80px;
        z-index: 1001;
      }
      .search-toggle {
        width: 48px;
        height: 48px;
        border-radius: 50%;
        border: none;
        background: var(--bg-dark);
        color: white;
        font-size: 20px;
        cursor: pointer;
        box-shadow: 0 2px 12px rgba(0,0,0,0.2);
        display: flex;
        align-items: center;
        justify-content: center;
        transition: transform 0.2s;
      }
      .search-toggle:hover {
        transform: scale(1.1);
      }
      .search-panel {
        display: none;
        position: absolute;
        top: 60px;
        right: 0;
        width: 400px;
        max-width: 90vw;
        background: var(--bg-paper);
        border-radius: 12px;
        box-shadow: 0 8px 32px rgba(0,0,0,0.3);
        overflow: hidden;
      }
      .search-panel.active {
        display: block;
      }
      .search-header {
        display: flex;
        align-items: center;
        padding: 12px 16px;
        border-bottom: 1px solid var(--border);
        background: var(--bg-dark);
      }
      .search-input {
        flex: 1;
        padding: 10px 16px;
        border: none;
        border-radius: 8px;
        font-size: 16px;
        background: rgba(255,255,255,0.1);
        color: white;
        outline: none;
      }
      .search-input::placeholder {
        color: rgba(255,255,255,0.5);
      }
      .search-close {
        width: 36px;
        height: 36px;
        border: none;
        background: none;
        color: white;
        font-size: 18px;
        cursor: pointer;
        margin-left: 8px;
      }
      .search-results {
        max-height: 60vh;
        overflow-y: auto;
        padding: 8px 0;
      }
      .search-result {
        padding: 12px 16px;
        border-bottom: 1px solid var(--border);
        cursor: pointer;
        transition: background 0.2s;
      }
      .search-result:hover {
        background: var(--box-bg);
      }
      .search-result-title {
        font-weight: 600;
        color: var(--accent-rust);
        margin-bottom: 4px;
      }
      .search-result-snippet {
        font-size: 14px;
        color: var(--text-muted);
        line-height: 1.4;
      }
      .search-result-match {
        background: rgba(255, 193, 7, 0.3);
        padding: 0 2px;
        border-radius: 2px;
      }
      .search-no-results {
        padding: 24px;
        text-align: center;
        color: var(--text-muted);
      }
      @media (max-width: 600px) {
        .search-panel {
          position: fixed;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          width: 100%;
          max-width: 100%;
          border-radius: 0;
        }
        .search-results {
          max-height: calc(100vh - 80px);
        }
      }
    `;
    document.head.appendChild(styles);
    document.body.appendChild(searchContainer);

    // Bind search input
    const input = document.getElementById('search-input');
    input.addEventListener('input', (e) => this.performSearch(e.target.value));
    input.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') this.toggle();
    });
  }

  async buildIndex() {
    // Build a simple index of chapter titles and key terms
    const chapters = [
      { file: 'chapter-01.html', title: 'The Call to Individuation', terms: 'self journey beginning rust programming' },
      { file: 'chapter-02.html', title: 'The Shadow', terms: 'unsafe raw pointers memory danger unconscious' },
      { file: 'chapter-03.html', title: 'The Anima', terms: 'memory ownership borrowing stack heap' },
      { file: 'chapter-04.html', title: 'The Persona', terms: 'traits interfaces generics masks' },
      { file: 'chapter-05.html', title: 'The Wise Old Man', terms: 'lifetimes references scope wisdom' },
      { file: 'chapter-06.html', title: 'The Great Mother', terms: 'error handling result option panic' },
      { file: 'chapter-07.html', title: 'The Puer Aeternus', terms: 'iterators lazy filter map collect' },
      { file: 'chapter-08.html', title: 'The Trickster', terms: 'closures fn dynamic dispatch capture' },
      { file: 'chapter-09.html', title: 'The Divine Child', terms: 'concurrency threads send sync mutex' },
      { file: 'chapter-10.html', title: 'Zero-Cost Abstractions', terms: 'performance optimization inline monomorphization' },
      { file: 'chapter-11.html', title: 'The Unix Philosophy', terms: 'cli tools composition text streams' },
      { file: 'chapter-12.html', title: 'The Ruler', terms: 'modules crates workspace structure' },
      { file: 'chapter-13.html', title: 'The Explorer', terms: 'network tcp http async sockets' },
      { file: 'chapter-14.html', title: 'The Detective', terms: 'debugging gdb lldb stack trace' },
      { file: 'chapter-15.html', title: 'The Sage', terms: 'testing unit integration property based' },
      { file: 'chapter-16.html', title: 'The Artist', terms: 'api design builder pattern elegance' },
      { file: 'chapter-17.html', title: 'The Hero', terms: 'unsafe rust raw pointers ffi' },
      { file: 'chapter-18.html', title: 'The Integrated Self', terms: 'synthesis mastery conclusion' },
    ];

    const interludes = [
      { file: 'interlude-01.html', title: 'Stack Frame Anatomy', terms: 'assembly registers rsp rbp' },
      { file: 'interlude-02.html', title: 'Heap Anatomy', terms: 'allocation fragmentation malloc' },
      { file: 'interlude-03.html', title: 'Pointer Anatomy', terms: 'references raw smart pointers fat' },
      { file: 'interlude-04.html', title: 'Async Internals', terms: 'futures wakers state machine pin' },
      { file: 'interlude-05.html', title: 'Optimization', terms: 'profiling benchmark flamegraph' },
    ];

    const appendices = [
      { file: 'appendix-a.html', title: 'Glossary', terms: 'definitions terms glossary' },
      { file: 'appendix-b.html', title: 'Further Reading', terms: 'books resources learning' },
      { file: 'appendix-c.html', title: 'Exercise Solutions', terms: 'solutions answers exercises' },
      { file: 'appendix-d.html', title: 'Index', terms: 'index reference' },
    ];

    this.index = [...chapters, ...interludes, ...appendices];
  }

  performSearch(query) {
    const resultsContainer = document.getElementById('search-results');
    
    if (!query.trim()) {
      resultsContainer.innerHTML = '';
      return;
    }

    const terms = query.toLowerCase().split(' ');
    const results = this.index
      .map(item => {
        const text = (item.title + ' ' + item.terms).toLowerCase();
        let score = 0;
        
        terms.forEach(term => {
          if (text.includes(term)) score += 1;
          if (item.title.toLowerCase().includes(term)) score += 2;
        });
        
        return { ...item, score };
      })
      .filter(item => item.score > 0)
      .sort((a, b) => b.score - a.score)
      .slice(0, 10);

    if (results.length === 0) {
      resultsContainer.innerHTML = '<div class="search-no-results">No results found</div>';
      return;
    }

    resultsContainer.innerHTML = results.map(item => {
      let snippet = item.terms;
      // Highlight matching terms
      terms.forEach(term => {
        const regex = new RegExp(`(${term})`, 'gi');
        snippet = snippet.replace(regex, '<span class="search-result-match">$1</span>');
      });
      
      return `
        <div class="search-result" onclick="location.href='${item.file}'">
          <div class="search-result-title">${item.title}</div>
          <div class="search-result-snippet">${snippet}</div>
        </div>
      `;
    }).join('');
  }

  toggle() {
    const panel = document.getElementById('search-panel');
    panel.classList.toggle('active');
    
    if (panel.classList.contains('active')) {
      document.getElementById('search-input').focus();
    }
  }
}

// Initialize
document.addEventListener('DOMContentLoaded', () => {
  window.bookSearch = new BookSearch();
});

// Keyboard shortcut: Ctrl/Cmd + K
document.addEventListener('keydown', (e) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault();
    window.bookSearch?.toggle();
  }
});
