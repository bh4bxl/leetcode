// Created by bh4bxl at 2025/09/26 13:09
// leetgo: 1.4.15
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        fn search(nums: &Vec<i32>, low: usize, high: usize) -> i32 {
            if low >= high {
                return nums[low];
            }
            let mid = low + (high - low) / 2;
            if nums[mid] == nums[high] {
                search(nums, low, high - 1)
            } else if nums[mid] >= nums[low] {
                nums[low].min(search(nums, mid + 1, high))
            } else {
                nums[mid].min(search(nums, low + 1, mid - 1))
            }
        }

        search(&nums, 0, nums.len() - 1)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::find_min(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
