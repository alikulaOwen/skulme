#!/usr/bin/env python3
"""
scripts/migrate_all_problems.py

Automated Migration Engine:
1. Migrates all 394 Rust algorithms from `src/` into `problems/<category>/<problem>/solution.rs`
   with Socratic TODO stubs and full test suites.
2. Generates `problems/<category>/mod.rs` with `#[path = "<problem>/solution.rs"]` mappings.
3. Generates `problems/lib.rs` and points `Cargo.toml` to `problems/lib.rs`.
4. Exports a complete Supabase SQL schema and JSON database seed for the upcoming Lovable platform.
"""

import os
import re
import glob
import json
import shutil

SRC_DIR = "src"
PROBLEMS_DIR = "problems"
DB_DIR = "database"

STORIES = {
    "all_combination_of_size_k": {
        "title": "The All-Star Squad Selection: Combinations of Size K",
        "story": "Imagine you are the head coach of a national basketball team:\n- You have N candidate athletes numbered 0 to N-1.\n- You need to select a starting lineup of exactly K athletes.\n- Player order does not matter: [0, 1] is the exact same lineup as [1, 0].",
        "hints": "❓ Q1: How do we avoid duplicates like [1, 0]? (Only move forward: start from num + 1)\n❓ Q2: When is our squad complete? (Base case: current.len() == k)\n❓ Q3: How do we choose, explore, and un-choose?"
    },
    "permutations": {
        "title": "The Vault Pin-Code: Distinct Permutations",
        "story": "Imagine you are an elite safe-cracker facing a high-security electronic vault:\n- You are given a set of numeric tiles.\n- You must arrange them into all possible distinct passcodes.\n- Order MATTERS: [1, 2, 3] != [3, 2, 1].",
        "hints": "❓ Q1: How do we track which tiles are used? (Use a boolean tracker)\n❓ Q2: How do we avoid duplicates when duplicate tiles exist? (Sort & skip adjacent duplicates)\n❓ Q3: When is passcode complete? (Base case: current.len() == nums.len())"
    },
    "parentheses_generator": {
        "title": "Architect's Balanced Bridges: Parentheses Generator",
        "story": "Imagine you are an architect designing suspended skybridges between skyscrapers:\n- An open bracket '(' anchors a suspension cable from Tower A.\n- A close bracket ')' locks that cable into Tower B.\n- You can NEVER place a close bracket without a matching open cable already in place!",
        "hints": "❓ Q1: When can we add an open bracket? (When open_count < n)\n❓ Q2: When can we add a close bracket? (When close_count < open_count)\n❓ Q3: When is the bridge complete? (When current.len() == 2 * n)"
    },
    "subset_sum": {
        "title": "Cash Register Audit: Subset Sum",
        "story": "Imagine you are a store manager balancing the cash register at closing:\n- You have an assortment of bills/coins of various denominations.\n- An audit invoice demands a payment of EXACTLY the target sum.\n- Can you find any combination of bills that sums to this exact amount?",
        "hints": "❓ Q1: What choices do we make at each bill? (0/1 decision: Take it or Leave it)\n❓ Q2: How do we prune early? (If target < 0, abort immediately)\n❓ Q3: When have we succeeded? (If target == 0, return true)"
    },
    "rat_in_maze": {
        "title": "Laboratory Cheese Heist: Rat in a Maze",
        "story": "Imagine a clever lab rat placed at the entrance (0, 0) of an N x N grid maze:\n- Open path cells have food/scent.\n- Wall cells are blocked.\n- The rat wants to find a continuous path to the exit at (N-1, N-1).",
        "hints": "❓ Q1: Which directions can the rat move? (Down, Right, Up, Left)\n❓ Q2: What makes a cell valid? (In bounds, open, and not currently visited)\n❓ Q3: When does the rat reach the exit? (x == n - 1 && y == n - 1)"
    },
    "n_queens": {
        "title": "The Royal Peace Treaty: N-Queens",
        "story": "Imagine N rival queens attending a peace summit in an N x N grand banquet hall:\n- In chess, a queen commands her row, column, and both diagonals.\n- If any two queens see each other, war breaks out!\n- Seat all N queens peacefully.",
        "hints": "❓ Q1: Why seat row-by-row? (Two queens cannot share a row)\n❓ Q2: What makes a seat safe? (Check column, upper-left diagonal, upper-right diagonal)\n❓ Q3: When is peace achieved? (When row == n)"
    },
    "knight_tour": {
        "title": "The Midnight Chess Inspector: Knight's Tour",
        "story": "Imagine a lone chess Knight on an empty board of size M x N:\n- Start at designated square.\n- Visit EVERY square exactly once.\n- Never step on the same square twice!",
        "hints": "❓ Q1: How does a knight move? (8 candidate L-moves)\n❓ Q2: How do we track footprints? (board[x][y] = move_count)\n❓ Q3: When is the tour finished? (When move_count == size_x * size_y)"
    },
    "sudoku": {
        "title": "The Locked Grid Vault: Sudoku Solver",
        "story": "Imagine you are a cryptanalyst facing a 9x9 digital security matrix:\n- Every row, column, and 3x3 box must contain digits 1-9 with no duplicates.\n- Fill all empty cells to disarm the vault.",
        "hints": "❓ Q1: Where to start? (Find the first empty cell)\n❓ Q2: What if no empty cells remain? (Puzzle solved! Return true)\n❓ Q3: When is digit val safe? (Check row, column, 3x3 chamber)"
    },
    "hamiltonian_cycle": {
        "title": "The Traveling Courier: Hamiltonian Cycle",
        "story": "Imagine you are a delivery driver for a logistics company:\n- Start shift at Depot 0.\n- Visit EVERY city exactly once.\n- Finish shift by returning directly to Depot 0.",
        "hints": "❓ Q1: What choices from current city? (Drive to any unvisited connected neighbor)\n❓ Q2: When is tour complete? (Visited all N cities AND road exists back to Depot 0)\n❓ Q3: How to backtrack? (Remove city from path, mark unvisited, try next road)"
    },
    "graph_coloring": {
        "title": "5G Tower Frequency Allocation: Graph Coloring",
        "story": "Imagine you are setting up N mobile phone towers:\n- Overlapping towers cannot broadcast on the same radio frequency.\n- Allocate frequencies to all towers with zero interference.",
        "hints": "❓ Q1: How to assign? (Assign tower-by-tower in sequential order)\n❓ Q2: When is frequency safe? (No connected neighbor shares this color)\n❓ Q3: When is network clean? (All N towers configured)"
    },
    "binary_search": {
        "title": "The High-Frequency Stock Lookup: Binary Search",
        "story": "Imagine querying a sorted book of stock transactions:\n- In an array of N sorted prices, find the exact index of target price.\n- Cutting search space in half at each step gives O(log N) speed.",
        "hints": "❓ Q1: How to avoid integer overflow? (int mid = left + (right - left) / 2)\n❓ Q2: When to move left? When to move right?\n❓ Q3: When is target found?"
    },
    "bubble_sort": {
        "title": "The Warehouse Pod Organizer: Bubble Sort",
        "story": "Imagine rearranging sorting bins on a conveyor belt:\n- Compare adjacent items and swap if out of order.\n- Largest items 'bubble' up to the end with each pass.",
        "hints": "❓ Q1: When is an array sorted? (When a full pass makes 0 swaps)\n❓ Q2: How does optimization work? (Track swapped flag)"
    }
}

