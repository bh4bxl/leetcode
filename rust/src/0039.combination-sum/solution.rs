// Created by bh4bxl at 2025/07/25 16:13
// leetgo: 1.4.15
// https://leetcode.com/problems/combination-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut list: Vec<Vec<i32>> = vec![];
        let mut temp: Vec<i32> = vec![];

        Self::backtrack(&mut list, &mut temp, candidates, target, 0);

        list
    }

    fn backtrack(
        list: &mut Vec<Vec<i32>>,
        temp: &mut Vec<i32>,
        nums: Vec<i32>,
        remain: i32,
        start: usize
    ) {
        if remain < 0 {
            return;
        }

        if remain == 0 {
            list.push(temp.clone());
        } else {
            let mut i = start;
            for num in &nums[start..] {
                temp.push(*num);
                Self::backtrack(list, temp, nums.clone(), remain - *num, i);
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
	let ans: Vec<Vec<i32>> = Solution::combination_sum(candidates, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
