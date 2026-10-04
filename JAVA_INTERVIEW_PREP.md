# Java Software Engineer Interview Fast-Track & DSA Handbook

> **Context:** Tailored specifically for engineers with deep Rust, Python (MScFE), and TypeScript experience who need to master Java Data Structures & Algorithms for an interview tomorrow.

---

## 🎯 Executive Game Plan for Tomorrow

1. **Focus on High-Yield Patterns**: Software engineering interviews do not test obscure algorithms; they test **Breadth-First Search, Depth-First Search, Binary Search, Sliding Window/Two Pointers, Hash Maps, Dynamic Programming (1D/2D), and Backtracking**.
2. **Translate Your Existing Mental Model**: You already know algorithms from Rust, Python, and TypeScript. You only need the **Java syntax, standard library collections, and interview idioms**.
3. **Avoid the Top 3 Java Interview Traps**:
   - `==` vs `.equals()` (Never use `==` to compare Strings, Objects, or boxed Integers!).
   - `s += c` in loops (Always use `StringBuilder` to avoid O(N²) quadratic allocations).
   - In Backtracking: `result.add(new ArrayList<>(path))` (Never `result.add(path)`).

---

## 🔄 The Multi-Language Rosetta Stone

| Concept | Rust | Python | TypeScript | Java (Standard Interview Form) |
| :--- | :--- | :--- | :--- | :--- |
| **Variable** | `let x = 5;` | `x = 5` | `const x = 5;` | `int x = 5;` (or `var x = 5;`) |
| **Constant** | `const X: i32 = 5;` | `X = 5` | `const X = 5;` | `static final int X = 5;` |
| **Dynamic List** | `Vec<T>` | `list` (`[]`) | `Array<T>` (`[]`) | `List<T> list = new ArrayList<>();` |
| **Fixed Array** | `[T; N]` | N/A | `[T, T]` | `int[] arr = new int[n];` |
| **Hash Map** | `HashMap<K, V>` | `dict` (`{}`) | `Map<K, V>` / `{}` | `Map<K, V> map = new HashMap<>();` |
| **Hash Set** | `HashSet<T>` | `set` (`set()`) | `Set<T>` | `Set<T> set = new HashSet<>();` |
| **Double Queue / Deque**| `VecDeque<T>` | `collections.deque` | `Array<T>` | `Deque<T> dq = new ArrayDeque<>();` |
| **Min-Heap** | `BinaryHeap` *(Max!)* | `heapq` *(Min)* | Custom PQ | `PriorityQueue<T> pq = new PriorityQueue<>();` |
| **Max-Heap** | `BinaryHeap` *(Standard)*| `heapq` *(-val)* | Custom PQ | `PriorityQueue<T> pq = new PriorityQueue<>(Collections.reverseOrder());` |
| **Nullable / Optional**| `Option<T>` | `None` | `null \| undefined` | `null` (or `Optional<T>`) |
| **Error / Result** | `Result<T, E>` | `try ... except` | `try ... catch` | `try { ... } catch (Exception e)` |
| **Sorting** | `slice.sort()` | `arr.sort()` | `arr.sort((a,b)=>a-b)`| `Arrays.sort(arr);` / `Collections.sort(list);` |
| **String Mutation** | `String::push_str` | `"".join(parts)` | `parts.join("")` | `StringBuilder sb = new StringBuilder();` |

---

## ⚠️ The Top 5 Java Interview Pitfalls

### 1. Object vs. Primitive Equality: `==` vs `.equals()`
- `==` compares **primitive values** (`3 == 3` is `true`), but for objects, it compares **memory references**!
```java
// TRAP:
String a = new String("hello");
String b = new String("hello");
boolean wrong = (a == b);        // FALSE! (Different memory addresses)
boolean right = a.equals(b);     // TRUE! (Compares character contents)

// Integer Cache Trap (-128 to 127):
Integer x = 127, y = 127;
boolean ok = (x == y);          // TRUE (Cached by JVM)

Integer c = 128, d = 128;
boolean trap = (c == d);        // FALSE! (Beyond 127, distinct objects allocated)
boolean correct = c.equals(d);  // TRUE! Always use .equals() for Objects!
```

### 2. String Concatenation in Loops (O(N) vs O(N²))
In Java, `String` is immutable. Every `s += c` creates a brand new string and copies all previous characters:
```java
// BAD: O(N^2) time complexity
String s = "";
for (char c : chars) s += c;

// GOOD: O(N) amortized
StringBuilder sb = new StringBuilder();
for (char c : chars) sb.append(c);
String s = sb.toString();
```

