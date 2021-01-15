use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::Conditions::{IntGreaterThanComparison, IntLessThanComparison, IntEqualComparison};
use std::collections::HashMap;
use crate::ColumnType::{ColumnInt32};
use aggr::apply_aggr;

mod conds;
mod query_parser;
mod aggr;

/*
fn main() {
    let filename = String::from("./src/data/1.csv");
    let query_string: String = String::from("year > 1995");
    find_in_file(&filename, &query_string);
}
*/

// We need a way to represent column types in our data file.
// So from now on, our source file is no longer simple CSV.
// Lets start simple: <col1>:<type>,<col2>:<type>
// Note: No need to cop out on space just yet (we only save the types once in the header)

#[derive(Debug)]
enum ColumnType {
    ColumnString,
    ColumnInt32,
    Null
}

#[derive(Debug)]
pub struct Column {
    name: String,
    dtype: ColumnType
}

enum Conditions {
    Noop,
    IntGreaterThanComparison(String, i32),
    IntLessThanComparison(String, i32),
    IntEqualComparison(String, i32),
}

// Representation of a Row, comprising of multiple "cells" of data items.
// Our check_rows() function should return a Vector of Rows, upon which we can apply
// aggregations.
#[derive(Debug)]
pub enum Cell {
    IntVal(i32),
    StringVal(String),
    BooleanVal(bool),
}

#[derive(Debug)]
pub struct Row {
    cells: Vec<Cell>
}


// Evaluate $conditions for every row in the table
fn check_rows(buf: BufReader<File>,
              conditions: &Conditions,
              cols_with_type: HashMap<usize, Column>) -> Vec<Row> {
    let mut rows = vec![];
    buf.lines().for_each(|line| {
        let actual_line = line.unwrap();
        for (idx, cell) in actual_line.split(",").enumerate() {
            let exp_col = cols_with_type.get(&idx).unwrap();
            match conditions {
                IntGreaterThanComparison(col_name, condition_val) => {
                    match exp_col.dtype {
                        ColumnInt32 => {
                            rows.push(conds::greater_than(&actual_line,
                                                &exp_col,
                                                &col_name,
                                                &cell,
                                                condition_val));
                        },
                        _ => {}
                    }
                },
                IntLessThanComparison(col_name, condition_val) => {
                    match exp_col.dtype {
                        ColumnInt32 => {
                            rows.push(conds::less_than(&actual_line,
                                             &exp_col,
                                             &col_name,
                                             &cell,
                                             condition_val));
                        },
                        _ => {}
                    }
                },
                IntEqualComparison(col_name, condition_val) => {
                    match exp_col.dtype {
                        ColumnInt32 => {
                            rows.push(conds::equal_to(&actual_line,
                                            &exp_col,
                                            &col_name,
                                            &cell,
                                            condition_val));
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    });
    rows
}

pub fn query_data(filename: &String, query: &String) {
    let file_match = File::open(filename);
    let parsed_query = query_parser::parse(query);
    let condition = parsed_query.cond;
    match file_match {
        Ok(file) => {
            // Reads the entire file into the buffer
            let mut buf = BufReader::new(file);
            let mut header = String::from("");
            buf.read_line(&mut header).unwrap_or_else(|err| {
                println!("Failed to read header = {}", err);
                0
            });
            let cols_with_type = parse_header(&header);
            let rows = check_rows(buf, &condition, cols_with_type);
            let mut result = vec![];
            rows.iter().for_each(|row| {
                if !row.cells.is_empty() {
                    result.push(row);
                }
            });
            apply_aggr(result, &parsed_query.aggr)
        }
        Err(e) => println!("Error = {}", e)
    }
}

fn parse_header(header: &String) -> HashMap<usize, Column> {
    let mut col_data = HashMap::new();
    header.split(",").enumerate().for_each(|(idx, col_pair)| {
        let col_name = col_pair.split(":").nth(0).unwrap();
        let raw_col_type = col_pair.split(":").nth(1).unwrap();
        let col_type = match raw_col_type.trim_end() {
            "string" => ColumnType::ColumnString,
            "int" => ColumnType::ColumnInt32,
            _ => ColumnType::Null,
        };
        let col = Column { name: col_name.parse().unwrap(), dtype: col_type };
        col_data.insert(idx, col);
    });
    col_data
}

pub fn find_in_file(filename: &String, query: &String) {
    let file_match = File::open(filename);
    match file_match {
        Ok(file) => {
            // add buffering to the reader
            let reader = BufReader::new(file);
            let result = reader.lines()
                .map(|line| {
                    line.unwrap()
                        .to_lowercase()
                        .split(",")
                        .nth(0).unwrap()
                        .to_string()
                })
                .find(|column| column == query);
            match result {
                Some(_) => (),
                None => println!("NOT FOUND")
            }
        }
        Err(e) => {
            println!("Error during file opening {}", e)
        }
    }
}

