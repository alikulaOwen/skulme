import java.util.*;

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
}