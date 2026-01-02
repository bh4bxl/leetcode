// Created by bh4bxl at 2025/12/29 19:37
// leetgo: 1.4.16
// https://leetcode.com/problems/data-stream-as-disjoint-intervals/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::collections::BTreeMap;

struct SummaryRanges {
    map: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, value: i32) {
        // Find right interval (first start > value)
        if let Some((&start, &end)) = self.map.range(..=value).next_back() {
            if value <= end {
                // Already covered
                return;
            }
            if end + 1 == value {
                // Extend left interval
                self.map.insert(start, value);
                // Check if can merge with right interval
                if let Some((&r_start, &r_end)) = self.map.range(value + 1..).next() {
                    if r_start == value + 1 {
                        self.map.insert(start, r_end);
                        self.map.remove(&r_start);
                    }
                }
                return;
            }
        }

        // Check if can merge with right interval
        if let Some((&r_start, &r_end)) = self.map.range(value + 1..).next() {
            if r_start == value + 1 {
                self.map.remove(&r_start);
                self.map.insert(value, r_end);
                return;
            }
        }

        // New isolated interval
        self.map.insert(value, value);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.map.iter().map(|(&s, &e)| vec![s, e]).collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = SummaryRanges::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "addNum" => {
                let method_params = split_array(&params[i])?;
                let value: i32 = deserialize(&method_params[0])?;
                obj.add_num(value);
                output.push("null".to_string());
            }
            "getIntervals" => {
                let ans: Vec<Vec<i32>> = obj.get_intervals().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
