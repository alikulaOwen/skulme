def missing_number(nums: list[int]) -> int:
    xor = len(nums)
    for i, x in enumerate(nums):
        xor ^= i ^ x
    return xor

if __name__ == "__main__":
    assert missing_number([3, 0, 1]) == 2
    assert missing_number([0, 1]) == 2
    print("[PASS] Python Missing Number passed!")
