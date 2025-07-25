// Created by bh4bxl at 2025/07/23 22:52
// leetgo: 1.4.15
// https://leetcode.com/problems/search-in-rotated-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        -1
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let target: i32 = deserialize(&read_line()?)?;
	let ans: i32 = Solution::search(nums, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
