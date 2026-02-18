"""
Exercise Loader Service

Loads exercises from /problems/ directory, parses metadata from template.rs files,
and provides caching for fast access.
"""

import os
import re
from pathlib import Path
from typing import Optional, List, Dict, Any
from dataclasses import dataclass, field
import logging

logger = logging.getLogger(__name__)

PROBLEMS_DIR = Path(
    "/home/mushroom/Downloads/rust-learner/rust-learning-ground/problems"
)

CATEGORIES = {
    "basics": "Rust Basics",
    "variables": "Variables & Mutability",
    "data_types": "Data Types",
    "functions": "Functions",
    "control_flow": "Control Flow",
    "ownership": "Ownership",
    "references": "References & Borrowing",
    "slices": "Slices",
    "structs": "Structs",
    "enums": "Enums",
    "pattern_matching": "Pattern Matching",
    "modules": "Modules",
    "collections": "Collections",
    "error_handling": "Error Handling",
    "generics": "Generics",
    "traits": "Traits",
    "lifetime": "Lifetimes",
    "testing": "Testing",
    "file_io": "File I/O",
    "advanced": "Advanced Topics",
}

EXERCISE_PATTERN = re.compile(r"//\s*Exercise\s*(\d+):\s*(.+)")
LEARNING_OBJ_PATTERN = re.compile(r"//\s*Learning\s*[Oo]bjective:?\s*(.+)")
DIFFICULTY_PATTERN = re.compile(r"//\s*Difficulty:\s*(\d+)")


@dataclass
class Exercise:
    id: int
    title: str
    description: str
    category: str
    difficulty: int
    template_code: str
    concepts: List[str] = field(default_factory=list)
    bonus: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": self.id,
            "title": self.title,
            "description": self.description,
            "category": self.category,
            "difficulty": self.difficulty,
            "concepts": self.concepts,
            "bonus": self.bonus,
        }


