function coinChange(coins: number[], amount: number): number {
    const dp = new Array(amount + 1).fill(amount + 1);
    dp[0] = 0;
    for (let i = 1; i <= amount; i++) {
        for (const coin of coins) {
            if (i >= coin) {
                dp[i] = Math.min(dp[i], dp[i - coin] + 1);
            }
        }
    }
    return dp[amount] > amount ? -1 : dp[amount];
}

if (coinChange([1, 2, 5], 11) !== 3) throw new Error("Failed 11");
if (coinChange([2], 3) !== -1) throw new Error("Failed 3");
console.log("[PASS] TypeScript Coin Change passed!");
