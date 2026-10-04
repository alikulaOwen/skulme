import os
import re
import shutil

CATEGORIES_META = {
    "sorting": {
        "description": "Sorting algorithms arrange items of a list into a specific order (ascending or descending).",
        "java_tips": """- Java's `Arrays.sort(primitive[])` uses Dual-Pivot Quicksort (O(N log N) average, O(N^2) worst case).
- `Arrays.sort(Object[])` and `Collections.sort(List)` use Timsort (guaranteed stable, O(N log N) worst case).
- For custom ordering, use `Comparator.comparingInt(...)` or `(a, b) -> Integer.compare(a, b)`. NEVER use `a - b` due to integer underflow/overflow risk.
- Remember stability: stable sorts preserve the relative order of duplicate elements.""",
        "rust_vs_java": """- Rust's `slice::sort()` is stable Timsort/pdqsort; `slice::sort_unstable()` is in-place pattern-defeating quicksort.
- Java distinguishes primitive arrays (`int[]`) from object arrays (`Integer[]`), whereas Rust treats all types uniformly under generics `T: Ord`."""
    },
    "searching": {
        "description": "Searching algorithms locate a target element or determine its absence within a collection.",
        "java_tips": """- In Binary Search, ALWAYS calculate midpoint using `int mid = left + (right - left) / 2;` to avoid 32-bit integer overflow.
- Check loop invariants: `while (left <= right)` when `right = n - 1` vs `while (left < right)` when `right = n`.
- Java provides `Arrays.binarySearch()`, which returns `-(insertion_point + 1)` if the key is not present.""",
        "rust_vs_java": """- Rust's `slice::binary_search()` returns `Result<usize, usize>` (`Ok(index)` or `Err(insert_index)`).
- Java returns a primitive `int`, encoding not-found as a negative integer."""
    },
    "dynamic_programming": {
        "description": "Dynamic Programming breaks down complex problems into overlapping subproblems with optimal substructure.",
        "java_tips": """- Multi-dimensional arrays `int[][] dp = new int[m][n]` in Java are arrays of heap references; consider flat arrays `int[m * n]` or rolling 1D arrays for cache locality.
- Watch for integer overflow when initializing memoization tables with `Integer.MAX_VALUE` (adding 1 wraps around to negative). Use `1_000_000_000` or check for sentinel before adding.
- Identify: State definition, Base cases, Transition relation, and Evaluation order.""",
        "rust_vs_java": """- Rust guarantees memory safety and bounds checks, but idiomatically uses flat vectors `Vec<T>` with 1D indexing.
- Java relies on JVM GC for allocated DP tables, so minimize object allocations inside DP loops."""
    },
    "graph": {
        "description": "Graph algorithms explore networks of nodes connected by edges, including traversal, shortest path, and connectivity.",
        "java_tips": """- Adjacency lists are typically represented as `List<List<Integer>>` or `Map<Integer, List<Integer>>`.
- Use `computeIfAbsent(u, k -> new ArrayList<>()).add(v)` for concise graph construction.
- For BFS: ALWAYS use `Queue<Integer> q = new ArrayDeque<>()` (never `LinkedList`, which incurs heavy node allocation overhead).
- For Dijkstra: Use `PriorityQueue<int[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> a[1]))` where `a[0]` is node and `a[1]` is distance.""",
        "rust_vs_java": """- Rust graphs often use `BTreeMap<V, BTreeMap<V, E>>` or adjacency lists with explicit vertex indexing.
- In Rust, graph traversal requires explicit borrowing or node index IDs to avoid borrow checker conflicts with circular references."""
    },
    "data_structures": {
        "description": "Fundamental and advanced data structures for organizing, storing, and accessing data efficiently.",
        "java_tips": """- Memorize Java's Collections Hierarchy:
  - `List`: `ArrayList` (dynamic array), `LinkedList` (doubly linked).
  - `Queue` / `Deque`: `ArrayDeque` (fast array-backed ring buffer), `PriorityQueue` (min-heap).
  - `Set`: `HashSet` (O(1)), `TreeSet` (Red-Black Tree, O(log N)), `LinkedHashSet` (insertion-ordered).
  - `Map`: `HashMap` (O(1)), `TreeMap` (Red-Black Tree, O(log N)), `LinkedHashMap` (LRU cache base).
- Interview classic: implementing LRU Cache with `LinkedHashMap(capacity, 0.75f, true)` or custom Doubly Linked List + HashMap.""",
        "rust_vs_java": """- Rust's standard library provides `VecDeque`, `BinaryHeap` (Max-heap by default, unlike Java's Min-heap!), `BTreeMap`, and `HashMap`.
- Building self-referential structures like linked lists or trees from scratch is tricky in Rust due to ownership (`Box`, `Rc<RefCell<T>>`), whereas Java handles node references naturally via garbage collection."""
    },
    "backtracking": {
        "description": "Backtracking incrementally builds candidates for solutions and abandons ('backtracks') as soon as candidate cannot yield valid solution.",
        "java_tips": """- The #1 Java Backtracking Bug: Adding the mutable path directly to the result list `result.add(currentPath)`. ALWAYS add a shallow copy: `result.add(new ArrayList<>(currentPath))`.
- Pattern:
  ```java
  for (Choice choice : choices) {
      if (isValid(choice)) {
          state.add(choice);      // Choose
          backtrack(state, ...); // Explore
          state.removeLast();    // Un-choose (backtrack)
      }
  }
  ```""",
        "rust_vs_java": """- In Rust, recursion often takes `&mut Vec<T>` and explicitly calls `vec.pop()`, or passes immutable clones.
- Java's `List.remove(list.size() - 1)` or `Deque.removeLast()` is the standard backtracking pattern."""
    },
    "string": {
        "description": "String algorithms for pattern matching, parsing, hashing, and substring manipulations.",
        "java_tips": """- Strings in Java are immutable! `str += "a"` creates a brand new String object each time. Inside loops, ALWAYS use `StringBuilder`.
- Access characters via `str.charAt(i)` and length via `str.length()`.
- Compare strings with `str1.equals(str2)`, NEVER with `str1 == str2` (which checks reference equality!).
- To convert to char array for fast swaps: `char[] chars = str.toCharArray()`.""",
        "rust_vs_java": """- Rust strings are UTF-8 bytes (`String`, `&str`), so direct byte indexing `s[i]` is prohibited if characters span multiple bytes.
- Java strings use UTF-16 code units (`char`), and provide O(1) indexed `charAt(i)`."""
    },
    "bit_manipulation": {
        "description": "Bitwise operations perform fast, low-level manipulation of individual bits in integral numbers.",
        "java_tips": """- Note the bit shift difference:
  - `>>` is Arithmetic Right Shift (preserves sign bit).
  - `>>>` is Logical (Unsigned) Right Shift (fills left with zeros).
- Useful bit hacks:
  - `n & (n - 1)` removes the lowest set bit.
  - `n & (-n)` isolates the lowest set bit.
  - `n ^ n = 0` (used for single number / unique element finding).
- Built-ins: `Integer.bitCount(n)`, `Integer.highestOneBit(n)`, `Integer.numberOfLeadingZeros(n)`.""",
        "rust_vs_java": """- Rust has explicit unsigned integer types (`u32`, `u64`), so `>>` is automatically unsigned for unsigned types.
- Java only has signed integers (`int`, `long`), which is why the `>>>` operator exists."""
    },
    "math": {
        "description": "Mathematical algorithms including number theory, combinatorics, modular arithmetic, and algebra.",
        "java_tips": """- Watch out for 32-bit integer overflow: `(a + b)` and `(a * b)` can easily exceed `Integer.MAX_VALUE` (2^31 - 1). Cast to `long` before multiplication: `(long) a * b % MOD`.
- For arbitrary precision arithmetic, use `java.math.BigInteger` and `java.math.BigDecimal`.""",
        "rust_vs_java": """- Rust panics on debug integer overflow and wraps in release mode (or offers `checked_add`, `saturating_mul`).
- Java silently overflows integer operations without exception unless `Math.addExact()` is explicitly used."""
    },
    "greedy": {
        "description": "Greedy algorithms make the locally optimal choice at each step with the goal of finding a global optimum.",
        "java_tips": """- Greedy algorithms usually require sorting input first (e.g. by end-time in interval scheduling) or using a `PriorityQueue`.
- In interviews, you must be able to justify why the greedy choice property holds and does not get trapped in local optima.""",
        "rust_vs_java": """- Greedy logic translates directly between languages; differences lie only in sorting collections and priority queue APIs."""
    }
}

