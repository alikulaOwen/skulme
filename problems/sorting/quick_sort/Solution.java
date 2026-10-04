import java.util.*;

public class Solution {

    /**
     * Quick Sort: In-place, Divide-and-Conquer
     * Time Complexity: O(N log N) average, O(N^2) worst
     * Space Complexity: O(log N) stack
     */
    public static void quickSort(int[] arr) {
        if (arr == null || arr.length <= 1) return;
        quickSort(arr, 0, arr.length - 1);
    }

    private static void quickSort(int[] arr, int low, int high) {
        if (low < high) {
            int p = partition(arr, low, high);
            quickSort(arr, low, p - 1);
            quickSort(arr, p + 1, high);
        }
    }

    private static int partition(int[] arr, int low, int high) {
        int pivot = arr[high];
        int i = low - 1;
        for (int j = low; j < high; j++) {
            if (arr[j] <= pivot) {
                i++;
                swap(arr, i, j);
            }
        }
        swap(arr, i + 1, high);
        return i + 1;
    }

    private static void swap(int[] arr, int i, int j) {
        int t = arr[i];
        arr[i] = arr[j];
        arr[j] = t;
    }

    public static void main(String[] args) {
        System.out.println("Running Quick Sort Tests...");
        int[] arr = {10, 7, 8, 9, 1, 5};
        quickSort(arr);
        assert Arrays.equals(arr, new int[]{1, 5, 7, 8, 9, 10}) : "Failed quick sort";
        System.out.println("[PASS] All Quick Sort test cases passed!");
    }
}