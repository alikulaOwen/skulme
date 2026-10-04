def binary_search(nums: list[int], target: int) -> int:
    left, right = 0, len(nums) - 1
    while left <= right:
        mid = (left + right) // 2
        if nums[mid] == target:
            return mid
        elif nums[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1

if __name__ == "__main__":
    nums = [1, 3, 5, 7, 9, 11, 15, 20]
    assert binary_search(nums, 1) == 0
    assert binary_search(nums, 7) == 3
    assert binary_search(nums, 20) == 7
    assert binary_search(nums, 2) == -1
    print("[PASS] Python Binary Search passed!")
