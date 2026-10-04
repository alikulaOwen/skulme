function maxSubArray(nums: number[]): number {
    let curMax = nums[0], globMax = nums[0];
    for (let i = 1; i < nums.length; i++) {
        curMax = Math.max(nums[i], curMax + nums[i]);
        globMax = Math.max(globMax, curMax);
    }
    return globMax;
}

if (maxSubArray([-2, 1, -3, 4, -1, 2, 1, -5, 4]) !== 6) throw new Error("Failed");
console.log("[PASS] TypeScript Maximum Subarray passed!");