### 3. Backtracking Reference Mutation
```java
// TRAP:
List<List<Integer>> result = new ArrayList<>();
List<Integer> path = new ArrayList<>();
// ...
result.add(path); // BUG! You added a pointer to path. When you backtrack, result is ruined!

// FIX: Always add a snapshot copy!
result.add(new ArrayList<>(path));
```

### 4. Custom Comparator Integer Underflow
```java
// RISKY: Can overflow if subtraction crosses Integer.MIN_VALUE or MAX_VALUE:
Arrays.sort(intervals, (a, b) -> a[0] - b[0]); 

// SAFE & IDIOMATIC:
Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
```

### 5. Midpoint Calculation Overflow in Binary Search
```java
// RISKY: (left + right) can overflow 32-bit signed int:
int mid = (left + right) / 2;

// SAFE:
int mid = left + (right - left) / 2;
```

---

## 🧰 Java Collections & APIs Quick Reference

### Arrays & Primitives
```java
int[] arr = new int[n];                // Default initialized to 0
int[] nums = {1, 2, 3, 4, 5};
int len = nums.length;                 // Note: .length (field, not method)
Arrays.sort(nums);                     // In-place Dual-Pivot Quicksort
Arrays.fill(arr, -1);                  // Fill entire array
int idx = Arrays.binarySearch(nums, 3);// Returns index, or -(insertionPoint + 1)
int[] copy = Arrays.copyOf(nums, len); // Fast shallow copy
```

### List (`ArrayList`)
```java
List<Integer> list = new ArrayList<>();
list.add(10);                          // Append O(1)
list.add(0, 5);                        // Insert at index O(N)
int val = list.get(0);                 // Access O(1)
list.set(0, 20);                       // Modify O(1)
list.remove(list.size() - 1);          // Remove last element O(1)
int size = list.size();                // Size check
boolean empty = list.isEmpty();
Collections.sort(list);                // Timsort O(N log N)
```

### Map (`HashMap`)
```java
Map<String, Integer> map = new HashMap<>();
map.put("key", 1);
int count = map.getOrDefault("key", 0);
boolean hasKey = map.containsKey("key");
map.remove("key");

// PRO-TIP FOR GRAPHS & FREQUENCY COUNTS:
map.put(word, map.getOrDefault(word, 0) + 1);

// Graph adjacency list construction:
Map<Integer, List<Integer>> graph = new HashMap<>();
graph.computeIfAbsent(u, k -> new ArrayList<>()).add(v);

// Iterating a Map:
for (Map.Entry<String, Integer> entry : map.entrySet()) {
    String k = entry.getKey();
    int v = entry.getValue();
}
```

### Set (`HashSet` & `TreeSet`)
```java
Set<Integer> set = new HashSet<>();    // O(1) average lookup
set.add(42);
boolean exists = set.contains(42);
set.remove(42);

// Ordered Set (Red-Black Tree, O(log N)):
TreeSet<Integer> treeSet = new TreeSet<>();
Integer higher = treeSet.higher(10);   // Smallest element > 10
Integer lower = treeSet.lower(10);     // Largest element < 10
```

### Queue & Stack (`ArrayDeque`)
> ⚠️ **Interviewer Tip:** Never use `java.util.Stack` or `java.util.LinkedList` for queue/stack operations in interviews. Modern Java uses `ArrayDeque` because it is cache-friendly and array-backed with zero node allocation overhead!
```java
// Queue (FIFO for BFS):
Queue<Integer> queue = new ArrayDeque<>();
queue.offer(val);                      // Push to back
int front = queue.poll();              // Pop from front
int peek = queue.peek();               // Inspect front

// Stack (LIFO for DFS / Monotonic Stacks):
Deque<Integer> stack = new ArrayDeque<>();
stack.push(val);                       // Push to top
int top = stack.pop();                 // Pop from top
int peekTop = stack.peek();            // Inspect top
```

### PriorityQueue (Min-Heap / Max-Heap)
```java
// Default: MIN-HEAP (smallest element polled first)
PriorityQueue<Integer> minHeap = new PriorityQueue<>();
minHeap.offer(10);
minHeap.offer(5);
int minVal = minHeap.poll();           // 5

// MAX-HEAP:
PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Collections.reverseOrder());

// Custom Comparator (e.g. for Dijkstra: [node, distance]):
PriorityQueue<int[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> a[1]));
```

