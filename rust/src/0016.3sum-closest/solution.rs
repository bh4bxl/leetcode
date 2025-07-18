// Created by bh4bxl at 2025/07/15 15:14
// leetgo: 1.4.15
// https://leetcode.com/problems/3sum-closest/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums_m = nums.clone();
        nums_m.sort();

        let mut sub = i32::MAX;
        let mut sum = 0;

        for i in 0..nums.len() {
            let mut lo = i + 1;
            let mut hi = nums.len() - 1;

            while lo < hi {
                let sum_tmp = nums_m[lo] + nums_m[hi] + nums_m[i] - target;
                if sum_tmp.abs() < sub {
                    sum = nums_m[lo] + nums_m[hi] + nums_m[i];
                    sub = (sum - target).abs();
                }
                if nums_m[lo] + nums_m[hi] + nums_m[i] > target {
                    hi -= 1;
                } else {
                    lo += 1;
                }
            }
        }

        sum
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let target: i32 = deserialize(&read_line()?)?;
	let ans: i32 = Solution::three_sum_closest(nums, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
