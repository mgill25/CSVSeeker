use crate::{Conditions, ColumnType};
use crate::ColumnType::{ColumnBool, ColumnInt32, ColumnString};

#[derive(Debug)]
pub struct Query {
    pub(crate) aggr: String,
    pub(crate) cond: Conditions,
}

fn parse_condition(cond: &str) -> Conditions {
    let column_name = cond.split_whitespace().nth(0).unwrap();
    let operator = cond.split_whitespace().nth(1).unwrap();
    let p= cond.split_whitespace().nth(2).unwrap();

    let ptype = infer_parameter(&p);

    match (operator, ptype) {
        (">", ColumnInt32) => {
            Conditions::IntGreaterThanComparison(column_name.parse().unwrap(), p.parse().unwrap())
        },
        ("<", ColumnInt32) => {
            Conditions::IntLessThanComparison(column_name.parse().unwrap(), p.parse().unwrap())
        },
        ("=", ColumnInt32) => {
            Conditions::IntEqualComparison(column_name.parse().unwrap(), p.parse().unwrap())
        },
        (">", ColumnString) => {
            Conditions::StringGreaterThanComparison(column_name.parse().unwrap(), p.parse().unwrap())
        },
        ("<", ColumnString) => {
            Conditions::StringLessThanComparison(column_name.parse().unwrap(), p.parse().unwrap())
        },
        ("=", ColumnString) => {
            Conditions::StringEqualComparison(column_name.parse().unwrap(), p.parse().unwrap())
        },
        ("=", ColumnBool) => {
            Conditions::BooleanComparison(column_name.parse().unwrap(), p.parse().unwrap())
        }
        (_, _) => Conditions::Noop
    }
}

// Infer a column type based on the single query parameter we supplied.
fn infer_parameter(parameter: &str) -> ColumnType {
    let lparam = parameter.to_lowercase();
    let is_digit = lparam.chars().all(|c| c.is_digit(10));
    if is_digit {
        ColumnInt32
    } else {
        let is_bool = lparam.eq("true") || lparam.eq("false");
        if is_bool { ColumnBool }
        else { ColumnString }
    }
}

// count(*) where dob = 1990
pub fn parse(query: &String) -> Query {
    if !query.contains("where") {
        // assume no aggregation, spit out all matching rows
        let condition = parse_condition(query);
        Query {
            aggr: "".parse().unwrap(),
            cond: condition
        }
    } else {
        let aggr = query.split("where").nth(0).unwrap();
        let cond = query.split("where").nth(1).unwrap();
        let condition = parse_condition(cond);
        Query {
            aggr: aggr.parse().unwrap(),
            cond: condition
        }
    }
}

