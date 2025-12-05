// Created by bh4bxl at 2025/12/02 13:05
// leetgo: 1.4.15
// https://leetcode.com/problems/range-sum-query-mutable/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

struct NumArray {
    nums: Vec<i32>,
    bit: Vec<i32>,
    n: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let bit = vec![0; n + 1];
        let mut obj = NumArray {
            nums: nums.clone(),
            bit,
            n,
        };

        for (i, &v) in nums.iter().enumerate() {
            obj.add(i + 1, v);
        }

        obj
    }

    fn add(&mut self, mut i: usize, delta: i32) {
        while i <= self.n {
            self.bit[i] += delta;
            i += i & (!i + 1);
        }
    }

    fn prefix_sum(&self, mut i: usize) -> i32 {
        let mut sum = 0;
        while i > 0 {
            sum += self.bit[i];
            i &= i - 1;
        }
        sum
    }

    fn update(&mut self, index: i32, val: i32) {
        let idx = index as usize;
        let delta = val - self.nums[idx];
        self.nums[idx] = val;
        self.add(idx + 1, delta);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (l, r) = (left as usize, right as usize);
        self.prefix_sum(r + 1) - self.prefix_sum(l)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let nums: Vec<i32> = deserialize(&constructor_params[0])?;
    #[allow(unused_mut)]
    let mut obj = NumArray::new(nums);

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "update" => {
                let method_params = split_array(&params[i])?;
                let index: i32 = deserialize(&method_params[0])?;
                let val: i32 = deserialize(&method_params[1])?;
                obj.update(index, val);
                output.push("null".to_string());
            }
            "sumRange" => {
                let method_params = split_array(&params[i])?;
                let left: i32 = deserialize(&method_params[0])?;
                let right: i32 = deserialize(&method_params[1])?;
                let ans: i32 = obj.sum_range(left, right).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
