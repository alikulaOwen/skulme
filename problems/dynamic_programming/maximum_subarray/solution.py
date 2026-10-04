def max_sub_array(nums: list[int]) -> int:
    cur_max = glob_max = nums[0]
    for x in nums[1:]:
        cur_max = max(x, cur_max + x)
        glob_max = max(glob_max, cur_max)
    return glob_max

if __name__ == "__main__":
    assert max_sub_array([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6
    assert max_sub_array([-3, -2, -5]) == -2
    print("[PASS] Python Maximum Subarray passed!")
