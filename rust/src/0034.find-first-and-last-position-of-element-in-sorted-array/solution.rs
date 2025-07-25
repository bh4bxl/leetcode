// Created by bh4bxl at 2025/07/24 13:18
// leetgo: 1.4.13
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = (-1, -1);
        if nums.is_empty() {
            return vec![ans.0, ans.1];
        }

        let (mut left, mut right) = (0, nums.len() as i32 - 1);

        while left <= right {
            let mid = (left + right) / 2;

            if target == nums[mid as usize] {
                let n = if mid > 0 { nums[mid as usize - 1] } else { i32::MIN };
                if target > n || mid == 0 {
                    ans.0 = mid as i32;
                    break;
                }
                right = mid - 1;
            } else if target < nums[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        (left, right) = (0, nums.len() as i32 - 1);

        while left <= right {
            let mid = (left + right) / 2;
            if target == nums[mid as usize] {
                let n = if mid < nums.len() as i32 - 1 {
                    nums[mid as usize + 1]
                } else {
                    i32::MAX
                };
                if target < n || mid == nums.len() as i32 - 1 {
                    ans.1 = mid as i32;
                    break;
                }
                left = mid + 1;
            } else if target < nums[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return vec![ans.0, ans.1];
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let target: i32 = deserialize(&read_line()?)?;
	let ans: Vec<i32> = Solution::search_range(nums, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
