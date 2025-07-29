// Created by bh4bxl at 2025/07/28 17:17
// leetgo: 1.4.15
// https://leetcode.com/problems/permutations/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut list = vec![];
        let mut temp = vec![];

        Self::backtack(&mut list, &mut temp, &nums);

        list
    }

    fn backtack(list: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>) {
        if temp.len() == nums.len() {
            list.push(temp.clone());
        } else {
            for i in nums {
                if temp.contains(&i) {
                    continue;
                }
                temp.push(*i);
                Self::backtack(list, temp, nums);
                temp.pop();
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: Vec<Vec<i32>> = Solution::permute(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
