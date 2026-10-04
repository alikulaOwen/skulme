import heapq

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
