import java.util.*;

public class Solution {

    /**
     * Kadane's Algorithm: Maximum Subarray Sum
     * Time Complexity: O(N)
     * Space Complexity: O(1)
     */
    public static int maxSubArray(int[] nums) {
        if (nums == null || nums.length == 0) return 0;

        int currentMax = nums[0];
        int globalMax = nums[0];

        for (int i = 1; i < nums.length; i++) {
            currentMax = Math.max(nums[i], currentMax + nums[i]);
            globalMax = Math.max(globalMax, currentMax);
        }
        return globalMax;
    }

    public static void main(String[] args) {
        System.out.println("Running Maximum Subarray Tests...");
        int[] arr1 = {-2, 1, -3, 4, -1, 2, 1, -5, 4};
        assert maxSubArray(arr1) == 6 : "Failed: [4,-1,2,1] has sum 6";
        assert maxSubArray(new int[]{1}) == 1 : "Failed: single positive element";
        assert maxSubArray(new int[]{5, 4, -1, 7, 8}) == 23 : "Failed: sum 23";
        assert maxSubArray(new int[]{-3, -2, -5}) == -2 : "Failed: all negative numbers";
        System.out.println("[PASS] All Maximum Subarray test cases passed!");
    }
}