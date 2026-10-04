from collections import deque

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
