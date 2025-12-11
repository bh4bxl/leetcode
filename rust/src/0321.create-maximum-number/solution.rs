// Created by bh4bxl at 2025/12/09 21:26
// leetgo: 1.4.15
// https://leetcode.com/problems/create-maximum-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut res = vec![0; k];
        let (n1, n2) = (nums1.len(), nums2.len());

        for i in (k.saturating_sub(n2))..=k.min(n1) {
            let (v1, v2) = (Self::max_subseq(&nums1, i), Self::max_subseq(&nums2, k - i));
            let merged = Self::merge(v1, v2);
            if merged > res {
                res = merged;
            }
        }

        res
    }

    fn max_subseq(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let mut stack = vec![];
        let mut drop = nums.len().saturating_sub(k);

        for &x in nums {
            while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < x {
                stack.pop();
                drop -= 1;
            }
            stack.push(x);
        }

        stack.truncate(k);
        stack
    }

    fn merge(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(a.len() + b.len());
        while !a.is_empty() || !b.is_empty() {
            if a > b {
                res.push(a.remove(0));
            } else {
                res.push(b.remove(0));
            }
        }
        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums1: Vec<i32> = deserialize(&read_line()?)?;
    let nums2: Vec<i32> = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::max_number(nums1, nums2, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
