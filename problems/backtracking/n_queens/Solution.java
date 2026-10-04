import java.util.*;

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
}