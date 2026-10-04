import java.util.*;

public class Solution {

    /**
     * Coin Change: returns minimum coins needed to make amount, or -1 if impossible.
     * Time Complexity: O(amount * coins.length)
     * Space Complexity: O(amount)
     */
    public static int coinChange(int[] coins, int amount) {
        if (amount < 0) return -1;
        if (amount == 0) return 0;

        int[] dp = new int[amount + 1];
        Arrays.fill(dp, amount + 1); // Sentinel value
        dp[0] = 0;

        for (int i = 1; i <= amount; i++) {
            for (int coin : coins) {
                if (i >= coin) {
                    dp[i] = Math.min(dp[i], dp[i - coin] + 1);
                }
            }
        }
        return dp[amount] > amount ? -1 : dp[amount];
    }

    public static void main(String[] args) {
        System.out.println("Running Coin Change Tests...");
        assert coinChange(new int[]{1, 2, 5}, 11) == 3 : "Failed: 11 = 5+5+1 (3 coins)";
        assert coinChange(new int[]{2}, 3) == -1 : "Failed: cannot make 3 with coin 2";
        assert coinChange(new int[]{1}, 0) == 0 : "Failed: amount 0 needs 0 coins";
        assert coinChange(new int[]{2, 5, 10, 1}, 27) == 4 : "Failed: 27 = 10+10+5+2 (4 coins)";
        System.out.println("[PASS] All Coin Change test cases passed!");
    }
}