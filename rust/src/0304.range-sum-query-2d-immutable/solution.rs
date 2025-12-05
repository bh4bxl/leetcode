// Created by bh4bxl at 2025/12/02 12:22
// leetgo: 1.4.15
// https://leetcode.com/problems/range-sum-query-2d-immutable/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (width, height) = (matrix[0].len(), matrix.len());
        let mut sums = vec![vec![0; width + 1]; height];

        for i in 0..height {
            for j in 0..width {
                sums[i][j + 1] = sums[i][j] + matrix[i][j];
            }
        }

        Self { sums: sums.clone() }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        let mut row1 = row1 as usize;

        while row1 <= row2 as usize {
            sum += self.sums[row1][col2 as usize + 1] - self.sums[row1][col1 as usize];
            row1 += 1;
        }

        sum
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let matrix: Vec<Vec<i32>> = deserialize(&constructor_params[0])?;
    let matrix_row_size: i32 = deserialize(&constructor_params[1])?;
    let matrix_col_size: i32 = deserialize(&constructor_params[2])?;
    #[allow(unused_mut)]
    let mut obj = NumMatrix::new(matrix, matrix_row_size, matrix_col_size);

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "sumRegion" => {
                let method_params = split_array(&params[i])?;
                let row1: i32 = deserialize(&method_params[0])?;
                let col1: i32 = deserialize(&method_params[1])?;
                let row2: i32 = deserialize(&method_params[2])?;
                let col2: i32 = deserialize(&method_params[3])?;
                let ans: i32 = obj.sum_region(row1, col1, row2, col2).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
