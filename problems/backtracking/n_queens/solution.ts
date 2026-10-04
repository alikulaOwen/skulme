function solveNQueens(n: number): string[][] {
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
