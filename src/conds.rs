use crate::{Column, Row};
use crate::Cell::IntVal;

pub fn greater_than(_actual_line: &String,
                    exp_col: &Column,
                    col_name: &String,
                    cell: &str,
                    condition_val: &i32) -> Row {
    if exp_col.name.eq(col_name) {
        let cell_result = cell.parse::<i32>();
        let matched_row = match cell_result {
            Ok(cell_parsed) => {
                if cell_parsed == *condition_val {
                    // println!("[match] row = {}", actual_line);
                    Row { cells: vec![IntVal(cell_parsed)] }
                } else {
                    Row {cells: vec![] }
                }
            },
            Err(e) => {
                println!("Failed to parse column `{}` err = {}", col_name, e);
                Row { cells: vec![] }
            }
        };
        matched_row
    } else {
        Row { cells: vec![] }
    }
}

pub fn less_than(_actual_line: &String,
                 exp_col: &Column,
                 col_name: &String,
                 cell: &str,
                 condition_val: &i32) -> Row {
    if exp_col.name.eq(col_name) {
        let cell_result = cell.parse::<i32>();
        let matched_row = match cell_result {
            Ok(cell_parsed) => {
                if cell_parsed == *condition_val {
                    // println!("[match] row = {}", actual_line);
                    Row { cells: vec![IntVal(cell_parsed)] }
                } else {
                    Row { cells: vec![] }
                }
            },
            Err(e) => {
                println!("Failed to parse column `{}` err = {}", col_name, e);
                Row{ cells: vec![] }
            }
        };
        matched_row
    } else {
        Row{ cells: vec![] }
    }
}

pub fn equal_to(_actual_line: &String,
                exp_col: &Column,
                col_name: &String,
                cell: &str,
                condition_val: &i32) -> Row {
    if exp_col.name.eq(col_name) {
        let cell_result = cell.parse::<i32>();
        let matched_row = match cell_result {
            Ok(cell_parsed) => {
                if cell_parsed == *condition_val {
                    // println!("[match] row = {}", actual_line);
                    Row { cells: vec![IntVal(cell_parsed)] }
                } else {
                    Row { cells: vec![] }
                }
            },
            Err(e) => {
                println!("Failed to parse column `{}` err = {}", col_name, e);
                Row { cells: vec![] }
            }
        };
        matched_row
    } else {
        Row { cells: vec![] }
    }
}
