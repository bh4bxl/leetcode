// Created by bh4bxl at 2025/09/25 18:18
// leetgo: 1.4.15
// https://leetcode.com/problems/maximum-product-subarray/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max, mut min) = (nums[0], nums[0]);
        let mut result = nums[0];

        for num in &nums[1..] {
            let tmp = max;
            max = *num.max(&(max * *num).max(min * *num));
            min = *num.min(&(tmp * *num).min(min * *num));
            result = result.max(max);
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_product(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
