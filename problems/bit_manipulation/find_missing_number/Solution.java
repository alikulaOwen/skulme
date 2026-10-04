import java.util.*;

public class Solution {

    /**
     * Missing Number in [0..n]
     * Uses XOR property: a ^ a = 0, a ^ 0 = a
     * Time Complexity: O(N)
     * Space Complexity: O(1)
     */
    public static int missingNumber(int[] nums) {
        int xor = nums.length;
        for (int i = 0; i < nums.length; i++) {
            xor ^= i ^ nums[i];
        }
        return xor;
    }

    public static void main(String[] args) {
        System.out.println("Running Missing Number Tests...");
        assert missingNumber(new int[]{3, 0, 1}) == 2 : "Failed [3, 0, 1]";
        assert missingNumber(new int[]{0, 1}) == 2 : "Failed [0, 1]";
        assert missingNumber(new int[]{9,6,4,2,3,5,7,0,1}) == 8 : "Failed missing 8";
        System.out.println("[PASS] All Missing Number test cases passed!");
    }
}