### String & Character
```java
String s = "Hello World";
int len = s.length();                  // Note: .length() method!
char ch = s.charAt(0);                 // 'H'
char[] chars = s.toCharArray();        // Convert to mutable char[]
String sub = s.substring(0, 5);        // "Hello" (end index is exclusive!)
boolean starts = s.startsWith("He");
String[] tokens = s.split(" ");

// Character helpers:
Character.isLetterOrDigit(ch);
Character.toLowerCase(ch);
Character.isDigit(ch);
```

---

## 🏆 Top 10 High-Yield Interview Problems (Must-Practice)

Every problem has a complete practice workspace. You can run the Java code, Python, TypeScript, or Rust tests using `./practice.py`:

| Problem | Category | Complexity | Quick Spin Command |
| :--- | :--- | :--- | :--- |
| **[Binary Search](problems/searching/binary_search/README.md)** | `searching` | O(log N) Time, O(1) Space | `python3 practice.py run searching/binary_search java` |
| **[Coin Change](problems/dynamic_programming/coin_change/README.md)** | `dynamic_programming` | O(N·A) Time, O(A) Space | `python3 practice.py run dynamic_programming/coin_change java` |
| **[Maximum Subarray (Kadane)](problems/dynamic_programming/maximum_subarray/README.md)** | `dynamic_programming` | O(N) Time, O(1) Space | `python3 practice.py run dynamic_programming/maximum_subarray java` |
| **[Merge Sort](problems/sorting/merge_sort/README.md)** | `sorting` | O(N log N) Time, O(N) Space | `python3 practice.py run sorting/merge_sort java` |
| **[Quick Sort](problems/sorting/quick_sort/README.md)** | `sorting` | O(N log N) Time, O(log N) Space | `python3 practice.py run sorting/quick_sort java` |
| **[Breadth-First Search](problems/graph/breadth_first_search/README.md)** | `graph` | O(V + E) Time, O(V) Space | `python3 practice.py run graph/breadth_first_search java` |
| **[Dijkstra's Algorithm](problems/graph/dijkstra/README.md)** | `graph` | O((V + E) log V) Time, O(V) Space | `python3 practice.py run graph/dijkstra java` |
| **[N-Queens](problems/backtracking/n_queens/README.md)** | `backtracking` | O(N!) Time, O(N) Space | `python3 practice.py run backtracking/n_queens java` |
| **[Trie / Prefix Tree](problems/data_structures/trie/README.md)** | `data_structures` | O(L) Insert/Search, O(Σ·L) Space | `python3 practice.py run data_structures/trie java` |
| **[Missing Number](problems/bit_manipulation/find_missing_number/README.md)** | `bit_manipulation` | O(N) Time, O(1) Space | `python3 practice.py run bit_manipulation/find_missing_number java` |

---

## 💻 How to Use the Practice Workspace

### Option A: Interactive CLI Runner (`practice.py`)
```bash
# List all 394 problems grouped by category
python3 practice.py list

# List problems in a category
python3 practice.py list dynamic_programming

# Run Java tests
python3 practice.py run dynamic_programming/coin_change java

# Run Python tests
python3 practice.py run dynamic_programming/coin_change py

# Run TypeScript tests
python3 practice.py run dynamic_programming/coin_change ts

# Run Rust reference tests from crate
python3 practice.py run dynamic_programming/coin_change rust
```

### Option B: Direct Folder Execution
Navigate into any problem directory:
```bash
cd problems/dynamic_programming/coin_change

# Run Java executable (OpenJDK 27 executes single-file directly!):
java -ea Solution.java

# Run Python:
python3 solution.py

# Run TypeScript:
bun solution.ts

# Compare with Rust original answer:
view README.md (links directly to src/dynamic_programming/coin_change.rs)
```

---

## ⚡ Concurrency & Modern Java (Senior SWE Interview Prep)

If the interview covers general software engineering and system architecture:
1. **Virtual Threads (Project Loom / Java 21+)**:
   - `Thread.ofVirtual().start(() -> doWork());`
   - Replaces traditional 1:1 OS threads with lightweight M:N virtual threads. Allows spinning up 1,000,000 concurrent threads with minimal memory footprint.
2. **Thread Safety**:
   - `ConcurrentHashMap` uses segmented locks/CAS operations (never locks the entire map).
   - `AtomicInteger`, `AtomicLong` use hardware compare-and-swap (CAS) without blocking.
   - `volatile` guarantees memory visibility across threads, preventing CPU cache staleness, but does *not* provide mutual exclusion for compound operations (`count++`).
3. **Garbage Collection (JVM)**:
   - Generational Hypothesis: Most objects die young (Eden -> Survivor S0/S1 -> Tenured / Old Gen).
   - Modern GCs: G1GC (default), ZGC (sub-millisecond pause times for large heaps).
