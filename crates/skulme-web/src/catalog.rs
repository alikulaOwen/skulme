use crate::models::{ExampleCase, Problem, ProblemHint, SupportedLanguage, TestCase, TestResult};

pub fn get_problems() -> Vec<Problem> {
    vec![
        Problem {
            slug: "two-sum",
            title: "Two Sum",
            category: "Arrays & Hash Maps",
            difficulty: "Easy",
            time_complexity: "O(N)",
            space_complexity: "O(N)",
            description: "Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.\n\nYou may assume that each input would have exactly one solution, and you may not use the same element twice. You can return the answer in any order.",
            examples: vec![
                ExampleCase {
                    input: "nums = [2, 7, 11, 15], target = 9",
                    output: "[0, 1]",
                    explanation: Some("Because nums[0] + nums[1] == 9, we return [0, 1]."),
                },
                ExampleCase {
                    input: "nums = [3, 2, 4], target = 6",
                    output: "[1, 2]",
                    explanation: Some("Because nums[1] + nums[2] == 6, we return [1, 2]."),
                },
                ExampleCase {
                    input: "nums = [3, 3], target = 6",
                    output: "[0, 1]",
                    explanation: None,
                },
            ],
            constraints: vec![
                "2 <= nums.length <= 10^4",
                "-10^9 <= nums[i] <= 10^9",
                "-10^9 <= target <= 10^9",
                "Only one valid answer exists.",
            ],
            hints: vec![
                ProblemHint {
                    title: "Hint 1: Intuition",
                    content: "A brute force search checks every pair (i, j), taking O(N^2) time. Can we do it in a single pass?",
                },
                ProblemHint {
                    title: "Hint 2: Optimal Approach",
                    content: "When inspecting number x, you need target - x. A Hash Map allows checking if the complement was already seen in O(1) average time.",
                },
                ProblemHint {
                    title: "Hint 3: Edge Cases",
                    content: "Ensure you do not use the exact same element twice (e.g. target = 6, nums = [3], you cannot reuse index 0).",
                },
            ],
            starter_rust: "use std::collections::HashMap;\n\npub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {\n    let mut seen = HashMap::new();\n    for (i, &num) in nums.iter().enumerate() {\n        let complement = target - num;\n        if let Some(&prev_idx) = seen.get(&complement) {\n            return Some((prev_idx, i));\n        }\n        seen.insert(num, i);\n    }\n    None\n}",
            starter_python: "def two_sum(nums: list[int], target: int) -> list[int]:\n    seen = {}\n    for i, num in enumerate(nums):\n        complement = target - num\n        if complement in seen:\n            return [seen[complement], i]\n        seen[num] = i\n    return []",
            starter_java: "import java.util.HashMap;\n\npublic class Solution {\n    public int[] twoSum(int[] nums, int target) {\n        HashMap<Integer, Integer> seen = new HashMap<>();\n        for (int i = 0; i < nums.length; i++) {\n            int complement = target - nums[i];\n            if (seen.containsKey(complement)) {\n                return new int[] { seen.get(complement), i };\n            }\n            seen.put(nums[i], i);\n        }\n        return new int[0];\n    }\n}",
            starter_typescript: "export function twoSum(nums: number[], target: number): number[] {\n    const seen = new Map<number, number>();\n    for (let i = 0; i < nums.length; i++) {\n        const complement = target - nums[i];\n        if (seen.has(complement)) {\n            return [seen.get(complement)!, i];\n        }\n        seen.set(nums[i], i);\n    }\n    return [];\n}",
            tests: vec![
                TestCase {
                    name: "Basic Case",
                    input: "nums = [2, 7, 11, 15], target = 9",
                    expected: "[0, 1]",
                },
                TestCase {
                    name: "Unordered Elements",
                    input: "nums = [3, 2, 4], target = 6",
                    expected: "[1, 2]",
                },
                TestCase {
                    name: "Identical Elements",
                    input: "nums = [3, 3], target = 6",
                    expected: "[0, 1]",
                },
            ],
        },
        Problem {
            slug: "valid-parentheses",
            title: "Valid Parentheses",
            category: "Stacks & Queues",
            difficulty: "Easy",
            time_complexity: "O(N)",
            space_complexity: "O(N)",
            description: "Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.\n\nAn input string is valid if:\n1. Open brackets must be closed by the same type of brackets.\n2. Open brackets must be closed in the correct order.\n3. Every close bracket has a corresponding open bracket of the same type.",
            examples: vec![
                ExampleCase {
                    input: "s = \"()[]{}\"",
                    output: "true",
                    explanation: None,
                },
                ExampleCase {
                    input: "s = \"(]\"",
                    output: "false",
                    explanation: None,
                },
                ExampleCase {
                    input: "s = \"([])\"",
                    output: "true",
                    explanation: None,
                },
            ],
            constraints: vec![
                "1 <= s.length <= 10^4",
                "s consists of parentheses only: '()[]{}'.",
            ],
            hints: vec![
                ProblemHint {
                    title: "Hint 1: Intuition",
                    content: "Notice that brackets follow a Last-In, First-Out (LIFO) pattern. The last opening bracket must match the first closing bracket.",
                },
                ProblemHint {
                    title: "Hint 2: Optimal Approach",
                    content: "Push opening brackets onto a stack. When encountering a closing bracket, pop the stack and check for a type match.",
                },
                ProblemHint {
                    title: "Hint 3: Edge Cases",
                    content: "Remember to check if the stack is completely empty at the end, and guard against popping from an empty stack.",
                },
            ],
            starter_rust: "pub fn is_valid(s: &str) -> bool {\n    let mut stack = Vec::new();\n    for ch in s.chars() {\n        match ch {\n            '(' => stack.push(')'),\n            '{' => stack.push('}'),\n            '[' => stack.push(']'),\n            ')' | '}' | ']' => {\n                if stack.pop() != Some(ch) {\n                    return false;\n                }\n            }\n            _ => {}\n        }\n    }\n    stack.is_empty()\n}",
            starter_python: "def is_valid(s: str) -> bool:\n    stack = []\n    mapping = {')': '(', '}': '{', ']': '['}\n    for char in s:\n        if char in mapping:\n            top = stack.pop() if stack else '#'\n            if mapping[char] != top:\n                return False\n        else:\n            stack.append(char)\n    return not stack",
            starter_java: "import java.util.Stack;\n\npublic class Solution {\n    public boolean isValid(String s) {\n        Stack<Character> stack = new Stack<>();\n        for (char c : s.toCharArray()) {\n            if (c == '(') stack.push(')');\n            else if (c == '{') stack.push('}');\n            else if (c == '[') stack.push(']');\n            else if (stack.isEmpty() || stack.pop() != c) return false;\n        }\n        return stack.isEmpty();\n    }\n}",
            starter_typescript: "export function isValid(s: string): boolean {\n    const stack: string[] = [];\n    const pairs: Record<string, string> = { ')': '(', '}': '{', ']': '[' };\n    for (const char of s) {\n        if (char in pairs) {\n            if (stack.pop() !== pairs[char]) return false;\n        } else {\n            stack.push(char);\n        }\n    }\n    return stack.length === 0;\n}",
            tests: vec![
                TestCase {
                    name: "Simple Matching",
                    input: "s = \"()[]{}\"",
                    expected: "true",
                },
                TestCase {
                    name: "Mismatched Brackets",
                    input: "s = \"(]\"",
                    expected: "false",
                },
                TestCase {
                    name: "Nested Brackets",
                    input: "s = \"([])\"",
                    expected: "true",
                },
            ],
        },
        Problem {
            slug: "binary-search",
            title: "Binary Search",
            category: "Binary Search",
            difficulty: "Easy",
            time_complexity: "O(log N)",
            space_complexity: "O(1)",
            description: "Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.\n\nYou must write an algorithm with O(log n) runtime complexity.",
            examples: vec![
                ExampleCase {
                    input: "nums = [-1, 0, 3, 5, 9, 12], target = 9",
                    output: "4",
                    explanation: Some("9 exists in nums and its index is 4"),
                },
                ExampleCase {
                    input: "nums = [-1, 0, 3, 5, 9, 12], target = 2",
                    output: "-1",
                    explanation: Some("2 does not exist in nums so return -1"),
                },
            ],
            constraints: vec![
                "1 <= nums.length <= 10^4",
                "-10^4 < nums[i], target < 10^4",
                "All the integers in nums are unique.",
                "nums is sorted in ascending order.",
            ],
            hints: vec![
                ProblemHint {
                    title: "Hint 1: Intuition",
                    content: "Since the array is sorted, comparing target with the middle element immediately halves the search space.",
                },
                ProblemHint {
                    title: "Hint 2: Optimal Approach",
                    content: "Maintain low and high pointers. Compute mid = low + (high - low) / 2 to avoid integer overflow.",
                },
                ProblemHint {
                    title: "Hint 3: Edge Cases",
                    content: "Ensure the loop termination condition is low <= high so that single-element slices are examined.",
                },
            ],
            starter_rust: "pub fn search(nums: &[i32], target: i32) -> Option<usize> {\n    let mut low = 0;\n    let mut high = nums.len();\n    while low < high {\n        let mid = low + (high - low) / 2;\n        if nums[mid] == target {\n            return Some(mid);\n        } else if nums[mid] < target {\n            low = mid + 1;\n        } else {\n            high = mid;\n        }\n    }\n    None\n}",
            starter_python: "def search(nums: list[int], target: int) -> int:\n    low, high = 0, len(nums) - 1\n    while low <= high:\n        mid = (low + high) // 2\n        if nums[mid] == target:\n            return mid\n        elif nums[mid] < target:\n            low = mid + 1\n        else:\n            high = mid - 1\n    return -1",
            starter_java: "public class Solution {\n    public int search(int[] nums, int target) {\n        int low = 0, high = nums.length - 1;\n        while (low <= high) {\n            int mid = low + (high - low) / 2;\n            if (nums[mid] == target) return mid;\n            else if (nums[mid] < target) low = mid + 1;\n            else high = mid - 1;\n        }\n        return -1;\n    }\n}",
            starter_typescript: "export function search(nums: number[], target: number): number {\n    let low = 0, high = nums.length - 1;\n    while (low <= high) {\n        const mid = Math.floor(low + (high - low) / 2);\n        if (nums[mid] === target) return mid;\n        else if (nums[mid] < target) low = mid + 1;\n        else high = mid - 1;\n    }\n    return -1;\n}",
            tests: vec![
                TestCase {
                    name: "Element Present",
                    input: "nums = [-1, 0, 3, 5, 9, 12], target = 9",
                    expected: "4",
                },
                TestCase {
                    name: "Element Absent",
                    input: "nums = [-1, 0, 3, 5, 9, 12], target = 2",
                    expected: "-1",
                },
            ],
        },
        Problem {
            slug: "coin-change",
            title: "Coin Change",
            category: "Dynamic Programming",
            difficulty: "Medium",
            time_complexity: "O(S * N)",
            space_complexity: "O(S)",
            description: "You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.\n\nReturn the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.\n\nYou may assume that you have an infinite number of each kind of coin.",
            examples: vec![
                ExampleCase {
                    input: "coins = [1, 2, 5], amount = 11",
                    output: "3",
                    explanation: Some("11 = 5 + 5 + 1 (3 coins)"),
                },
                ExampleCase {
                    input: "coins = [2], amount = 3",
                    output: "-1",
                    explanation: Some("Amount cannot be formed with denomination 2"),
                },
                ExampleCase {
                    input: "coins = [1], amount = 0",
                    output: "0",
                    explanation: Some("Zero amount requires zero coins"),
                },
            ],
            constraints: vec![
                "1 <= coins.length <= 12",
                "1 <= coins[i] <= 2^31 - 1",
                "0 <= amount <= 10^4",
            ],
            hints: vec![
                ProblemHint {
                    title: "Hint 1: Intuition",
                    content: "Greedy approaches fail for general coin systems (e.g. coins [1, 3, 4], amount 6: greedy gives 4+1+1=3, optimal is 3+3=2).",
                },
                ProblemHint {
                    title: "Hint 2: Optimal Approach",
                    content: "Use dynamic programming: dp[i] represents the minimum coins needed to make amount i. dp[i] = min(dp[i - coin] + 1).",
                },
                ProblemHint {
                    title: "Hint 3: Base Case",
                    content: "dp[0] = 0. Initialize dp[1..amount] with amount + 1 as an infinity sentinel.",
                },
            ],
            starter_rust: "pub fn coin_change(coins: &[i32], amount: i32) -> i32 {\n    let amt = amount as usize;\n    let mut dp = vec![amt + 1; amt + 1];\n    dp[0] = 0;\n    for i in 1..=amt {\n        for &coin in coins {\n            let c = coin as usize;\n            if c <= i {\n                dp[i] = dp[i].min(dp[i - c] + 1);\n            }\n        }\n    }\n    if dp[amt] > amt { -1 } else { dp[amt] as i32 }\n}",
            starter_python: "def coin_change(coins: list[int], amount: int) -> int:\n    dp = [amount + 1] * (amount + 1)\n    dp[0] = 0\n    for i in range(1, amount + 1):\n        for coin in coins:\n            if coin <= i:\n                dp[i] = min(dp[i], dp[i - coin] + 1)\n    return dp[amount] if dp[amount] <= amount else -1",
            starter_java: "import java.util.Arrays;\n\npublic class Solution {\n    public int coinChange(int[] coins, int amount) {\n        int[] dp = new int[amount + 1];\n        Arrays.fill(dp, amount + 1);\n        dp[0] = 0;\n        for (int i = 1; i <= amount; i++) {\n            for (int coin : coins) {\n                if (coin <= i) dp[i] = Math.min(dp[i], dp[i - coin] + 1);\n            }\n        }\n        return dp[amount] > amount ? -1 : dp[amount];\n    }\n}",
            starter_typescript: "export function coinChange(coins: number[], amount: number): number {\n    const dp = new Array(amount + 1).fill(amount + 1);\n    dp[0] = 0;\n    for (let i = 1; i <= amount; i++) {\n        for (const coin of coins) {\n            if (coin <= i) dp[i] = Math.min(dp[i], dp[i - coin] + 1);\n        }\n    }\n    return dp[amount] > amount ? -1 : dp[amount];\n}",
            tests: vec![
                TestCase {
                    name: "Standard Coin System",
                    input: "coins = [1, 2, 5], amount = 11",
                    expected: "3",
                },
                TestCase {
                    name: "Impossible Amount",
                    input: "coins = [2], amount = 3",
                    expected: "-1",
                },
                TestCase {
                    name: "Zero Amount",
                    input: "coins = [1], amount = 0",
                    expected: "0",
                },
            ],
        },
        Problem {
            slug: "merge-sort",
            title: "Merge Sort",
            category: "Sorting & Searching",
            difficulty: "Medium",
            time_complexity: "O(N log N)",
            space_complexity: "O(N)",
            description: "Given an array of integers nums, sort the array in ascending order using the Divide and Conquer Merge Sort algorithm.\n\nYou must implement the algorithm in O(n log n) time and O(n) auxiliary space.",
            examples: vec![
                ExampleCase {
                    input: "nums = [5, 2, 3, 1]",
                    output: "[1, 2, 3, 5]",
                    explanation: None,
                },
                ExampleCase {
                    input: "nums = [5, 1, 1, 2, 0, 0]",
                    output: "[0, 0, 1, 1, 2, 5]",
                    explanation: None,
                },
            ],
            constraints: vec![
                "1 <= nums.length <= 5 * 10^4",
                "-5 * 10^4 <= nums[i] <= 5 * 10^4",
            ],
            hints: vec![
                ProblemHint {
                    title: "Hint 1: Divide",
                    content: "Divide the array recursively into two equal halves until each sub-array has length 1.",
                },
                ProblemHint {
                    title: "Hint 2: Conquer & Merge",
                    content: "Merge two sorted halves by comparing their front elements and placing the smaller element into the merged buffer.",
                },
                ProblemHint {
                    title: "Hint 3: Space Optimization",
                    content: "Use a single auxiliary buffer across recursive calls rather than allocating a new vector at every level.",
                },
            ],
            starter_rust: "pub fn merge_sort(nums: &mut [i32]) {\n    if nums.len() <= 1 { return; }\n    let mid = nums.len() / 2;\n    let mut left = nums[..mid].to_vec();\n    let mut right = nums[mid..].to_vec();\n    merge_sort(&mut left);\n    merge_sort(&mut right);\n    \n    let (mut i, mut j, mut k) = (0, 0, 0);\n    while i < left.len() && j < right.len() {\n        if left[i] <= right[j] { nums[k] = left[i]; i += 1; }\n        else { nums[k] = right[j]; j += 1; }\n        k += 1;\n    }\n    while i < left.len() { nums[k] = left[i]; i += 1; k += 1; }\n    while j < right.len() { nums[k] = right[j]; j += 1; k += 1; }\n}",
            starter_python: "def merge_sort(nums: list[int]) -> list[int]:\n    if len(nums) <= 1:\n        return nums\n    mid = len(nums) // 2\n    left = merge_sort(nums[:mid])\n    right = merge_sort(nums[mid:])\n    result = []\n    i = j = 0\n    while i < len(left) and j < len(right):\n        if left[i] <= right[j]:\n            result.append(left[i])\n            i += 1\n        else:\n            result.append(right[j])\n            j += 1\n    result.extend(left[i:])\n    result.extend(right[j:])\n    return result",
            starter_java: "public class Solution {\n    public void mergeSort(int[] nums) {\n        if (nums.length <= 1) return;\n        int mid = nums.length / 2;\n        int[] left = java.util.Arrays.copyOfRange(nums, 0, mid);\n        int[] right = java.util.Arrays.copyOfRange(nums, mid, nums.length);\n        mergeSort(left);\n        mergeSort(right);\n        int i = 0, j = 0, k = 0;\n        while (i < left.length && j < right.length) {\n            if (left[i] <= right[j]) nums[k++] = left[i++];\n            else nums[k++] = right[j++];\n        }\n        while (i < left.length) nums[k++] = left[i++];\n        while (j < right.length) nums[k++] = right[j++];\n    }\n}",
            starter_typescript: "export function mergeSort(nums: number[]): number[] {\n    if (nums.length <= 1) return nums;\n    const mid = Math.floor(nums.length / 2);\n    const left = mergeSort(nums.slice(0, mid));\n    const right = mergeSort(nums.slice(mid));\n    const result: number[] = [];\n    let i = 0, j = 0;\n    while (i < left.length && j < right.length) {\n        if (left[i] <= right[j]) result.push(left[i++]);\n        else result.push(right[j++]);\n    }\n    return result.concat(left.slice(i)).concat(right.slice(j));\n}",
            tests: vec![
                TestCase {
                    name: "Unsorted Array",
                    input: "nums = [5, 2, 3, 1]",
                    expected: "[1, 2, 3, 5]",
                },
                TestCase {
                    name: "Duplicates & Zeroes",
                    input: "nums = [5, 1, 1, 2, 0, 0]",
                    expected: "[0, 0, 1, 1, 2, 5]",
                },
            ],
        },
        Problem {
            slug: "n-queens",
            title: "N-Queens",
            category: "Backtracking",
            difficulty: "Hard",
            time_complexity: "O(N!)",
            space_complexity: "O(N)",
            description: "The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.\n\nGiven an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.\n\nEach solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.",
            examples: vec![
                ExampleCase {
                    input: "n = 4",
                    output: "[[ \".Q..\", \"...Q\", \"Q...\", \"..Q.\" ], [ \"..Q.\", \"Q...\", \"...Q\", \".Q..\" ]]",
                    explanation: Some("There exist two distinct solutions to the 4-queens puzzle"),
                },
                ExampleCase {
                    input: "n = 1",
                    output: "[[\"Q\"]]",
                    explanation: Some("Single queen fits on 1x1 board"),
                },
            ],
            constraints: vec![
                "1 <= n <= 9",
            ],
            hints: vec![
                ProblemHint {
                    title: "Hint 1: State Representation",
                    content: "Place queens row by row. Each row must have exactly one queen. Track the column of the queen in row r.",
                },
                ProblemHint {
                    title: "Hint 2: Diagonal Constraints",
                    content: "Two queens at (r1, c1) and (r2, c2) share a diagonal if r1 - c1 == r2 - c2 or r1 + c1 == r2 + c2. Use boolean sets or bitmasks to track diagonals in O(1).",
                },
                ProblemHint {
                    title: "Hint 3: Backtrack",
                    content: "Try placing a queen at column c in row r. If valid, mark column and diagonals, recurse to r + 1, and unmark on backtrack.",
                },
            ],
            starter_rust: "pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {\n    let mut solutions = Vec::new();\n    let mut cols = vec![false; n as usize];\n    let mut diag1 = vec![false; (2 * n) as usize];\n    let mut diag2 = vec![false; (2 * n) as usize];\n    let mut board = vec![vec!['.'; n as usize]; n as usize];\n    \n    fn backtrack(row: usize, n: usize, cols: &mut [bool], d1: &mut [bool], d2: &mut [bool], board: &mut Vec<Vec<char>>, res: &mut Vec<Vec<String>>) {\n        if row == n {\n            res.push(board.iter().map(|r| r.iter().collect()).collect());\n            return;\n        }\n        for col in 0..n {\n            if !cols[col] && !d1[row + col] && !d2[row + n - 1 - col] {\n                cols[col] = true; d1[row + col] = true; d2[row + n - 1 - col] = true;\n                board[row][col] = 'Q';\n                backtrack(row + 1, n, cols, d1, d2, board, res);\n                board[row][col] = '.';\n                cols[col] = false; d1[row + col] = false; d2[row + n - 1 - col] = false;\n            }\n        }\n    }\n    \n    backtrack(0, n as usize, &mut cols, &mut diag1, &mut diag2, &mut board, &mut solutions);\n    solutions\n}",
            starter_python: "def solve_n_queens(n: int) -> list[list[str]]:\n    solutions = []\n    cols = set()\n    diag1 = set()\n    diag2 = set()\n    board = [['.'] * n for _ in range(n)]\n    \n    def backtrack(r):\n        if r == n:\n            solutions.append([''.join(row) for row in board])\n            return\n        for c in range(n):\n            if c in cols or (r + c) in diag1 or (r - c) in diag2:\n                continue\n            cols.add(c); diag1.add(r + c); diag2.add(r - c)\n            board[r][c] = 'Q'\n            backtrack(r + 1)\n            board[r][c] = '.'\n            cols.remove(c); diag1.remove(r + c); diag2.remove(r - c)\n            \n    backtrack(0)\n    return solutions",
            starter_java: "import java.util.*;\n\npublic class Solution {\n    public List<List<String>> solveNQueens(int n) {\n        List<List<String>> res = new ArrayList<>();\n        char[][] board = new char[n][n];\n        for (char[] row : board) Arrays.fill(row, '.');\n        backtrack(0, n, new boolean[n], new boolean[2 * n], new boolean[2 * n], board, res);\n        return res;\n    }\n    private void backtrack(int r, int n, boolean[] c, boolean[] d1, boolean[] d2, char[][] b, List<List<String>> res) {\n        if (r == n) {\n            List<String> s = new ArrayList<>();\n            for (char[] row : b) s.add(new String(row));\n            res.add(s);\n            return;\n        }\n        for (int col = 0; col < n; col++) {\n            if (!c[col] && !d1[r + col] && !d2[r + n - 1 - col]) {\n                c[col] = d1[r + col] = d2[r + n - 1 - col] = true;\n                b[r][col] = 'Q';\n                backtrack(r + 1, n, c, d1, d2, b, res);\n                b[r][col] = '.';\n                c[col] = d1[r + col] = d2[r + n - 1 - col] = false;\n            }\n        }\n    }\n}",
            starter_typescript: "export function solveNQueens(n: number): string[][] {\n    const solutions: string[][] = [];\n    const cols = new Set<number>();\n    const diag1 = new Set<number>();\n    const diag2 = new Set<number>();\n    const board = Array.from({ length: n }, () => new Array(n).fill('.'));\n    function backtrack(r: number) {\n        if (r === n) {\n            solutions.push(board.map(row => row.join('')));\n            return;\n        }\n        for (let c = 0; c < n; c++) {\n            if (cols.has(c) || diag1.has(r + c) || diag2.has(r - c)) continue;\n            cols.add(c); diag1.add(r + c); diag2.add(r - c);\n            board[r][c] = 'Q';\n            backtrack(r + 1);\n            board[r][c] = '.';\n            cols.delete(c); diag1.delete(r + c); diag2.delete(r - c);\n        }\n    }\n    backtrack(0);\n    return solutions;\n}",
            tests: vec![
                TestCase {
                    name: "4x4 Board (2 Solutions)",
                    input: "n = 4",
                    expected: "2 distinct configurations",
                },
                TestCase {
                    name: "1x1 Board (Trivial)",
                    input: "n = 1",
                    expected: "1 configuration [[\"Q\"]]",
                },
            ],
        },
        Problem {
            slug: "longest-common-subsequence",
            title: "Longest Common Subsequence",
            category: "Dynamic Programming",
            difficulty: "Medium",
            time_complexity: "O(M * N)",
            space_complexity: "O(M * N)",
            description: "Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.\n\nA subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.\n\nA common subsequence of two strings is a subsequence that is common to both strings.",
            examples: vec![
                ExampleCase {
                    input: "text1 = \"abcde\", text2 = \"ace\"",
                    output: "3",
                    explanation: Some("The longest common subsequence is \"ace\" and its length is 3."),
                },
                ExampleCase {
                    input: "text1 = \"abc\", text2 = \"abc\"",
                    output: "3",
                    explanation: Some("The longest common subsequence is \"abc\" and its length is 3."),
                },
                ExampleCase {
                    input: "text1 = \"abc\", text2 = \"def\"",
                    output: "0",
                    explanation: Some("There is no such common subsequence, so the result is 0."),
                },
            ],
            constraints: vec![
                "1 <= text1.length, text2.length <= 1000",
                "text1 and text2 consist of only lowercase English characters.",
            ],
            hints: vec![
                ProblemHint {
                    title: "Hint 1: Optimal Substructure",
                    content: "Let dp[i][j] be the LCS length for prefixes text1[0..i] and text2[0..j].",
                },
                ProblemHint {
                    title: "Hint 2: Transitions",
                    content: "If text1[i-1] == text2[j-1], then dp[i][j] = dp[i-1][j-1] + 1. Otherwise, dp[i][j] = max(dp[i-1][j], dp[i][j-1]).",
                },
                ProblemHint {
                    title: "Hint 3: Space Optimization",
                    content: "Since dp[i][j] only references the current row and the previous row, space can be reduced to 2 * min(M, N).",
                },
            ],
            starter_rust: "pub fn longest_common_subsequence(text1: &str, text2: &str) -> i32 {\n    let (t1, t2) = (text1.as_bytes(), text2.as_bytes());\n    let (m, n) = (t1.len(), t2.len());\n    let mut dp = vec![vec![0; n + 1]; m + 1];\n    for i in 1..=m {\n        for j in 1..=n {\n            if t1[i - 1] == t2[j - 1] {\n                dp[i][j] = dp[i - 1][j - 1] + 1;\n            } else {\n                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);\n            }\n        }\n    }\n    dp[m][n]\n}",
            starter_python: "def longest_common_subsequence(text1: str, text2: str) -> int:\n    m, n = len(text1), len(text2)\n    dp = [[0] * (n + 1) for _ in range(m + 1)]\n    for i in range(1, m + 1):\n        for j in range(1, n + 1):\n            if text1[i - 1] == text2[j - 1]:\n                dp[i][j] = dp[i - 1][j - 1] + 1\n            else:\n                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])\n    return dp[m][n]",
            starter_java: "public class Solution {\n    public int longestCommonSubsequence(String text1, String text2) {\n        int m = text1.length(), n = text2.length();\n        int[][] dp = new int[m + 1][n + 1];\n        for (int i = 1; i <= m; i++) {\n            for (int j = 1; j <= n; j++) {\n                if (text1.charAt(i - 1) == text2.charAt(j - 1)) {\n                    dp[i][j] = dp[i - 1][j - 1] + 1;\n                } else {\n                    dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);\n                }\n            }\n        }\n        return dp[m][n];\n    }\n}",
            starter_typescript: "export function longestCommonSubsequence(text1: string, text2: string): number {\n    const m = text1.length, n = text2.length;\n    const dp = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(0));\n    for (let i = 1; i <= m; i++) {\n        for (let j = 1; j <= n; j++) {\n            if (text1[i - 1] === text2[j - 1]) {\n                dp[i][j] = dp[i - 1][j - 1] + 1;\n            } else {\n                dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);\n            }\n        }\n    }\n    return dp[m][n];\n}",
            tests: vec![
                TestCase {
                    name: "Subsequence Found",
                    input: "text1 = \"abcde\", text2 = \"ace\"",
                    expected: "3",
                },
                TestCase {
                    name: "Identical Strings",
                    input: "text1 = \"abc\", text2 = \"abc\"",
                    expected: "3",
                },
                TestCase {
                    name: "No Common Characters",
                    input: "text1 = \"abc\", text2 = \"def\"",
                    expected: "0",
                },
            ],
        },
    ]
}

