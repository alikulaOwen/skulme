function bfs(graph: Record<number, number[]>, start: number): number[] {
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