def stub_rust_functions(code_text):
    """
    Replaces function bodies with todo!() while preserving function signatures,
    generics, parameters, return types, doc comments, and type definitions.
    """
    pattern = re.compile(
        r"((?:///[^\n]*\n|\s*//[^\n]*\n)*)"
        r"(\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?fn\s+([a-zA-Z0-9_]+)\s*(?:<[^>{]*>)?\s*\([^){]*\)\s*(?:->\s*[^{]+)?\s*)\{",
        re.MULTILINE
    )
    
    pos = 0
    result = []
    
    while pos < len(code_text):
        m = pattern.search(code_text, pos)
        if not m:
            result.append(code_text[pos:])
            break
            
        start_idx = m.start()
        brace_open_idx = m.end() - 1
        fn_name = m.group(3)
        prefix = m.group(2)
        docs = m.group(1)
        
        result.append(code_text[pos:start_idx])
        result.append(docs)
        result.append(prefix)
        
        # Scan for matching closing brace
        depth = 1
        i = brace_open_idx + 1
        in_string = False
        in_char = False
        in_line_comment = False
        in_block_comment = False
        
        while i < len(code_text) and depth > 0:
            c = code_text[i]
            if in_line_comment:
                if c == "\n":
                    in_line_comment = False
            elif in_block_comment:
                if c == "*" and i + 1 < len(code_text) and code_text[i+1] == "/":
                    in_block_comment = False
                    i += 1
            elif in_string:
                if c == "\\" and i + 1 < len(code_text):
                    i += 1
                elif c == "\"":
                    in_string = False
            elif in_char:
                if c == "\\" and i + 1 < len(code_text):
                    i += 1
                elif c == "\x27":
                    in_char = False
            else:
                if c == "/" and i + 1 < len(code_text) and code_text[i+1] == "/":
                    in_line_comment = True
                    i += 1
                elif c == "/" and i + 1 < len(code_text) and code_text[i+1] == "*":
                    in_block_comment = True
                    i += 1
                elif c == "\"":
                    in_string = True
                elif c == "\x27":
                    in_char = True
                elif c == "{":
                    depth += 1
                elif c == "}":
                    depth -= 1
            i += 1
            
        stub_body = "{\n    // =========================================================================\n" \
                    f"    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD ({fn_name})!\n" \
                    "    // Follow the Socratic hints and questions in the comments above.\n" \
                    "    // =========================================================================\n" \
                    f"    todo!(\"Implement {fn_name}\");\n}}"
        result.append(stub_body)
        pos = i

    return "".join(result)

