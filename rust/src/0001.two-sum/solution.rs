// Created by bh4bxl at 2025/06/30 11:06
// leetgo: 1.4.14
// https://leetcode.com/problems/two-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut index_i: usize = 0;
 
        'top: for i in &nums {
            let mut index_j: usize;
            index_j = index_i + 1;
            for j in &nums[index_i + 1..] {
                if i + j == target {
                    res.push(index_i as i32);
                    res.push(index_j as i32);
                    break 'top;
                }
                index_j += 1;
            }
            index_i += 1;
        }
        res
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let target: i32 = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::two_sum(nums, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
