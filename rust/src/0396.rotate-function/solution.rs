// Created by bh4bxl at 2026/01/27 20:51
// leetgo: 1.4.16
// https://leetcode.com/problems/rotate-function/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();

        let mut f = 0;
        for (i, &v) in nums.iter().enumerate() {
            f += i as i32 * v;
        }

        let mut res = f;
        for k in 1..nums.len() {
            f = f + sum - n * nums[nums.len() - k];
            res = res.max(f);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_rotate_function(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
