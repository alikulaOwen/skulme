def solve_n_queens(n: int) -> list[list[str]]:
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
