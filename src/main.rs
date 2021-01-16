use std::env::args_os;

fn main() {
    if args_os().len() < 2 {
        println!("Usage: prop_data <path_to_file>");
    } else {
        match args_os()
            .nth(1).unwrap()
            .into_string() {
            Ok(file_name) => {
                // let filename = String::from("./src/data/3.csv");
                let query = String::from("age > 30");
                prop_data::query_data(&file_name, &query);
            },
            Err(e) => {
                println!("Err = {:?}", e);
            }
        }
    }
}

