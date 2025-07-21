function longestConsecutive(nums: number[]): number {
    // 去重
    nums = Array.from(new Set(nums));

    // 1. 排序
    nums = nums.toSorted((a, b) => a - b);

    // 2. 截断
    let newNums: number[][] = [];

    let index = 0;

    while (nums.length !== 0) {
        if (Math.abs(nums[index] - nums[index + 1]) !== 1) {
            newNums.push(nums.splice(0, index + 1));
            index = 0;
            continue;
        }
        index++;
    }

    if (nums.length != 0) {
        newNums.push(nums);
    }

    // 3. 输出最长数组长度
    return newNums.reduce((acc, v) => {
        if (v.length > acc) {
            acc = v.length;
        }

        return acc;
    }, 0);
}

// console.log(longestConsecutive([100, 4, 200, 1, 3, 2]));
// console.log(longestConsecutive([1, 2, 3, 4]));
// console.log(longestConsecutive([1, 0, 1, 2]));
console.log(longestConsecutive([-8, -4, 9, 9, 4, 6, 1, -4, -1, 6, 8]));
