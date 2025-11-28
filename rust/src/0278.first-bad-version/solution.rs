// Created by bh4bxl at 2025/11/24 21:10
// leetgo: 1.4.15
// https://leetcode.com/problems/first-bad-version/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut left, mut right) = (1, n);

        while left < right {
            let mid = left + (right - left) / 2;

            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let bad: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::first_bad_version(n, bad).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
