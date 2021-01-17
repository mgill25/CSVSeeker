use crate::{Column, Row};
use crate::Cell::IntVal;
use crate::rowutils::cell_to_bool;

pub fn greater_than(exp_col: &Column,
                    col_name: &String,
                    cell: &str,
                    condition_val: &i32) -> bool {
    if exp_col.name.eq(col_name) {
        let cell_result = cell.parse::<i32>();
        let matched_row = match cell_result {
            Ok(cell_parsed) => { cell_parsed > *condition_val },
            Err(e) => {
                println!("Failed to parse column `{}` err = {}", col_name, e);
                false
            }
        };
        matched_row
    } else {
        false
    }
}

pub fn less_than(exp_col: &Column,
                    col_name: &String,
                    cell: &str,
                    condition_val: &i32) -> bool {
    if exp_col.name.eq(col_name) {
        let cell_result = cell.parse::<i32>();
        let matched_row = match cell_result {
            Ok(cell_parsed) => { cell_parsed < *condition_val },
            Err(e) => {
                println!("Failed to parse column `{}` err = {}", col_name, e);
                false
            }
        };
        matched_row
    } else {
        false
    }
}

pub fn equal_to(exp_col: &Column,
                    col_name: &String,
                    cell: &str,
                    condition_val: &i32) -> bool {
    if exp_col.name.eq(col_name) {
        let cell_result = cell.parse::<i32>();
        let matched_row = match cell_result {
            Ok(cell_parsed) => { cell_parsed == *condition_val },
            Err(e) => {
                println!("Failed to parse column `{}` err = {}", col_name, e);
                false
            }
        };
        matched_row
    } else {
        false
    }
}

pub fn string_equal_to(exp_col: &Column, col_name: &String, cell: &str, condition_val: &str) -> bool {
    if exp_col.name.eq(col_name) { cell == condition_val } else { false }
}

pub fn string_gt(exp_col: &Column, col_name: &String, cell: &str, condition_val: &str) -> bool {
    if exp_col.name.eq(col_name) { cell > condition_val } else { false }
}
pub fn string_lt(exp_col: &Column, col_name: &String, cell: &str, condition_val: &str) -> bool {
    if exp_col.name.eq(col_name) { cell < condition_val } else { false }
}

pub fn boolean_equal_to(exp_col: &Column, col_name: &String, cell: &str, condition_val: &bool) -> bool {
    if exp_col.name.eq(col_name) { cell_to_bool(cell) == *condition_val } else { false }
}
