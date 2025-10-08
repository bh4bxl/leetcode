// Created by bh4bxl at 2025/10/08 16:16
// leetgo: 1.4.15
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);
        let mut sum = numbers[left] + numbers[right];
        while sum != target {
            if sum < target {
                left = left + 1;
            } else {
                right = right - 1;
            }
            sum = numbers[left] + numbers[right];
        }

        vec![left as i32 + 1, right as i32 + 1]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let numbers: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::two_sum(numbers, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
