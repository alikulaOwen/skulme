import java.util.*;

public class Solution {

    /**
     * Merge Sort: Stable, Divide-and-Conquer
     * Time Complexity: O(N log N) in all cases
     * Space Complexity: O(N)
     */
    public static void mergeSort(int[] arr) {
        if (arr == null || arr.length <= 1) return;
        int[] temp = new int[arr.length];
        sort(arr, 0, arr.length - 1, temp);
    }

    private static void sort(int[] arr, int left, int right, int[] temp) {
        if (left >= right) return;
        int mid = left + (right - left) / 2;
        sort(arr, left, mid, temp);
        sort(arr, mid + 1, right, temp);
        merge(arr, left, mid, right, temp);
    }

    private static void merge(int[] arr, int left, int mid, int right, int[] temp) {
        int i = left, j = mid + 1, k = left;
        while (i <= mid && j <= right) {
            if (arr[i] <= arr[j]) { // <= preserves stability
                temp[k++] = arr[i++];
            } else {
                temp[k++] = arr[j++];
            }
        }
        while (i <= mid) temp[k++] = arr[i++];
        while (j <= right) temp[k++] = arr[j++];
        for (int p = left; p <= right; p++) arr[p] = temp[p];
    }

    public static void main(String[] args) {
        System.out.println("Running Merge Sort Tests...");
        int[] arr = {12, 11, 13, 5, 6, 7};
        mergeSort(arr);
        assert Arrays.equals(arr, new int[]{5, 6, 7, 11, 12, 13}) : "Failed merge sort";
        
        int[] sorted = {1, 2, 3, 4};
        mergeSort(sorted);
        assert Arrays.equals(sorted, new int[]{1, 2, 3, 4});

        int[] dupes = {4, 2, 2, 1, 4};
        mergeSort(dupes);
        assert Arrays.equals(dupes, new int[]{1, 2, 2, 4, 4});
        System.out.println("[PASS] All Merge Sort test cases passed!");
    }
}