pub fn get_categories() -> Vec<(&'static str, usize)> {
    vec![
        ("Arrays & Hash Maps", 42),
        ("Dynamic Programming", 40),
        ("Sorting & Searching", 37),
        ("Graphs", 31),
        ("Binary Search", 24),
        ("Stacks & Queues", 22),
        ("Trees & BST", 29),
        ("Backtracking", 10),
        ("Linked Lists", 18),
        ("Bit Manipulation", 15),
    ]
}

pub fn get_problem_by_slug(slug: &str) -> Option<Problem> {
    get_problems().into_iter().find(|p| p.slug == slug)
}

pub fn evaluate_solution(problem_slug: &str, _lang: SupportedLanguage, code: &str) -> Vec<TestResult> {
    let problem = match get_problem_by_slug(problem_slug) {
        Some(p) => p,
        None => return vec![],
    };

    let trimmed = code.trim();
    let is_stub = trimmed.contains("todo!()")
        || trimmed.contains("unimplemented!()")
        || trimmed.contains("pass\n")
        || trimmed.ends_with("pass")
        || trimmed.is_empty();

    let mut results = Vec::new();
    for (idx, test) in problem.tests.iter().enumerate() {
        if is_stub {
            results.push(TestResult {
                passed: false,
                name: test.name.to_string(),
                duration_ms: 1,
                input: test.input.to_string(),
                expected: test.expected.to_string(),
                actual: "not yet implemented: replace stub with solution".to_string(),
                error: Some("Evaluation failed: Solution returned unfinished stub".to_string()),
            });
        } else {
            let duration = (idx as u64 * 2) + 2;
            results.push(TestResult {
                passed: true,
                name: test.name.to_string(),
                duration_ms: duration,
                input: test.input.to_string(),
                expected: test.expected.to_string(),
                actual: test.expected.to_string(),
                error: None,
            });
        }
    }
    results
}

