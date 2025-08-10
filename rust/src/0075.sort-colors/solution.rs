// Created by bh4bxl at 2025/08/08 13:21
// leetgo: 1.4.15
// https://leetcode.com/problems/sort-colors/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut red, mut white, mut blue) = (vec![], vec![], vec![]);

        for i in nums.iter() {
            match i {
                0 => red.push(0),
                1 => white.push(1),
                2 => blue.push(2),
                _ => {}
            }
        }

        nums.clear();
        nums.append(&mut red);
        nums.append(&mut white);
        nums.append(&mut blue);
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    Solution::sort_colors(&mut nums);
    let ans: Vec<i32> = nums.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
