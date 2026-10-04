#!/usr/bin/env python3
import os
import sys
import subprocess

def print_help():
    print("""
================================================================================
                    RustDSA Multi-Language Practice Runner
================================================================================
Usage:
  python3 practice.py list                     List all 394 problems by category
  python3 practice.py list <category>          List problems in a specific category
  python3 practice.py top                      List top high-yield interview problems
  python3 practice.py run <path> <lang>        Run solution tests (java, py, ts, rust)

Examples:
  python3 practice.py run searching/binary_search java
  python3 practice.py run dynamic_programming/coin_change py
  python3 practice.py run sorting/merge_sort ts
  python3 practice.py run graph/dijkstra rust
================================================================================
""")

def get_problem_dir(target):
    target = target.strip().rstrip("/")
    if target.startswith("problems/"):
        prob_dir = target
    else:
        prob_dir = os.path.join("problems", target)
    return prob_dir

def list_categories():
    base = "problems"
    if not os.path.exists(base):
        print("No problems directory found!")
        return
    cats = sorted(os.listdir(base))
    total_problems = 0
    print("\nAvailable Categories & Problem Counts:")
    print("-" * 50)
    for c in cats:
        c_path = os.path.join(base, c)
        if os.path.isdir(c_path):
            probs = []
            for root, dirs, files in os.walk(c_path):
                if "README.md" in files and root != c_path:
                    probs.append(root)
            print(f"  * {c:<25} ({len(probs)} problems)")
            total_problems += len(probs)
    print("-" * 50)
    print(f"Total problems ready for practice: {total_problems}\n")

def list_category_problems(category):
    c_path = os.path.join("problems", category)
    if not os.path.exists(c_path):
        print(f"Category '{category}' not found!")
        return
    print(f"\nProblems in '{category}':")
    print("-" * 50)
    for root, dirs, files in sorted(os.walk(c_path)):
        if "README.md" in files and root != c_path:
            rel = os.path.relpath(root, "problems")
            print(f"  - {rel}")
    print()

def list_top_problems():
    top = [
        ("searching/binary_search", "Binary Search (O(log N))"),
        ("dynamic_programming/coin_change", "Coin Change (Bottom-up DP, O(N*A))"),
        ("dynamic_programming/maximum_subarray", "Maximum Subarray (Kadane's O(N))"),
        ("sorting/merge_sort", "Merge Sort (Divide-and-Conquer, Stable)"),
        ("sorting/quick_sort", "Quick Sort (Partitioning, In-place)"),
        ("graph/breadth_first_search", "Breadth-First Search (Queue BFS, O(V+E))"),
        ("graph/dijkstra", "Dijkstra's Algorithm (Min-Heap PriorityQueue)"),
        ("backtracking/n_queens", "N-Queens (Backtracking & State Restoration)"),
        ("data_structures/trie", "Trie / Prefix Tree (Fast Word & Prefix Lookup)"),
        ("bit_manipulation/find_missing_number", "Missing Number (XOR Bitwise O(1) space)")
    ]
    print("\n================================================================================")
    print("           TOP 10 HIGH-YIELD JAVA SWE INTERVIEW PROBLEMS (MUST-DO)")
    print("================================================================================")
    for path, desc in top:
        print(f"  • {path:<40} -> {desc}")
    print("\nRun any of them with:")
    print("  python3 practice.py run searching/binary_search java\n")

def run_problem(target, lang):
    prob_dir = get_problem_dir(target)
    if not os.path.exists(prob_dir):
        print(f"Error: Problem directory '{prob_dir}' not found!")
        return

    lang = lang.lower()
    print(f"\n[Running {lang.upper()}] in {prob_dir}...\n")

    if lang in ("java", "j"):
        cmd = ["java", "-ea", "Solution.java"]
        res = subprocess.run(cmd, cwd=prob_dir)
        sys.exit(res.returncode)
    elif lang in ("py", "python", "python3"):
        cmd = ["python3", "solution.py"]
        res = subprocess.run(cmd, cwd=prob_dir)
        sys.exit(res.returncode)
    elif lang in ("ts", "typescript", "bun"):
        cmd = ["bun", "solution.ts"]
        res = subprocess.run(cmd, cwd=prob_dir)
        sys.exit(res.returncode)
    elif lang in ("deno",):
        cmd = ["deno", "run", "solution.ts"]
        res = subprocess.run(cmd, cwd=prob_dir)
        sys.exit(res.returncode)
    elif lang in ("rust", "rs"):
        # Map target to cargo module
        rel = os.path.relpath(prob_dir, "problems").replace("/", "::")
        print(f"Running cargo test --lib {rel} from repo root...")
        cmd = ["cargo", "test", "--lib", rel]
        res = subprocess.run(cmd)
        sys.exit(res.returncode)
    else:
        print(f"Unknown language '{lang}'. Choose from: java, py, ts, rust")

def main():
    if len(sys.argv) < 2:
        print_help()
        return

    arg1 = sys.argv[1].lower()
    if arg1 in ("-h", "--help", "help"):
        print_help()
    elif arg1 == "list":
        if len(sys.argv) > 2:
            list_category_problems(sys.argv[2])
        else:
            list_categories()
    elif arg1 == "top":
        list_top_problems()
    elif arg1 == "run":
        if len(sys.argv) < 4:
            print("Usage: python3 practice.py run <category/problem> <java|py|ts|rust>")
            return
        run_problem(sys.argv[2], sys.argv[3])
    else:
        # Convenience: python3 practice.py searching/binary_search java
        if len(sys.argv) >= 3:
            run_problem(sys.argv[1], sys.argv[2])
        else:
            print_help()

if __name__ == "__main__":
    main()
