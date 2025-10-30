// Created by bh4bxl at 2025/10/28 13:21
// leetgo: 1.4.15
// https://leetcode.com/problems/contains-duplicate-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(last) = map.get(&num) {
                if i - last <= k as usize {
                    return true;
                } else {
                    map.insert(num.clone(), i);
                }
            } else {
                map.insert(num.clone(), i);
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::contains_nearby_duplicate(nums, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
