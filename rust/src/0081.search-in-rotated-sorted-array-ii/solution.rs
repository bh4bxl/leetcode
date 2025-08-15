// Created by bh4bxl at 2025/08/13 21:43
// leetgo: 1.4.15
// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut start, mut end) = (0, nums.len() - 1);

        while start <= end {
            let mid = (start + end) / 2;
            if target == nums[mid] {
                return true;
            }

            if nums[start] < nums[mid] {
                if target >= nums[start] && target < nums[mid] {
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            } else if nums[start] == nums[mid] {
                start += 1;
            } else {
                if target > nums[mid] && target <= nums[end] {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
        }

        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::search(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
