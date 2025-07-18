// Created by bh4bxl at 2025/07/11 13:13
// leetgo: 1.4.14
// https://leetcode.com/problems/median-of-two-sorted-arrays/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        if l1 > l2 {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let k = (l1 + l2 + 1) / 2;

        let mut l = 0;
        let mut r = l1;

        while l < r {
            let m1 = l + (r - l) / 2;
            let m2 = k - m1;
            if nums1[m1] < nums2[m2 - 1] {
                l = m1 + 1;
            } else {
                r = m1;
            }
        }

        let m1 = l;
        let m2 = k - l;

        let c1 = std::cmp::max(
            if m1 <= 0 { i32::MIN } else { nums1[m1 - 1] },
            if m2 <= 0 { i32::MIN } else { nums2[m2 - 1] }
        );

        if (l1 + l2) % 2 == 1 {
            return c1 as f64;
        }

        let c2 = std::cmp::min(
            if m1 >= l1 { i32::MAX } else { nums1[m1] },
            if m2 >= l2 { i32::MAX } else { nums2[m2] }
        );

        (c1 + c2) as f64 * 0.5
    }
}

// @lc code=end

fn main() -> Result<()> {
	let nums1: Vec<i32> = deserialize(&read_line()?)?;
	let nums2: Vec<i32> = deserialize(&read_line()?)?;
	let ans: f64 = Solution::find_median_sorted_arrays(nums1, nums2).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
