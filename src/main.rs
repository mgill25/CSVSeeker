use std::env::args_os;
use std::io;
use std::io::{BufRead, Write};

fn main() {
    if args_os().len() < 2 {
        println!("Usage: prop_data <path_to_file>");
    } else {
        match args_os()
            .nth(1).unwrap()
            .into_string() {
            Ok(file_name) => {
                // let filename = String::from("./src/data/3.csv");
                let stdin = io::stdin();
                println!("Welcome to CSVSeeker!");
                loop {
                    print!("-# ");
                    let _ = io::stdout().flush();
                    let mut query = String::new();
                    stdin.lock().read_line(&mut query).unwrap();
                    if !query.is_empty() {
                        prop_data::query_data(&file_name, &query);
                    }
                }
            },
            Err(e) => {
                println!("Err = {:?}", e);
            }
        }
    }
}

