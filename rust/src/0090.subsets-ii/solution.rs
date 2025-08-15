// Created by bh4bxl at 2025/08/14 20:52
// leetgo: 1.4.15
// https://leetcode.com/problems/subsets-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut temp = vec![];
        let mut nums_m = nums.clone();

        nums_m.sort();

        Self::get_subsets(&nums_m, 0, &mut temp, &mut res);

        res
    }

    fn get_subsets(nums: &Vec<i32>, start: usize, temp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(temp.clone());

        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            temp.push(nums[i]);
            Self::get_subsets(nums, i + 1, temp, res);
            temp.pop();
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::subsets_with_dup(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
