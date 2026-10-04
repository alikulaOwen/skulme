function dijkstra(graph: Record<number, Array<[number, number]>>, source: number): Record<number, number> {
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
