// Created by bh4bxl at 2025/10/07 13:50
// leetgo: 1.4.15
// https://leetcode.com/problems/compare-version-numbers/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let ver1: Vec<&str> = version1.split('.').collect();
        let ver2: Vec<&str> = version2.split('.').collect();

        let len1 = ver1.len();
        let len2 = ver2.len();
        let max_len = len1.max(len2);

        for i in 0..max_len {
            let (mut v1, mut v2) = (0, 0);
            if i < len1 {
                v1 = ver1[i].parse::<i32>().unwrap();
            }
            if i < len2 {
                v2 = ver2[i].parse::<i32>().unwrap();
            }
            if v1 > v2 {
                return 1;
            }
            if v1 < v2 {
                return -1;
            }
        }

        0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let version1: String = deserialize(&read_line()?)?;
    let version2: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::compare_version(version1, version2).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
