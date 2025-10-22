// Created by bh4bxl at 2025/10/21 15:07
// leetgo: 1.4.15
// https://leetcode.com/problems/minimum-size-subarray-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let (mut left, mut right) = (-1, -1);
        let mut res = len as i32;
        let mut sum: i32 = 0;
        let mut flag = false;

        while left < len as i32 {
            if sum < target && right < len as i32 {
                right += 1;
                if right >= len as i32 {
                    break;
                }
                sum += nums[right as usize];
            }
            if sum >= target {
                flag = true;
                res = res.min((right - left) as i32);
                if left < right {
                    left += 1;
                    sum -= nums[left as usize];
                } else {
                    return 1;
                }
            }
        }

        if flag { res } else { 0 }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let target: i32 = deserialize(&read_line()?)?;
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::min_sub_array_len(target, nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
