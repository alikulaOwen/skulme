function binarySearch(nums: number[], target: number): number {
    let left = 0, right = nums.length - 1;
    while (left <= right) {
        const mid = left + Math.floor((right - left) / 2);
        if (nums[mid] === target) return mid;
        if (nums[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;
}

const nums = [1, 3, 5, 7, 9, 11, 15, 20];
if (binarySearch(nums, 1) !== 0) throw new Error("Test 1 failed");
if (binarySearch(nums, 7) !== 3) throw new Error("Test 2 failed");
if (binarySearch(nums, 2) !== -1) throw new Error("Test 3 failed");
console.log("[PASS] TypeScript Binary Search passed!");
