use crate::{Conditions, Column, ColumnType};
use crate::Conditions::{IntGreaterThanComparison, IntLessThanComparison, IntEqualComparison};
use crate::conds::{greater_than, equal_to, less_than};

// true -> this row should be kept
// false -> this row should be skipped
pub fn filter_row(cond: &Conditions, curr_cell: &str, cell_data: &Column) -> bool {
    match (&cell_data.dtype, cond) {
        (ColumnType::ColumnInt32, IntGreaterThanComparison(col_name, cond_val)) =>
            greater_than(&cell_data, &col_name, &curr_cell, cond_val),
        (ColumnType::ColumnInt32, IntLessThanComparison(col_name, cond_val)) =>
            less_than(&cell_data, &col_name, &curr_cell, cond_val),
        (ColumnType::ColumnInt32, IntEqualComparison(col_name, cond_val)) =>
            equal_to(&cell_data, &col_name, &curr_cell, cond_val),
        _ => false
    }
}