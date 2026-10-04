import java.util.*;

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
}