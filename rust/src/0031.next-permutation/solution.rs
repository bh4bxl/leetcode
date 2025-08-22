// Created by bh4bxl at 2025/07/23 20:38
// leetgo: 1.4.15
// https://leetcode.com/problems/next-permutation/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as i32 - 2;

        while i >= 0 && nums[i as usize + 1] <= nums[i as usize] {
            i -= 1;
        }

        if i < 0 {
            Self::reverse(nums, 0);
            return;
        }

        let mut j = nums.len() as i32 - 1;

        while j >= 0 && nums[j as usize] <= nums[i as usize] {
            j -= 1;
        }

        if j < 0 {
            j = nums.len() as i32 - 1;
        }

        nums.swap(i as usize, j as usize);

        Self::reverse(nums, i as usize + 1);
    }

    fn reverse(nums: &mut Vec<i32>, start: usize) {
        let (mut i, mut j) = (start, nums.len() - 1);
        while i < j {
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    Solution::next_permutation(&mut nums);
    let ans: Vec<i32> = nums.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