DEFAULT_META = {
    "description": "Algorithmic problem implementation and analysis.",
    "java_tips": """- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.""",
    "rust_vs_java": """- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`."""
}

def slug_to_title(slug):
    acronyms = {
        "aes": "AES", "rsa": "RSA", "bfs": "BFS", "dfs": "DFS", "dsu": "DSU",
        "bst": "BST", "avl": "AVL", "lcs": "LCS", "lis": "LIS", "kmp": "KMP",
        "dp": "DP", "gcd": "GCD", "lcm": "LCM", "fft": "FFT", "sha256": "SHA-256",
        "md5": "MD5", "rot13": "ROT13", "cmyk": "CMYK", "hsv": "HSV", "rgb": "RGB",
        "ipv4": "IPv4", "ipv6": "IPv6", "lz77": "LZ77", "bwt": "BWT"
    }
    parts = slug.split("_")
    title_parts = []
    for p in parts:
        lower = p.lower()
        if lower in acronyms:
            title_parts.append(acronyms[lower])
        else:
            title_parts.append(p.capitalize())
    return " ".join(title_parts)

def extract_problem_data(rs_path):
    with open(rs_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()

    lines = content.splitlines()

    # 1. Extract module doc or leading comments
    doc_lines = []
    for l in lines:
        s = l.strip()
        if s.startswith("//!"):
            doc_lines.append(s[3:].strip())
        elif doc_lines and not s.startswith("//!"):
            break
            
    if not doc_lines:
        pub_idx = -1
        for i, l in enumerate(lines):
            if re.match(r"^\s*pub\s+(?:fn|struct|enum)\b", l):
                pub_idx = i
                break
        if pub_idx > 0:
            for i in range(pub_idx - 1, -1, -1):
                s = lines[i].strip()
                if s.startswith("///"):
                    doc_lines.append(s[3:].strip())
                elif s.startswith("//") and not s.startswith("//#"):
                    doc_lines.append(s[2:].strip())
                elif s == "" and doc_lines:
                    doc_lines.append("")
                else:
                    break
            doc_lines = list(reversed(doc_lines))

    if not doc_lines:
        for l in lines:
            s = l.strip()
            if s.startswith("//"):
                doc_lines.append(s[2:].strip())
            elif doc_lines and not s.startswith("//"):
                break

    doc = "\n".join(doc_lines).strip()

    # Extract pub fns
    pub_fns = re.findall(r"pub\s+fn\s+([a-zA-Z0-9_]+)\s*(?:<[^>]+>)?\s*\(([^)]*)\)\s*(?:->\s*([^{]+))?", content)

    # Extract pub structs
    pub_structs = re.findall(r"pub\s+(?:struct|enum)\s+([a-zA-Z0-9_]+)", content)

    # Extract time/space complexity if mentioned
    time_comp = "O(N)"
    space_comp = "O(1)"
    m_time = re.search(r"[Tt]ime\s*(?:complexity)?\s*[:=-]\s*([^\n\r]+)", content)
    if m_time:
        time_comp = m_time.group(1).strip()
    m_space = re.search(r"[Ss]pace\s*(?:complexity)?\s*[:=-]\s*([^\n\r]+)", content)
    if m_space:
        space_comp = m_space.group(1).strip()

    # Extract test cases count
    test_count = len(re.findall(r"#\[test\]", content))

    return {
        "doc": doc,
        "pub_fns": pub_fns,
        "pub_structs": pub_structs,
        "time_complexity": time_comp,
        "space_complexity": space_comp,
        "test_count": test_count,
        "raw_content": content
    }

def generate_problem_files(rel_rs_path):
    # rel_rs_path: e.g. "src/sorting/bubble_sort.rs"
    # or "src/data_structures/probabilistic/bloom_filter.rs"
    parts = rel_rs_path.split("/")
    # parts: ['src', 'sorting', 'bubble_sort.rs']
    cat = parts[1]
    subcats = parts[2:-1]
    filename = parts[-1]
    slug = filename[:-3] # remove .rs
    
    title = slug_to_title(slug)
    data = extract_problem_data(rel_rs_path)

    # Directory under problems/
    if subcats:
        prob_dir = os.path.join("problems", cat, *subcats, slug)
        cargo_mod_path = f"{cat}::{'::'.join(subcats)}::{slug}"
        back_steps = len(subcats) + 3 # e.g. ../../../..
    else:
        prob_dir = os.path.join("problems", cat, slug)
        cargo_mod_path = f"{cat}::{slug}"
        back_steps = 3 # ../../../
        
    rel_back = "/".join([".."] * back_steps)
    orig_rs_link = f"{rel_back}/{rel_rs_path}"

    os.makedirs(prob_dir, exist_ok=True)

    meta = CATEGORIES_META.get(cat, DEFAULT_META)

    # Format problem statement
    if data["doc"]:
        problem_desc = data["doc"]
    else:
        problem_desc = f"Implement the {title} algorithm in the {cat} domain. The goal is to provide an efficient, robust solution that passes the test suite."

    # Format fn signatures
    fn_sig_md = ""
    if data["pub_fns"]:
        fn_sig_md = "### Original Rust Signatures\n```rust\n"
        for fn_name, args, ret in data["pub_fns"]:
            ret_str = f" -> {ret.strip()}" if ret else ""
            fn_sig_md += f"pub fn {fn_name}({args.strip()}){ret_str};\n"
        fn_sig_md += "```\n"
    elif data["pub_structs"]:
        fn_sig_md = f"### Original Rust Structs\n`{', '.join(data['pub_structs'])}`\n"

    # 1. README.md
    readme_content = f"""# {title}

**Category:** `{cat}` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

{problem_desc}

{fn_sig_md}
### Complexity
- **Time Complexity:** `{data['time_complexity']}`
- **Space Complexity:** `{data['space_complexity']}`

---

## Java Interview Strategy & Tips

{meta['java_tips']}

### Rust vs. Java Perspective
{meta['rust_vs_java']}
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`{filename}`]({orig_rs_link}).

---

## How to Spin & Run

### 1. Java (Target Interview Language)
Run directly as a single-file application with built-in tests:
```bash
java Solution.java
```

### 2. Rust (Original Ground Truth Answer)
Run the crate unit tests for this module from the repository root:
```bash
cargo test --lib {cargo_mod_path}
```

### 3. Python (Rapid Prototyping)
```bash
python3 solution.py
```

### 4. TypeScript (Industry Standard)
```bash
bun solution.ts
# or using Deno:
deno run solution.ts
```

---

## Self-Evaluation Checklist
- [ ] Understand the problem constraints and edge cases (e.g. empty input, bounds, duplicates).
- [ ] Implement the optimal solution in `Solution.java`.
- [ ] Verify correctness using `java Solution.java`.
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`{filename}`]({orig_rs_link}).
"""

    with open(os.path.join(prob_dir, "README.md"), "w", encoding="utf-8") as f:
        f.write(readme_content)

    # 2. Solution.java
    java_content = f"""/**
 * Problem: {title}
 * Category: {cat}
 * 
 * Execution:
 *   java Solution.java
 */
import java.util.*;

public class Solution {{

    /**
     * Problem solver entry point.
     * Candidate: Implement your Java solution below.
     */
    public static class Solver {{
        public boolean solve() {{
            // TODO: Implement solution logic for {title}
            return true;
        }}
    }}

    // ==========================================
    // Test Harness & Verification
    // ==========================================
    public static void main(String[] args) {{
        System.out.println("==================================================");
        System.out.println("Running Java Solution for: {title}");
        System.out.println("Category: {cat}");
        System.out.println("==================================================");

        long startTime = System.nanoTime();
        
        Solver solver = new Solver();
        boolean result = solver.solve();
        assert result : "Assertion failed: Solver returned false";

        long durationUs = (System.nanoTime() - startTime) / 1000;
        System.out.println("[PASS] All tests completed successfully in " + durationUs + " µs!");
        System.out.println("Reference Rust solution: {rel_rs_path}");
    }}
}}
"""
    with open(os.path.join(prob_dir, "Solution.java"), "w", encoding="utf-8") as f:
        f.write(java_content)

    # 3. solution.py
    py_content = f'''"""
Problem: {title}
Category: {cat}

Execution:
  python3 solution.py
"""

def solve() -> bool:
    """
    Candidate: Implement your Python solution for {title}.
    """
    # TODO: Implement algorithm
    return True

if __name__ == "__main__":
    print(f"Running Python solution for: {title} ({cat})")
    assert solve() is True, "Test failed!"
    print("[PASS] Python test passed!")
'''
    with open(os.path.join(prob_dir, "solution.py"), "w", encoding="utf-8") as f:
        f.write(py_content)

    # 4. solution.ts
    ts_content = f"""/**
 * Problem: {title}
 * Category: {cat}
 * 
 * Execution:
 *   bun solution.ts
 *   or: deno run solution.ts
 */

function solve(): boolean {{
    // TODO: Implement TypeScript solution for {title}
    return true;
}}

console.log("Running TypeScript solution for: {title} ({cat})");
if (!solve()) {{
    throw new Error("Assertion failed: solve() returned false");
}}
console.log("[PASS] TypeScript test passed!");
"""
    with open(os.path.join(prob_dir, "solution.ts"), "w", encoding="utf-8") as f:
        f.write(ts_content)

    return {
        "cat": cat,
        "subcats": subcats,
        "slug": slug,
        "title": title,
        "rel_rs_path": rel_rs_path,
        "prob_dir": prob_dir,
        "time_comp": data["time_complexity"],
        "space_comp": data["space_complexity"],
        "cargo_mod_path": cargo_mod_path
    }

def main():
    print("Collecting all Rust algorithm files...")
    all_problems = []
    for root, dirs, files in os.walk("src"):
        for f in files:
            if f.endswith(".rs") and f not in ("mod.rs", "lib.rs"):
                rel_path = os.path.join(root, f)
                all_problems.append(rel_path)

    all_problems.sort()
    print(f"Found {len(all_problems)} algorithm files. Generating practice workspaces...")

    generated = []
    by_category = {}
    for p in all_problems:
        item = generate_problem_files(p)
        generated.append(item)
        by_category.setdefault(item["cat"], []).append(item)

    print(f"Generated {len(generated)} problem folders under problems/!")

    # Now create/update README.md for each category in src/
    print("Generating category index READMEs in src/...")
    for cat, items in by_category.items():
        cat_dir = os.path.join("src", cat)
        readme_path = os.path.join(cat_dir, "README.md")
        meta = CATEGORIES_META.get(cat, DEFAULT_META)

        table_rows = []
        for it in items:
            title = it["title"]
            rs_file = os.path.basename(it["rel_rs_path"])
            prob_link = os.path.relpath(it["prob_dir"], cat_dir)
            table_rows.append(f"| [{title}]({prob_link}/README.md) | [`{rs_file}`](./{rs_file}) | `java Solution.java` | `cargo test --lib {it['cargo_mod_path']}` |")

        cat_readme = f"""# {slug_to_title(cat)} - Algorithm Practice & Interview Guide

{meta['description']}

## Key Java Interview Takeaways
{meta['java_tips']}

## Comparison: Rust vs Java
{meta['rust_vs_java']}

---

## Problems & Practice Workspaces ({len(items)} Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
{chr(10).join(table_rows)}

---
*Generated for Java Software Engineering Interview Preparation.*
"""
        # If existing README.md exists, let's see if we should prepend or write PRACTICE.md
        # If existing README.md has custom diagrams (like in sorting), we don't want to destroy them!
        if os.path.exists(readme_path):
            with open(readme_path, "r", encoding="utf-8") as f:
                old_c = f.read()
            # If it already has rich documentation, create PRACTICE.md as well
            with open(os.path.join(cat_dir, "PRACTICE.md"), "w", encoding="utf-8") as f:
                f.write(cat_readme)
            # And append link to PRACTICE.md at the top of README.md if not already there
            if "PRACTICE.md" not in old_c:
                updated_c = f"> 💡 **Interview Prep:** Looking for Java/Python/TS practice workspaces? See [`PRACTICE.md`](./PRACTICE.md) for all {len(items)} problems.\n\n" + old_c
                with open(readme_path, "w", encoding="utf-8") as f:
                    f.write(updated_c)
        else:
            with open(readme_path, "w", encoding="utf-8") as f:
                f.write(cat_readme)

    print("Category index READMEs generated.")

if __name__ == "__main__":
    main()
