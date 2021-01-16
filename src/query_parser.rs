use crate::Conditions;

pub struct Query {
    pub(crate) aggr: String,
    pub(crate) cond: Conditions,
}

fn parse_condition(cond: &str) -> Conditions {
    let column_name = cond.split_whitespace().nth(0).unwrap();
    let operator = cond.split_whitespace().nth(1).unwrap();
    let parameter = cond.split_whitespace().nth(2).unwrap();

    let p = parameter.parse::<i32>().unwrap();

    if operator.eq(">") {
        Conditions::IntGreaterThanComparison(column_name.parse().unwrap(), p)
    } else if operator.eq("<") {
        Conditions::IntLessThanComparison(column_name.parse().unwrap(), p)
    } else if operator.eq("=") {
        Conditions::IntEqualComparison(column_name.parse().unwrap(), p)
    } else {
        Conditions::Noop
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

