// Created by bh4bxl at 2025/08/10 12:27
// leetgo: 1.4.15
// https://leetcode.com/problems/subsets/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut temp = vec![];

        Self::get_subsets(&nums, 0, &mut temp, &mut res);

        res
    }

    fn get_subsets(nums: &Vec<i32>, start: usize, temp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(temp.clone());

        for i in start..nums.len() {
            temp.push(nums[i]);
            Self::get_subsets(nums, i + 1, temp, res);
            temp.pop();
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::subsets(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
