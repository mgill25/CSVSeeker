use std::env::args_os;
use std::io;
use std::io::{BufRead, Write};

fn main() {
    if args_os().len() < 2 {
        println!("Usage: csvseeker <path_to_file>");
    } else {
        match args_os()
            .nth(1).unwrap()
            .into_string() {
            Ok(file_name) => {
                let stdin = io::stdin();
                println!("Welcome to CSVSeeker!");
                loop {
                    print!("-# ");
                    let _ = io::stdout().flush();
                    let mut query = String::new();
                    let bytes = stdin.lock().read_line(&mut query).unwrap();
                    if bytes == 0 {
                        println!("Finito~");
                        std::process::exit(0);
                    }
                    if !query.trim_end().is_empty() {
                        println!("Query = {}", query);
                        csvseeker::query_data(&file_name, &query);
                    }
                }
            },
            Err(e) => {
                println!("Err = {:?}", e);
            }
        }
    }
}