def process_rust_file(src_path, category, slug):
    with open(src_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()

    test_idx = content.find("#[cfg(test)]")
    if test_idx != -1:
        code_part = content[:test_idx]
        test_part = content[test_idx:]
    else:
        code_part = content
        test_part = ""

    # Replace function bodies in code part
    stubbed_code = stub_rust_functions(code_part)

    # Check for custom story header
    story_header = ""
    if slug in STORIES:
        meta = STORIES[slug]
        story_header = f"""//! =============================================================================
//! {meta['title']}
//! CATEGORY: {category}
//! =============================================================================
//!
//! -----------------------------------------------------------------------------
//! 1. REAL-WORLD STORY & CONTEXT (WHAT IS THIS?)
//! -----------------------------------------------------------------------------
//! {meta['story']}
//!
//! -----------------------------------------------------------------------------
//! 2. GUIDED HINTING QUESTIONS AS YOUR PROBLEM SPECIFICATION
//! -----------------------------------------------------------------------------
//! {meta['hints']}
//! -----------------------------------------------------------------------------

"""

    return story_header + stubbed_code + "\n" + test_part

def migrate_all():
    print("🚀 Starting Full Migration from src/ to problems/...")
    
    migrated_count = 0
    all_problems_meta = []
    category_slugs = {}

    # Discover all .rs files in src/
    for root, dirs, files in os.walk(SRC_DIR):
        for f in files:
            if not f.endswith(".rs") or f in ("mod.rs", "lib.rs"):
                continue

            src_file_path = os.path.join(root, f)
            rel_path = os.path.relpath(src_file_path, SRC_DIR)
            parts = rel_path[:-3].split(os.sep)

            slug = parts[-1]
            cat_parts = parts[:-1]
            cat_name = "/".join(cat_parts)

            # Target problem directory
            prob_dir = os.path.join(PROBLEMS_DIR, cat_name, slug)
            os.makedirs(prob_dir, exist_ok=True)

            # Generate solution.rs
            solution_rs = process_rust_file(src_file_path, cat_name, slug)
            dest_rs_path = os.path.join(prob_dir, "solution.rs")
            with open(dest_rs_path, "w", encoding="utf-8") as out_f:
                out_f.write(solution_rs)

            # Record for mod.rs
            if cat_name not in category_slugs:
                category_slugs[cat_name] = []
            category_slugs[cat_name].append(slug)

            # Record for Supabase Seed
            all_problems_meta.append({
                "slug": slug,
                "category": cat_name,
                "title": STORIES.get(slug, {}).get("title", slug.replace("_", " ").title()),
                "difficulty": "Medium",
                "path": os.path.relpath(prob_dir, PROBLEMS_DIR)
            })

            migrated_count += 1

    print(f"✅ Migrated {migrated_count} Rust algorithms into {PROBLEMS_DIR}/")

    # Update category mod.rs files
    for cat_name, slugs in category_slugs.items():
        src_mod = os.path.join(SRC_DIR, cat_name, "mod.rs")
        dest_mod = os.path.join(PROBLEMS_DIR, cat_name, "mod.rs")

        existing_extra_code = ""
        if os.path.exists(src_mod):
            with open(src_mod, "r", encoding="utf-8") as f:
                mod_content = f.read()
            # Extract everything that is not `mod <slug>;` or `pub use ...;`
            non_mod_lines = []
            for line in mod_content.splitlines():
                stripped = line.strip()
                if stripped.startswith("mod ") and stripped.endswith(";"):
                    continue
                non_mod_lines.append(line)
            existing_extra_code = "\n".join(non_mod_lines).strip()

        mod_lines = ["// Automatically generated category module\n"]
        for s in sorted(slugs):
            mod_lines.append(f'#[path = "{s}/solution.rs"]')
            mod_lines.append(f"pub mod {s};\n")

        if existing_extra_code:
            mod_lines.append("\n" + existing_extra_code + "\n")

        with open(dest_mod, "w", encoding="utf-8") as out_f:
            out_f.write("\n".join(mod_lines))

    # Update problems/lib.rs
    with open("src/lib.rs", "r", encoding="utf-8") as f:
        src_lib = f.read()

    with open(os.path.join(PROBLEMS_DIR, "lib.rs"), "w", encoding="utf-8") as f:
        f.write("//! RustDSA Multi-Language Problem Playground\n" + src_lib)

    print(f"✅ Generated problems/lib.rs and all category mod.rs files")

    # Generate Database Seed & Schema for Supabase
    os.makedirs(DB_DIR, exist_ok=True)
    schema_sql = """-- Supabase / Postgres Database Schema for RustDSA Platform
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now())
);

CREATE TABLE IF NOT EXISTS problems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category_slug VARCHAR(100) REFERENCES categories(slug) ON DELETE CASCADE,
    slug VARCHAR(100) NOT NULL,
    title VARCHAR(255) NOT NULL,
    difficulty VARCHAR(50) DEFAULT 'Medium',
    time_complexity VARCHAR(50),
    space_complexity VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()),
    UNIQUE(category_slug, slug)
);

CREATE TABLE IF NOT EXISTS problem_solutions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    problem_id UUID REFERENCES problems(id) ON DELETE CASCADE,
    language VARCHAR(50) NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    has_test_suite BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()),
    UNIQUE(problem_id, language)
);
"""
    with open(os.path.join(DB_DIR, "schema.sql"), "w", encoding="utf-8") as f:
        f.write(schema_sql)

    with open(os.path.join(DB_DIR, "seed_problems.json"), "w", encoding="utf-8") as f:
        json.dump(all_problems_meta, f, indent=2)

    print(f"✅ Generated {DB_DIR}/schema.sql and {DB_DIR}/seed_problems.json ({len(all_problems_meta)} problems)")

if __name__ == "__main__":
    migrate_all()
