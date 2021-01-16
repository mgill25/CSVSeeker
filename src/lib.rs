use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::Conditions::{IntGreaterThanComparison, IntLessThanComparison, IntEqualComparison};
use std::collections::HashMap;
use crate::ColumnType::{ColumnInt32};
use aggr::apply_aggr;
use crate::Cell::IntVal;

mod conds;
mod query_parser;
mod aggr;
mod filter;

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

pub enum Conditions {
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
/**
for every row
  cells = split by comma
  for idx, cell in cells
    get_current_cell_type()
    match on input conditions {
        if greaterThan/lessThan/equalTo {
            match on current_cell_type {
                if ColumnInt32 =>
                    apply greater_than/less_than/equal_to function
                    push result to rows vector
            }
        }
    }
  return rows
----

// Question: Do we *always* do Full Table Scans whenever we touch rows on disk?
// In other words: Is an Index the only strategy we have in PostgreSQL/MySQL to do
// "data skipping" ? Surely not.
// Databricks-like Data Skipping is possible in MySQL via "Clustered Indexes".
// (but those are indexes!)

for every row
    cells = split by comma
    for idx,cell in cells
        cell_type = get_current_cell_type() // O(1) but can be cached outside the loop
        here, we know what sort of condition is to be applied at what "cell".
        apply_condition(cell, cell_type, condition)

*/
fn check_rows(buf: BufReader<File>,
              conditions: &Conditions,
              cols_with_type: HashMap<usize, Column>) -> Vec<Row> {
    let mut rows = vec![];
    buf.lines().for_each(|line| {
        let actual_line = line.unwrap();
        for (idx, cell) in actual_line.split(",").enumerate() {
            let exp_col = cols_with_type.get(&idx).unwrap();
            if filter::filter_row(&conditions,
                                  cell,
                                  exp_col) {
                let cell_parsed = cell.parse::<i32>().unwrap();
                rows.push(Row { cells: vec![IntVal(cell_parsed)] });
            } else {
                rows.push(Row { cells: vec![] });
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

