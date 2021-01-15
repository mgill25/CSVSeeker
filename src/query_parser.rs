use crate::Conditions;

pub struct Query {
    pub(crate) aggr: String,
    pub(crate) cond: Conditions,
}

// count(*) where dob = 1990
pub fn parse(query: &String) -> Query {
    let aggr = query.split("where").nth(0).unwrap();
    let cond = query.split("where").nth(1).unwrap();

    let column_name = cond.split_whitespace().nth(0).unwrap();
    let operator = cond.split_whitespace().nth(1).unwrap();
    let parameter = cond.split_whitespace().nth(2).unwrap();

    let p = parameter.parse::<i32>().unwrap();

    let condition = if operator.eq(">") {
        Conditions::IntGreaterThanComparison(column_name.parse().unwrap(), p)
    } else if operator.eq("<") {
        Conditions::IntLessThanComparison(column_name.parse().unwrap(), p)
    } else if operator.eq("=") {
        Conditions::IntEqualComparison(column_name.parse().unwrap(), p)
    } else {
        Conditions::Noop
    };

    Query {
        aggr: aggr.parse().unwrap(),
        cond: condition
    }
}