class ExerciseLoader:
    _instance: Optional["ExerciseLoader"] = None
    _exercises: Dict[int, Exercise] = {}
    _categories_cache: Optional[Dict[str, Any]] = None

    def __new__(cls) -> "ExerciseLoader":
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            cls._instance._loaded = False
        return cls._instance

    def __init__(self):
        if not self._loaded:
            self._load_all_exercises()
            self._loaded = True

    def _parse_exercise_file(
        self, exercise_id: int, file_path: Path
    ) -> Optional[Exercise]:
        try:
            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()
        except Exception as e:
            logger.error(f"Failed to read {file_path}: {e}")
            return None

        lines = content.split("\n")
        header_lines = []
        in_header = True
        template_code = []

        for i, line in enumerate(lines):
            if in_header:
                if line.startswith("fn main()"):
                    in_header = False
                    template_code = lines[i:]
                else:
                    header_lines.append(line)
            else:
                template_code = lines[i:]
                break

        header_text = "\n".join(header_lines)

        title_match = EXERCISE_PATTERN.search(header_text)
        title = (
            title_match.group(2).strip() if title_match else f"Exercise {exercise_id}"
        )

        description_match = LEARNING_OBJ_PATTERN.search(header_text)
        description = description_match.group(1).strip() if description_match else ""

        lines_for_difficulty = header_text[:500]
        difficulty_match = DIFFICULTY_PATTERN.search(lines_for_difficulty)
        difficulty = int(difficulty_match.group(1)) if difficulty_match else 1

        category = self._extract_category(header_text, title)
        concepts = self._extract_concepts(header_text)
        bonus = self._extract_bonus(header_text)

        return Exercise(
            id=exercise_id,
            title=title,
            description=description,
            category=category,
            difficulty=difficulty,
            template_code="\n".join(template_code),
            concepts=concepts,
            bonus=bonus,
        )

    def _extract_category(self, header: str, title: str) -> str:
        title_lower = title.lower()

        category_keywords = {
            "ownership": ["ownership"],
            "borrowing": ["borrowing", "borrow"],
            "references": ["reference", "borrow"],
            "slices": ["slice"],
            "structs": ["struct"],
            "enums": ["enum"],
            "pattern_matching": ["match", "pattern"],
            "modules": ["module"],
            "collections": ["vector", "hashmap", "hash set", "string", "vec"],
            "error_handling": ["error", "result", "option"],
            "generics": ["generic"],
            "traits": ["trait", "impl"],
            "lifetime": ["lifetime"],
            "testing": ["test", "assert"],
            "file_io": ["file", "read", "write"],
            "functions": ["function", "fn "],
            "control_flow": ["if", "else", "loop", "while", "for ", "match "],
            "variables": ["variable", "mut", "const", "shadow"],
            "data_types": ["type", "integer", "float", "bool", "char", "tuple"],
            "basics": ["println", "hello", "basic"],
        }

        header_lower = header.lower()

        for category, keywords in category_keywords.items():
            for keyword in keywords:
                if keyword in title_lower or keyword in header_lower:
                    return category

        return "basics"

    def _extract_concepts(self, header: str) -> List[str]:
        concepts = []
        concept_patterns = [
            ("Ownership", r"ownership"),
            ("Borrowing", r"borrow(ing)?"),
            ("References", r"reference"),
            ("Slices", r"slice"),
            ("Structs", r"struct"),
            ("Enums", r"enum"),
            ("Pattern Matching", r"match"),
            ("Modules", r"module"),
            ("Vectors", r"vec!|vector"),
            ("HashMap", r"hashmap|hash map"),
            ("Functions", r"fn |function"),
            ("Control Flow", r"if|else|loop|while|for"),
            ("Variables", r"mut|const|variable"),
            ("Data Types", r"type|integer|float|bool|char|tuple"),
            ("Error Handling", r"result|option|error"),
            ("Generics", r"generic"),
            ("Traits", r"trait"),
            ("Lifetimes", r"lifetime"),
            ("Testing", r"test|assert"),
            ("File I/O", r"file|read|write"),
        ]

        header_lower = header.lower()
        for concept, pattern in concept_patterns:
            if re.search(pattern, header_lower):
                concepts.append(concept)

        return concepts if concepts else ["Rust Basics"]

    def _extract_bonus(self, header: str) -> Optional[str]:
        bonus_match = re.search(
            r"BONUS[:\s]*(.+?)(?:\n\n|\n//|\Z)", header, re.IGNORECASE | re.DOTALL
        )
        if bonus_match:
            return bonus_match.group(1).strip()
        return None

    def _load_all_exercises(self) -> None:
        self._exercises = {}

        if not PROBLEMS_DIR.exists():
            logger.warning(f"Problems directory not found: {PROBLEMS_DIR}")
            return

        for entry in PROBLEMS_DIR.iterdir():
            if entry.is_dir() and entry.name.endswith("_exercise"):
                try:
                    exercise_id = int(entry.name.split("_")[0])
                except (ValueError, IndexError):
                    continue

                template_path = entry / "template.rs"
                if template_path.exists():
                    exercise = self._parse_exercise_file(exercise_id, template_path)
                    if exercise:
                        self._exercises[exercise_id] = exercise
                        logger.debug(f"Loaded exercise {exercise_id}: {exercise.title}")

        logger.info(f"Loaded {len(self._exercises)} exercises")

    def get_exercise(self, exercise_id: int) -> Optional[Exercise]:
        return self._exercises.get(exercise_id)

    def list_exercises(
        self,
        cursor: Optional[int] = None,
        limit: int = 20,
        difficulty: Optional[int] = None,
        category: Optional[str] = None,
        search: Optional[str] = None,
    ) -> Dict[str, Any]:
        sorted_ids = sorted(self._exercises.keys())

        if cursor:
            start_idx = None
            for i, eid in enumerate(sorted_ids):
                if eid == cursor:
                    start_idx = i + 1
                    break
            if start_idx is None:
                start_idx = 0
        else:
            start_idx = 0

        filtered = []
        for eid in sorted_ids[start_idx:]:
            ex = self._exercises[eid]
            if difficulty and ex.difficulty != difficulty:
                continue
            if category and ex.category != category:
                continue
            if search:
                search_lower = search.lower()
                if (
                    search_lower not in ex.title.lower()
                    and search_lower not in ex.description.lower()
                ):
                    continue
            filtered.append(ex)
            if len(filtered) >= limit:
                break

        has_more = len(filtered) >= limit
        next_cursor = filtered[-1].id if filtered and has_more else None

        return {
            "data": [ex.to_dict() for ex in filtered],
            "pagination": {
                "cursor": cursor,
                "next_cursor": next_cursor,
                "limit": limit,
                "has_more": has_more,
                "total": len(self._exercises),
            },
            "filters": {
                "difficulty": difficulty,
                "category": category,
                "search": search,
            },
        }

    def get_categories(self) -> Dict[str, Any]:
        if self._categories_cache:
            return self._categories_cache

        categories = {}
        for ex in self._exercises.values():
            cat = ex.category
            if cat not in categories:
                categories[cat] = {
                    "id": cat,
                    "name": CATEGORIES.get(cat, cat.title()),
                    "count": 0,
                    "exercises": [],
                }
            categories[cat]["count"] += 1
            categories[cat]["exercises"].append({"id": ex.id, "title": ex.title})

        self._categories_cache = {
            "categories": list(categories.values()),
            "total": len(categories),
        }
        return self._categories_cache

    def get_difficulties(self) -> List[Dict[str, Any]]:
        difficulties = {}
        for ex in self._exercises.values():
            d = ex.difficulty
            if d not in difficulties:
                difficulties[d] = {"level": d, "count": 0, "label": f"Level {d}"}
            difficulties[d]["count"] += 1
        return list(difficulties.values())


def get_exercise_loader() -> ExerciseLoader:
    return ExerciseLoader()