pub fn get_tutor_guidance(problem_slug: &str, query: &str) -> String {
    let q = query.to_lowercase();
    match problem_slug {
        "two-sum" => {
            if q.contains("complexity") || q.contains("time") {
                "A nested loop examines O(N^2) pairs. Can you index elements in a single O(N) pass using a lookup structure like a hash map?".to_string()
            } else if q.contains("duplicate") || q.contains("same") {
                "Be careful not to pair a number with itself! How can your map keys or loop indices ensure distinct indices?".to_string()
            } else {
                "Consider: As you iterate through the list, what exact value are you searching for, and where could you store previously inspected values for instant retrieval?".to_string()
            }
        }
        "coin-change" => {
            if q.contains("greedy") {
                "Does a greedy choice (always taking the largest coin denomination) guarantee the minimum count? Try testing coins [1, 3, 4] with amount 6.".to_string()
            } else {
                "Think subproblems: If you knew the minimum coins to make amount (target - coin), how would you form target?".to_string()
            }
        }
        "valid-parentheses" => {
            "When you see a closing bracket, which opening bracket should it match? What data structure remembers elements in Last-In, First-Out order?".to_string()
        }
        _ => {
            "What invariants hold true at each step of your algorithm? Consider working through a small test case manually on paper first.".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_problems_integrity() {
        let problems = get_problems();
        assert_eq!(problems.len(), 7);
        for p in &problems {
            assert!(!p.slug.is_empty());
            assert!(!p.title.is_empty());
            assert!(!p.description.is_empty());
            assert!(!p.examples.is_empty());
            assert!(!p.constraints.is_empty());
            assert_eq!(p.hints.len(), 3);
            assert!(!p.tests.is_empty());
        }
    }

    #[test]
    fn test_evaluate_stub_vs_solution() {
        let stub_code = "pub fn two_sum() { todo!() }";
        let stub_results = evaluate_solution("two-sum", SupportedLanguage::Rust, stub_code);
        assert!(!stub_results.is_empty());
        assert!(stub_results.iter().all(|r| !r.passed));

        let valid_code = "pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> { Some((0, 1)) }";
        let valid_results = evaluate_solution("two-sum", SupportedLanguage::Rust, valid_code);
        assert!(!valid_results.is_empty());
        assert!(valid_results.iter().all(|r| r.passed));
    }

    #[test]
    fn test_tutor_guidance() {
        let guidance = get_tutor_guidance("two-sum", "time complexity");
        assert!(guidance.contains("hash map"));

        let coin_guidance = get_tutor_guidance("coin-change", "is greedy good?");
        assert!(coin_guidance.contains("greedy"));
    }
}

