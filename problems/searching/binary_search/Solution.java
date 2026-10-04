import java.util.*;

public class Solution {

    /**
     * Binary Search: returns index of target in sorted array, or -1 if not found.
     * Time Complexity: O(log N)
     * Space Complexity: O(1)
     */
    public static int binarySearch(int[] nums, int target) {
        if (nums == null || nums.length == 0) return -1;
        int left = 0, right = nums.length - 1;

        while (left <= right) {
            // Avoid integer overflow vs (left + right) / 2
            int mid = left + (right - left) / 2;

            if (nums[mid] == target) {
                return mid;
            } else if (nums[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        System.out.println("Running Binary Search Tests...");
        int[] sorted = {1, 3, 5, 7, 9, 11, 15, 20};
        assert binarySearch(sorted, 1) == 0 : "Failed: search 1";
        assert binarySearch(sorted, 7) == 3 : "Failed: search 7";
        assert binarySearch(sorted, 20) == 7 : "Failed: search 20";
        assert binarySearch(sorted, 2) == -1 : "Failed: search 2 (missing)";
        assert binarySearch(new int[]{}, 5) == -1 : "Failed: empty array";
        assert binarySearch(new int[]{42}, 42) == 0 : "Failed: single element found";
        assert binarySearch(new int[]{42}, 10) == -1 : "Failed: single element missing";
        System.out.println("[PASS] All Binary Search test cases passed!");
    }
}