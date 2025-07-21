function moveZeroes(nums: number[]): void {
    // 链接：https://leetcode.cn/problems/move-zeroes/solutions/2831126/jskuai-man-zhi-zhen-fu-gai-yuan-shu-zu-h-wfga/

    let slow = 0;
    for (let fast = 0; fast < nums.length; fast++) {
        if (nums[fast] !== 0) {
            nums[slow] = nums[fast];
            slow++;
        }
    }
    nums.fill(0, slow);
}

// moveZeroes([0, 1, 0, 3, 12]);
moveZeroes([0, 0, 1]);
moveZeroes([0, 1, 0, 3, 12]);
