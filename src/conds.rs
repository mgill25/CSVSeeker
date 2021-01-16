use crate::{Column, Row};
use crate::Cell::IntVal;

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