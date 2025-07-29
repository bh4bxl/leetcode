// Created by bh4bxl at 2025/07/28 17:32
// leetgo: 1.4.15
// https://leetcode.com/problems/permutations-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut list = vec![];
        let mut temp = vec![];
        let mut nums_m = nums.clone();

        nums_m.sort();

        Self::backtack(&mut list, &mut temp, &mut nums_m);

        list
    }

    fn backtack(list: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &mut Vec<i32>) {
        let hidden = -100;

        if temp.len() == nums.len() {
            list.push(temp.clone());
        } else {
            for i in 0..nums.len() {
                if nums[i] == hidden {
                    continue;
                }
                if i > 0 && nums[i - 1] != hidden && nums[i - 1] == nums[i] {
                    continue;
                }
                temp.push(nums[i]);
                let bak = nums[i];
                nums[i] = hidden;
                Self::backtack(list, temp, nums);
                nums[i] = bak;
                temp.pop();
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let ans: Vec<Vec<i32>> = Solution::permute_unique(nums).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
