// Created by bh4bxl at 2025/07/25 17:22
// leetgo: 1.4.15
// https://leetcode.com/problems/combination-sum-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut temp: Vec<i32> = vec![];
        let mut list = candidates.clone();

        list.sort();

        Self::backtrack(&mut res, &mut temp, list, target, 0);

        res
    }

    fn backtrack(
        list: &mut Vec<Vec<i32>>,
        temp: &mut Vec<i32>,
        nums: Vec<i32>,
        target: i32,
        start: usize
    ) {
        if target < 0 {
            return;
        }

        if target == 0 {
            list.push(temp.clone());
        } else {
            let mut i = start;
            for num in &nums[start..] {
                if i > start && *num == nums[i - 1] {
                    i += 1;
                    continue;
                }
                temp.push(*num);
                Self::backtrack(list, temp, nums.clone(), target - *num, i + 1);
                i += 1;
                temp.pop();
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
	let candidates: Vec<i32> = deserialize(&read_line()?)?;
	let target: i32 = deserialize(&read_line()?)?;
	let ans: Vec<Vec<i32>> = Solution::combination_sum2(candidates, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
