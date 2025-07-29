// Created by bh4bxl at 2025/07/27 16:20
// leetgo: 1.4.15
// https://leetcode.com/problems/first-missing-positive/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut nums_m = nums.clone();
        for i in 0..len {
            while nums_m[i] > 0 &&
                nums_m[i] <= len as i32 &&
                nums_m[i] != nums_m[nums_m[i] as usize - 1] {
                    let n = nums_m[i] as usize - 1;
                    nums_m.swap(i, n);
            }
        }

        for i in 0..len {
            if nums_m[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }

        return len as i32 + 1
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: i32 = Solution::first_missing_positive(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
