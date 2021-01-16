use crate::{Row, rowutils};
use crate::Cell::IntVal;

pub fn apply_aggr(rows: Vec<&Row>, aggr_query: &String) {
    match aggr_query.trim_end() {
        "" => {
            // no aggregation to be done, just print matched rows
            for row in rows {
                rowutils::print_row(&row);
            }
        }
        "count(*)" => {
            println!("> count = {}", rows.len());
        },
        // Yes, this is not very sophisticated.
        // Assuming the cells are also the aggregation columns in the case
        // of sum().
        // So we cannot do "sum(foo) where bar = 10" just yet.
        // We can only do "sum(foo) where foo = 10"
        // also, how can we pattern match in a nested way?
        // match "sum(SOMETHING)" { ... } match SOMETHING { ... }"
        "sum(*)" => {
            let mut total = 0;
            rows.iter().for_each(|item| {
                // assuming single item row
                if let IntVal(value) = item.cells.get(0).unwrap() {
                    // println!("adding value = {}", value);
                    total += value
                }
            });
            println!("> sum = {}", total);
        }
        _ => {}
    }
}