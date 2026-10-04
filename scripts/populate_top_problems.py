import os

TOP_PROBLEMS = {
    "problems/searching/binary_search": {
        "java": """import java.util.*;

public class Solution {

    /**
     * Binary Search: returns index of target in sorted array, or -1 if not found.
     * Time Complexity: O(log N)
     * Space Complexity: O(1)
     */
    public static int binarySearch(int[] nums, int target) {
        if (nums == null || nums.length == 0) return -1;
        int left = 0, right = nums.length - 1;

        while (left <= right) {
            // Avoid integer overflow vs (left + right) / 2
            int mid = left + (right - left) / 2;

            if (nums[mid] == target) {
                return mid;
            } else if (nums[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        System.out.println("Running Binary Search Tests...");
        int[] sorted = {1, 3, 5, 7, 9, 11, 15, 20};
        assert binarySearch(sorted, 1) == 0 : "Failed: search 1";
        assert binarySearch(sorted, 7) == 3 : "Failed: search 7";
        assert binarySearch(sorted, 20) == 7 : "Failed: search 20";
        assert binarySearch(sorted, 2) == -1 : "Failed: search 2 (missing)";
        assert binarySearch(new int[]{}, 5) == -1 : "Failed: empty array";
        assert binarySearch(new int[]{42}, 42) == 0 : "Failed: single element found";
        assert binarySearch(new int[]{42}, 10) == -1 : "Failed: single element missing";
        System.out.println("[PASS] All Binary Search test cases passed!");
    }
}""",
        "py": """def binary_search(nums: list[int], target: int) -> int:
    left, right = 0, len(nums) - 1
    while left <= right:
        mid = (left + right) // 2
        if nums[mid] == target:
            return mid
        elif nums[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1

if __name__ == "__main__":
    nums = [1, 3, 5, 7, 9, 11, 15, 20]
    assert binary_search(nums, 1) == 0
    assert binary_search(nums, 7) == 3
    assert binary_search(nums, 20) == 7
    assert binary_search(nums, 2) == -1
    print("[PASS] Python Binary Search passed!")
""",
        "ts": """function binarySearch(nums: number[], target: number): number {
    let left = 0, right = nums.length - 1;
    while (left <= right) {
        const mid = left + Math.floor((right - left) / 2);
        if (nums[mid] === target) return mid;
        if (nums[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;
}

const nums = [1, 3, 5, 7, 9, 11, 15, 20];
if (binarySearch(nums, 1) !== 0) throw new Error("Test 1 failed");
if (binarySearch(nums, 7) !== 3) throw new Error("Test 2 failed");
if (binarySearch(nums, 2) !== -1) throw new Error("Test 3 failed");
console.log("[PASS] TypeScript Binary Search passed!");
"""
    },
    "problems/dynamic_programming/coin_change": {
        "java": """import java.util.*;

public class Solution {

    /**
     * Coin Change: returns minimum coins needed to make amount, or -1 if impossible.
     * Time Complexity: O(amount * coins.length)
     * Space Complexity: O(amount)
     */
    public static int coinChange(int[] coins, int amount) {
        if (amount < 0) return -1;
        if (amount == 0) return 0;

        int[] dp = new int[amount + 1];
        Arrays.fill(dp, amount + 1); // Sentinel value
        dp[0] = 0;

        for (int i = 1; i <= amount; i++) {
            for (int coin : coins) {
                if (i >= coin) {
                    dp[i] = Math.min(dp[i], dp[i - coin] + 1);
                }
            }
        }
        return dp[amount] > amount ? -1 : dp[amount];
    }

    public static void main(String[] args) {
        System.out.println("Running Coin Change Tests...");
        assert coinChange(new int[]{1, 2, 5}, 11) == 3 : "Failed: 11 = 5+5+1 (3 coins)";
        assert coinChange(new int[]{2}, 3) == -1 : "Failed: cannot make 3 with coin 2";
        assert coinChange(new int[]{1}, 0) == 0 : "Failed: amount 0 needs 0 coins";
        assert coinChange(new int[]{2, 5, 10, 1}, 27) == 4 : "Failed: 27 = 10+10+5+2 (4 coins)";
        System.out.println("[PASS] All Coin Change test cases passed!");
    }
}""",
        "py": """def coin_change(coins: list[int], amount: int) -> int:
    dp = [amount + 1] * (amount + 1)
    dp[0] = 0
    for i in range(1, amount + 1):
        for c in coins:
            if i >= c:
                dp[i] = min(dp[i], dp[i - c] + 1)
    return dp[amount] if dp[amount] <= amount else -1

if __name__ == "__main__":
    assert coin_change([1, 2, 5], 11) == 3
    assert coin_change([2], 3) == -1
    assert coin_change([1], 0) == 0
    print("[PASS] Python Coin Change passed!")
""",
        "ts": """function coinChange(coins: number[], amount: number): number {
    const dp = new Array(amount + 1).fill(amount + 1);
    dp[0] = 0;
    for (let i = 1; i <= amount; i++) {
        for (const coin of coins) {
            if (i >= coin) {
                dp[i] = Math.min(dp[i], dp[i - coin] + 1);
            }
        }
    }
    return dp[amount] > amount ? -1 : dp[amount];
}

if (coinChange([1, 2, 5], 11) !== 3) throw new Error("Failed 11");
if (coinChange([2], 3) !== -1) throw new Error("Failed 3");
console.log("[PASS] TypeScript Coin Change passed!");
"""
    },
    "problems/dynamic_programming/maximum_subarray": {
        "java": """import java.util.*;

public class Solution {

    /**
     * Kadane's Algorithm: Maximum Subarray Sum
     * Time Complexity: O(N)
     * Space Complexity: O(1)
     */
    public static int maxSubArray(int[] nums) {
        if (nums == null || nums.length == 0) return 0;

        int currentMax = nums[0];
        int globalMax = nums[0];

        for (int i = 1; i < nums.length; i++) {
            currentMax = Math.max(nums[i], currentMax + nums[i]);
            globalMax = Math.max(globalMax, currentMax);
        }
        return globalMax;
    }

    public static void main(String[] args) {
        System.out.println("Running Maximum Subarray Tests...");
        int[] arr1 = {-2, 1, -3, 4, -1, 2, 1, -5, 4};
        assert maxSubArray(arr1) == 6 : "Failed: [4,-1,2,1] has sum 6";
        assert maxSubArray(new int[]{1}) == 1 : "Failed: single positive element";
        assert maxSubArray(new int[]{5, 4, -1, 7, 8}) == 23 : "Failed: sum 23";
        assert maxSubArray(new int[]{-3, -2, -5}) == -2 : "Failed: all negative numbers";
        System.out.println("[PASS] All Maximum Subarray test cases passed!");
    }
}""",
        "py": """def max_sub_array(nums: list[int]) -> int:
    cur_max = glob_max = nums[0]
    for x in nums[1:]:
        cur_max = max(x, cur_max + x)
        glob_max = max(glob_max, cur_max)
    return glob_max

if __name__ == "__main__":
    assert max_sub_array([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6
    assert max_sub_array([-3, -2, -5]) == -2
    print("[PASS] Python Maximum Subarray passed!")
""",
        "ts": """function maxSubArray(nums: number[]): number {
    let curMax = nums[0], globMax = nums[0];
    for (let i = 1; i < nums.length; i++) {
        curMax = Math.max(nums[i], curMax + nums[i]);
        globMax = Math.max(globMax, curMax);
    }
    return globMax;
}

if (maxSubArray([-2, 1, -3, 4, -1, 2, 1, -5, 4]) !== 6) throw new Error("Failed");
console.log("[PASS] TypeScript Maximum Subarray passed!");
"""
    },
    "problems/sorting/merge_sort": {
        "java": """import java.util.*;

public class Solution {

    /**
     * Merge Sort: Stable, Divide-and-Conquer
     * Time Complexity: O(N log N) in all cases
     * Space Complexity: O(N)
     */
    public static void mergeSort(int[] arr) {
        if (arr == null || arr.length <= 1) return;
        int[] temp = new int[arr.length];
        sort(arr, 0, arr.length - 1, temp);
    }

    private static void sort(int[] arr, int left, int right, int[] temp) {
        if (left >= right) return;
        int mid = left + (right - left) / 2;
        sort(arr, left, mid, temp);
        sort(arr, mid + 1, right, temp);
        merge(arr, left, mid, right, temp);
    }

    private static void merge(int[] arr, int left, int mid, int right, int[] temp) {
        int i = left, j = mid + 1, k = left;
        while (i <= mid && j <= right) {
            if (arr[i] <= arr[j]) { // <= preserves stability
                temp[k++] = arr[i++];
            } else {
                temp[k++] = arr[j++];
            }
        }
        while (i <= mid) temp[k++] = arr[i++];
        while (j <= right) temp[k++] = arr[j++];
        for (int p = left; p <= right; p++) arr[p] = temp[p];
    }

    public static void main(String[] args) {
        System.out.println("Running Merge Sort Tests...");
        int[] arr = {12, 11, 13, 5, 6, 7};
        mergeSort(arr);
        assert Arrays.equals(arr, new int[]{5, 6, 7, 11, 12, 13}) : "Failed merge sort";
        
        int[] sorted = {1, 2, 3, 4};
        mergeSort(sorted);
        assert Arrays.equals(sorted, new int[]{1, 2, 3, 4});

        int[] dupes = {4, 2, 2, 1, 4};
        mergeSort(dupes);
        assert Arrays.equals(dupes, new int[]{1, 2, 2, 4, 4});
        System.out.println("[PASS] All Merge Sort test cases passed!");
    }
}""",
        "py": """def merge_sort(arr: list[int]) -> list[int]:
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    
    merged = []
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            merged.append(left[i])
            i += 1
        else:
            merged.append(right[j])
            j += 1
    merged.extend(left[i:])
    merged.extend(right[j:])
    return merged

if __name__ == "__main__":
    assert merge_sort([12, 11, 13, 5, 6, 7]) == [5, 6, 7, 11, 12, 13]
    print("[PASS] Python Merge Sort passed!")
""",
        "ts": """function mergeSort(arr: number[]): number[] {
    if (arr.length <= 1) return arr;
    const mid = Math.floor(arr.length / 2);
    const left = mergeSort(arr.slice(0, mid));
    const right = mergeSort(arr.slice(mid));
    const res: number[] = [];
    let i = 0, j = 0;
    while (i < left.length && j < right.length) {
        if (left[i] <= right[j]) res.push(left[i++]);
        else res.push(right[j++]);
    }
    return res.concat(left.slice(i)).concat(right.slice(j));
}

const sorted = mergeSort([12, 11, 13, 5, 6, 7]);
if (JSON.stringify(sorted) !== JSON.stringify([5, 6, 7, 11, 12, 13])) throw new Error("Failed");
console.log("[PASS] TypeScript Merge Sort passed!");
"""
    },
    "problems/sorting/quick_sort": {
        "java": """import java.util.*;

public class Solution {

    /**
     * Quick Sort: In-place, Divide-and-Conquer
     * Time Complexity: O(N log N) average, O(N^2) worst
     * Space Complexity: O(log N) stack
     */
    public static void quickSort(int[] arr) {
        if (arr == null || arr.length <= 1) return;
        quickSort(arr, 0, arr.length - 1);
    }

    private static void quickSort(int[] arr, int low, int high) {
        if (low < high) {
            int p = partition(arr, low, high);
            quickSort(arr, low, p - 1);
            quickSort(arr, p + 1, high);
        }
    }

    private static int partition(int[] arr, int low, int high) {
        int pivot = arr[high];
        int i = low - 1;
        for (int j = low; j < high; j++) {
            if (arr[j] <= pivot) {
                i++;
                swap(arr, i, j);
            }
        }
        swap(arr, i + 1, high);
        return i + 1;
    }

    private static void swap(int[] arr, int i, int j) {
        int t = arr[i];
        arr[i] = arr[j];
        arr[j] = t;
    }

    public static void main(String[] args) {
        System.out.println("Running Quick Sort Tests...");
        int[] arr = {10, 7, 8, 9, 1, 5};
        quickSort(arr);
        assert Arrays.equals(arr, new int[]{1, 5, 7, 8, 9, 10}) : "Failed quick sort";
        System.out.println("[PASS] All Quick Sort test cases passed!");
    }
}""",
        "py": """def quick_sort(arr: list[int]) -> list[int]:
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quick_sort(left) + middle + quick_sort(right)

if __name__ == "__main__":
    assert quick_sort([10, 7, 8, 9, 1, 5]) == [1, 5, 7, 8, 9, 10]
    print("[PASS] Python Quick Sort passed!")
""",
        "ts": """function quickSort(arr: number[]): number[] {
    if (arr.length <= 1) return arr;
    const pivot = arr[Math.floor(arr.length / 2)];
    const left = arr.filter(x => x < pivot);
    const mid = arr.filter(x => x === pivot);
    const right = arr.filter(x => x > pivot);
    return quickSort(left).concat(mid).concat(quickSort(right));
}

if (JSON.stringify(quickSort([10, 7, 8, 9, 1, 5])) !== JSON.stringify([1, 5, 7, 8, 9, 10])) throw new Error("Failed");
console.log("[PASS] TypeScript Quick Sort passed!");
"""
    },
    "problems/graph/breadth_first_search": {
        "java": """import java.util.*;

public class Solution {

    /**
     * Breadth-First Search (BFS): Level-order graph traversal
     * Time Complexity: O(V + E)
     * Space Complexity: O(V)
     */
    public static List<Integer> bfs(Map<Integer, List<Integer>> graph, int start) {
        List<Integer> visitedOrder = new ArrayList<>();
        Set<Integer> visited = new HashSet<>();
        // Use ArrayDeque for BFS queue (faster than LinkedList)
        Queue<Integer> queue = new ArrayDeque<>();

        queue.offer(start);
        visited.add(start);

        while (!queue.isEmpty()) {
            int node = queue.poll();
            visitedOrder.add(node);

            List<Integer> neighbors = graph.getOrDefault(node, Collections.emptyList());
            for (int neighbor : neighbors) {
                if (!visited.contains(neighbor)) {
                    visited.add(neighbor);
                    queue.offer(neighbor);
                }
            }
        }
        return visitedOrder;
    }

    public static void main(String[] args) {
        System.out.println("Running BFS Tests...");
        Map<Integer, List<Integer>> graph = new HashMap<>();
        graph.put(0, Arrays.asList(1, 2));
        graph.put(1, Arrays.asList(2));
        graph.put(2, Arrays.asList(0, 3));
        graph.put(3, Arrays.asList(3));

        List<Integer> order = bfs(graph, 2);
        assert order.equals(Arrays.asList(2, 0, 3, 1)) : "Failed BFS order";
        System.out.println("[PASS] All BFS test cases passed!");
    }
}""",
        "py": """from collections import deque

def bfs(graph: dict[int, list[int]], start: int) -> list[int]:
    visited = set([start])
    queue = deque([start])
    order = []
    while queue:
        node = queue.popleft()
        order.append(node)
        for neighbor in graph.get(node, []):
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)
    return order

if __name__ == "__main__":
    g = {0: [1, 2], 1: [2], 2: [0, 3], 3: [3]}
    assert bfs(g, 2) == [2, 0, 3, 1]
    print("[PASS] Python BFS passed!")
""",
        "ts": """function bfs(graph: Record<number, number[]>, start: number): number[] {
    const visited = new Set<number>([start]);
    const queue = [start];
    const order: number[] = [];
    while (queue.length > 0) {
        const node = queue.shift()!;
        order.push(node);
        for (const neighbor of graph[node] || []) {
            if (!visited.has(neighbor)) {
                visited.add(neighbor);
                queue.push(neighbor);
            }
        }
    }
    return order;
}

const g = {0: [1, 2], 1: [2], 2: [0, 3], 3: [3]};
if (JSON.stringify(bfs(g, 2)) !== JSON.stringify([2, 0, 3, 1])) throw new Error("Failed");
console.log("[PASS] TypeScript BFS passed!");
"""
    },
    "problems/graph/dijkstra": {
        "java": """import java.util.*;

public class Solution {

    public static class Edge {
        int to, weight;
        public Edge(int to, int weight) {
            this.to = to;
            this.weight = weight;
        }
    }

    /**
     * Dijkstra's Shortest Path Algorithm
     * Time Complexity: O((V + E) log V)
     * Space Complexity: O(V)
     */
    public static Map<Integer, Integer> dijkstra(Map<Integer, List<Edge>> graph, int source) {
        Map<Integer, Integer> dist = new HashMap<>();
        // PriorityQueue stores [node, distance], min-heap ordered by distance
        PriorityQueue<int[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> a[1]));

        dist.put(source, 0);
        pq.offer(new int[]{source, 0});

        while (!pq.isEmpty()) {
            int[] curr = pq.poll();
            int u = curr[0];
            int d = curr[1];

            // If we found a shorter path already, skip
            if (d > dist.getOrDefault(u, Integer.MAX_VALUE)) continue;

            for (Edge edge : graph.getOrDefault(u, Collections.emptyList())) {
                int nextDist = d + edge.weight;
                if (nextDist < dist.getOrDefault(edge.to, Integer.MAX_VALUE)) {
                    dist.put(edge.to, nextDist);
                    pq.offer(new int[]{edge.to, nextDist});
                }
            }
        }
        return dist;
    }

    public static void main(String[] args) {
        System.out.println("Running Dijkstra Tests...");
        Map<Integer, List<Edge>> graph = new HashMap<>();
        graph.computeIfAbsent(0, k -> new ArrayList<>()).add(new Edge(1, 4));
        graph.computeIfAbsent(0, k -> new ArrayList<>()).add(new Edge(2, 1));
        graph.computeIfAbsent(2, k -> new ArrayList<>()).add(new Edge(1, 2));
        graph.computeIfAbsent(1, k -> new ArrayList<>()).add(new Edge(3, 1));
        graph.computeIfAbsent(2, k -> new ArrayList<>()).add(new Edge(3, 5));

        Map<Integer, Integer> dists = dijkstra(graph, 0);
        assert dists.get(0) == 0;
        assert dists.get(2) == 1;
        assert dists.get(1) == 3 : "0 -> 2 -> 1 is 1 + 2 = 3";
        assert dists.get(3) == 4 : "0 -> 2 -> 1 -> 3 is 4";
        System.out.println("[PASS] All Dijkstra test cases passed!");
    }
}""",
        "py": """import heapq

def dijkstra(graph: dict[int, list[tuple[int, int]]], source: int) -> dict[int, int]:
    dist = {source: 0}
    pq = [(0, source)]
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist.get(u, float('inf')):
            continue
        for v, weight in graph.get(u, []):
            if d + weight < dist.get(v, float('inf')):
                dist[v] = d + weight
                heapq.heappush(pq, (d + weight, v))
    return dist

if __name__ == "__main__":
    g = {
        0: [(1, 4), (2, 1)],
        2: [(1, 2), (3, 5)],
        1: [(3, 1)]
    }
    d = dijkstra(g, 0)
    assert d[1] == 3
    assert d[3] == 4
    print("[PASS] Python Dijkstra passed!")
""",
        "ts": """function dijkstra(graph: Record<number, Array<[number, number]>>, source: number): Record<number, number> {
    const dist: Record<number, number> = { [source]: 0 };
    const queue: Array<[number, number]> = [[0, source]];

    while (queue.length > 0) {
        queue.sort((a, b) => a[0] - b[0]);
        const [d, u] = queue.shift()!;
        if (d > (dist[u] ?? Infinity)) continue;
        for (const [v, w] of graph[u] || []) {
            if (d + w < (dist[v] ?? Infinity)) {
                dist[v] = d + w;
                queue.push([d + w, v]);
            }
        }
    }
    return dist;
}

const g = {0: [[1, 4], [2, 1]], 2: [[1, 2], [3, 5]], 1: [[3, 1]]};
const res = dijkstra(g as any, 0);
if (res[1] !== 3 || res[3] !== 4) throw new Error("Failed");
console.log("[PASS] TypeScript Dijkstra passed!");
"""
    },
    "problems/backtracking/n_queens": {
        "java": """import java.util.*;

public class Solution {

    /**
     * N-Queens: place N non-attacking queens on an N x N chessboard.
     * Time Complexity: O(N!)
     * Space Complexity: O(N)
     */
    public static List<List<String>> solveNQueens(int n) {
        List<List<String>> solutions = new ArrayList<>();
        char[][] board = new char[n][n];
        for (int i = 0; i < n; i++) Arrays.fill(board[i], '.');

        Set<Integer> cols = new HashSet<>();
        Set<Integer> diag1 = new HashSet<>(); // row - col
        Set<Integer> diag2 = new HashSet<>(); // row + col

        backtrack(0, n, board, solutions, cols, diag1, diag2);
        return solutions;
    }

    private static void backtrack(int row, int n, char[][] board, List<List<String>> solutions,
                                  Set<Integer> cols, Set<Integer> diag1, Set<Integer> diag2) {
        if (row == n) {
            List<String> valid = new ArrayList<>();
            for (char[] r : board) valid.add(new String(r));
            solutions.add(valid);
            return;
        }

        for (int col = 0; col < n; col++) {
            if (cols.contains(col) || diag1.contains(row - col) || diag2.contains(row + col)) {
                continue;
            }

            board[row][col] = 'Q';
            cols.add(col);
            diag1.add(row - col);
            diag2.add(row + col);

            backtrack(row + 1, n, board, solutions, cols, diag1, diag2);

            // Backtrack
            board[row][col] = '.';
            cols.remove(col);
            diag1.remove(row - col);
            diag2.remove(row + col);
        }
    }

    public static void main(String[] args) {
        System.out.println("Running N-Queens Tests...");
        List<List<String>> sol4 = solveNQueens(4);
        assert sol4.size() == 2 : "4-Queens should have exactly 2 solutions";

        List<List<String>> sol1 = solveNQueens(1);
        assert sol1.size() == 1 : "1-Queen has 1 solution";

        List<List<String>> sol8 = solveNQueens(8);
        assert sol8.size() == 92 : "8-Queens should have exactly 92 solutions";
        System.out.println("[PASS] All N-Queens test cases passed!");
    }
}""",
        "py": """def solve_n_queens(n: int) -> list[list[str]]:
    solutions = []
    cols, diag1, diag2 = set(), set(), set()
    board = [['.'] * n for _ in range(n)]

    def backtrack(r):
        if r == n:
            solutions.append(["".join(row) for row in board])
            return
        for c in range(n):
            if c in cols or (r - c) in diag1 or (r + c) in diag2:
                continue
            board[r][c] = 'Q'
            cols.add(c); diag1.add(r - c); diag2.add(r + c)
            backtrack(r + 1)
            board[r][c] = '.'
            cols.remove(c); diag1.remove(r - c); diag2.remove(r + c)

    backtrack(0)
    return solutions

if __name__ == "__main__":
    assert len(solve_n_queens(4)) == 2
    assert len(solve_n_queens(8)) == 92
    print("[PASS] Python N-Queens passed!")
""",
        "ts": """function solveNQueens(n: number): string[][] {
    const solutions: string[][] = [];
    const cols = new Set<number>();
    const diag1 = new Set<number>();
    const diag2 = new Set<number>();
    const board = Array.from({length: n}, () => Array(n).fill('.'));

    function backtrack(r: number) {
        if (r === n) {
            solutions.push(board.map(row => row.join('')));
            return;
        }
        for (let c = 0; c < n; c++) {
            if (cols.has(c) || diag1.has(r - c) || diag2.has(r + c)) continue;
            board[r][c] = 'Q';
            cols.add(c); diag1.add(r - c); diag2.add(r + c);
            backtrack(r + 1);
            board[r][c] = '.';
            cols.delete(c); diag1.delete(r - c); diag2.delete(r + c);
        }
    }
    backtrack(0);
    return solutions;
}

if (solveNQueens(4).length !== 2) throw new Error("Failed");
console.log("[PASS] TypeScript N-Queens passed!");
"""
    },
    "problems/data_structures/trie": {
        "java": """import java.util.*;

public class Solution {

    public static class TrieNode {
        Map<Character, TrieNode> children = new HashMap<>();
        boolean isEndOfWord = false;
    }

    public static class Trie {
        private final TrieNode root = new TrieNode();

        public void insert(String word) {
            TrieNode current = root;
            for (char ch : word.toCharArray()) {
                current = current.children.computeIfAbsent(ch, c -> new TrieNode());
            }
            current.isEndOfWord = true;
        }

        public boolean search(String word) {
            TrieNode node = findNode(word);
            return node != null && node.isEndOfWord;
        }

        public boolean startsWith(String prefix) {
            return findNode(prefix) != null;
        }

        private TrieNode findNode(String str) {
            TrieNode current = root;
            for (char ch : str.toCharArray()) {
                current = current.children.get(ch);
                if (current == null) return null;
            }
            return current;
        }
    }

    public static void main(String[] args) {
        System.out.println("Running Trie Tests...");
        Trie trie = new Trie();
        trie.insert("apple");
        assert trie.search("apple") : "Failed: search apple";
        assert !trie.search("app") : "Failed: app is not whole word yet";
        assert trie.startsWith("app") : "Failed: startsWith app";
        trie.insert("app");
        assert trie.search("app") : "Failed: search app after insert";
        System.out.println("[PASS] All Trie test cases passed!");
    }
}""",
        "py": """class Trie:
    def __init__(self):
        self.root = {}

    def insert(self, word: str) -> None:
        cur = self.root
        for ch in word:
            cur = cur.setdefault(ch, {})
        cur["#"] = True

    def search(self, word: str) -> bool:
        cur = self.root
        for ch in word:
            if ch not in cur: return False
            cur = cur[ch]
        return "#" in cur

    def starts_with(self, prefix: str) -> bool:
        cur = self.root
        for ch in prefix:
            if ch not in cur: return False
            cur = cur[ch]
        return True

if __name__ == "__main__":
    t = Trie()
    t.insert("apple")
    assert t.search("apple") is True
    assert t.search("app") is False
    assert t.starts_with("app") is True
    t.insert("app")
    assert t.search("app") is True
    print("[PASS] Python Trie passed!")
""",
        "ts": """class Trie {
    root: Record<string, any> = {};

    insert(word: string): void {
        let cur = this.root;
        for (const ch of word) {
            cur[ch] = cur[ch] || {};
            cur = cur[ch];
        }
        cur["#"] = true;
    }

    search(word: string): boolean {
        let cur = this.root;
        for (const ch of word) {
            if (!cur[ch]) return false;
            cur = cur[ch];
        }
        return !!cur["#"];
    }

    startsWith(prefix: string): boolean {
        let cur = this.root;
        for (const ch of prefix) {
            if (!cur[ch]) return false;
            cur = cur[ch];
        }
        return true;
    }
}

const t = new Trie();
t.insert("apple");
if (!t.search("apple") || t.search("app") || !t.startsWith("app")) throw new Error("Failed");
console.log("[PASS] TypeScript Trie passed!");
"""
    },
    "problems/bit_manipulation/find_missing_number": {
        "java": """import java.util.*;

public class Solution {

    /**
     * Missing Number in [0..n]
     * Uses XOR property: a ^ a = 0, a ^ 0 = a
     * Time Complexity: O(N)
     * Space Complexity: O(1)
     */
    public static int missingNumber(int[] nums) {
        int xor = nums.length;
        for (int i = 0; i < nums.length; i++) {
            xor ^= i ^ nums[i];
        }
        return xor;
    }

    public static void main(String[] args) {
        System.out.println("Running Missing Number Tests...");
        assert missingNumber(new int[]{3, 0, 1}) == 2 : "Failed [3, 0, 1]";
        assert missingNumber(new int[]{0, 1}) == 2 : "Failed [0, 1]";
        assert missingNumber(new int[]{9,6,4,2,3,5,7,0,1}) == 8 : "Failed missing 8";
        System.out.println("[PASS] All Missing Number test cases passed!");
    }
}""",
        "py": """def missing_number(nums: list[int]) -> int:
    xor = len(nums)
    for i, x in enumerate(nums):
        xor ^= i ^ x
    return xor

if __name__ == "__main__":
    assert missing_number([3, 0, 1]) == 2
    assert missing_number([0, 1]) == 2
    print("[PASS] Python Missing Number passed!")
""",
        "ts": """function missingNumber(nums: number[]): number {
    let xor = nums.length;
    for (let i = 0; i < nums.length; i++) {
        xor ^= i ^ nums[i];
    }
    return xor;
}

if (missingNumber([3, 0, 1]) !== 2) throw new Error("Failed");
console.log("[PASS] TypeScript Missing Number passed!");
"""
    }
}

for path, files in TOP_PROBLEMS.items():
    if os.path.exists(path):
        with open(os.path.join(path, "Solution.java"), "w", encoding="utf-8") as f:
            f.write(files["java"])
        with open(os.path.join(path, "solution.py"), "w", encoding="utf-8") as f:
            f.write(files["py"])
        with open(os.path.join(path, "solution.ts"), "w", encoding="utf-8") as f:
            f.write(files["ts"])
        print(f"Populated top problem: {path}")

print("Completed populating top interview problems!")
