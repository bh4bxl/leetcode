// Created by bh4bxl at 2025/12/10 13:54
// leetgo: 1.4.15
// https://leetcode.com/problems/wiggle-sort-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let len = nums.len();
        let idx = if len % 2 == 0 { len / 2 } else { len / 2 + 1 };

        nums.sort();

        let mut arr = nums.clone();
        let sec = arr.split_off(idx);

        for i in 0..idx {
            nums[i * 2] = arr[idx - i - 1];
            if i * 2 + 1 < len {
                nums[i * 2 + 1] = sec[sec.len() - i - 1];
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    Solution::wiggle_sort(&mut nums);
    let ans: Vec<i32> = nums.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
