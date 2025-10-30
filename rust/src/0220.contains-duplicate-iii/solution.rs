// Created by bh4bxl at 2025/10/28 18:11
// leetgo: 1.4.15
// https://leetcode.com/problems/contains-duplicate-iii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        use std::collections::BTreeSet;
        let mut set = BTreeSet::new();

        for i in 0..nums.len() {
            let num = nums[i] as i64;

            if let Some(candidate) = set
                .range(num - value_diff as i64..=num + value_diff as i64)
                .next()
            {
                if (candidate - num).abs() <= value_diff as i64 {
                    return true;
                }
            }

            set.insert(num);

            if i >= index_diff as usize {
                set.remove(&(nums[i - index_diff as usize] as i64));
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let index_diff: i32 = deserialize(&read_line()?)?;
    let value_diff: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::contains_nearby_almost_duplicate(nums, index_diff, value_diff).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
