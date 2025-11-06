// Created by bh4bxl at 2025/11/04 12:15
// leetgo: 1.4.15
// https://leetcode.com/problems/majority-element-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for num in &nums {
            if let Some(v) = map.get(&num) {
                map.insert(num, v + 1);
            } else {
                map.insert(num, 1);
            }
        }

        let mut res = vec![];
        for (num, v) in map.iter() {
            if v * 3 > nums.len() {
                res.push(**num);
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::majority_element(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
