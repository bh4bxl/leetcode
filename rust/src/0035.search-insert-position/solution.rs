// Created by bh4bxl at 2025/07/24 16:07
// leetgo: 1.4.13
// https://leetcode.com/problems/search-insert-position/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let mid = (left + right) / 2;
            if target == nums[mid] {
                return mid as i32;
            } else if target > nums[mid] {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return 0;
                }
                right = mid - 1;
            }
        }

        if left - 1 == right {
            if nums[right] > target {
                return left as i32 - 1;
            }
            return left as i32;
        }

        return 0;
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums: Vec<i32> = deserialize(&read_line()?)?;
	let target: i32 = deserialize(&read_line()?)?;
	let ans: i32 = Solution::search_insert(nums, target).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
