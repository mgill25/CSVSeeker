use crate::{Row, Cell, Column, ColumnType};
use std::collections::HashMap;
use crate::Cell::{StringVal, IntVal, NullVal, BooleanVal};

pub fn to_cells(row_string: &str, cols_with_type: &HashMap<usize, Column>) -> Row {
    let mut cells = vec![];
    row_string.split(",").enumerate().for_each(|(idx, cell_string)| {
        let exp_col = cols_with_type.get(&idx).unwrap();
        let cell = match exp_col.dtype {
            ColumnType::ColumnString => {StringVal(String::from(cell_string))},
            ColumnType::ColumnInt32 => {IntVal(cell_string.parse::<i32>().unwrap())},
            ColumnType::Null => {NullVal}
        };
        cells.push(cell);
    });
    Row { cells }
}

pub fn print_row(row: &Row) {
    for cell in &row.cells {
        match cell {
            StringVal(s) => print!("{:<10}", s),
            IntVal(s) => print!("{:<10}", s),
            BooleanVal(s) => print!("{:<10}", s),
            NullVal => print!("NULL"),
        }
        print!("\t");
    }
    println!();
}