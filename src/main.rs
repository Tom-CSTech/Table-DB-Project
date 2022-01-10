//Final Project - Thomas Ivanov

#![allow(dead_code)]
#![allow(unused_variables)]

mod logic;
mod datastore;
mod persistence;

use std::error::Error;
use std::process;
use std::env;
use std::io;
use std::fs;

use crate::logic::*;
use crate::datastore::*;

fn main() -> Result<(), Box<dyn Error>> {

    //If new() function to create Config struct successful, return struct, else run code with error message
    let mut config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut column_labels: datastore::Header;
    let mut data: Vec<datastore::DataRow>;

    //If load_data() function to create Vec<DataRow> from file successful, return vector, else run code with error message
    match fs::File::open("datastore.csv") {
        Ok(f) =>    {
                        config.change_file(String::from("datastore.csv"));
                        let both = load(&config).unwrap_or_else(|err| {
                            eprintln!("Data\t error: {}", err);
                            process::exit(1);
                        });
                        column_labels = both.0;
                        data = both.1;
                    }
        Err(e) =>   {
                        let both = load(&config).unwrap_or_else(|err| {
                            eprintln!("Data\t error: {}", err);
                            process::exit(1);
                        });
                        column_labels = both.0;
                        data = both.1;
                    }
    }

    loop {
        //Line below clears console window
        std::process::Command::new("cmd").args(&["/C","cls"]).status().expect("Failed to execute process (CLS)");

        //Print header lines, including column labels
        println!("Covid Data CLI App - Thomas Ivanov");
        
        //Process user input to determine whether to show next page or quit
        println!("\nInput a key to select an option (Q to exit)
1) View all the current data
2) Save current data to file
3) View specific records
4) Edit a record
5) Delete a record
6) Clear and refresh all records");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        if input.len() != 1 {
            println!("String length is not 1! Press Enter to try again.");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            continue;
        }
        else {
            let input = input.chars().next().unwrap();
            match input {
                'q'|'Q' => break,
                '1' => {
                    //Run function to load data, and if an error is output (propagated from function), run code with error message
                    match display(&config, &column_labels, &data) {
                        Err(e) => {eprintln!("Application\t error: {}", e);process::exit(1);}
                        Ok(()) => {}
                    }
                },
                '2' => {
                    //Run function to save data, and if an error is output (propagated from function), run code with error message
                    if let Err(e) = save(&column_labels, &data) {
                        eprintln!("Application\t error: {}", e);
                        process::exit(1);
                    }
                },
                '3' => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");

                    //The data row indices we collected in this list are used to collect the matching data rows
                    let mut out: Vec<DataRow> = Vec::new();

                    //Run function to search data, and if an error is output (propagated from function), run code with error message
                    match search(input, &config, &data) {
                        Err(e) => {eprintln!("Application\t error: {}", e);process::exit(1);}
                        Ok(final_data) => {
                                                        for entry in final_data {
                                                            if entry < data.len() {
                                                                out.push(data[entry].clone());
                                                            }
                                                        }
                                                    
                                                        //The collected data rows are passed to our display function for the user to see
                                                        display(&config, &column_labels, &out)?;
                                                    }
                    }
                },
                '4' => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    //Run function to edit data, and if an error is output (propagated from function), run code with error message
                    if let Err(e) = edit(input, &config, &column_labels, &mut data) {
                        eprintln!("Application\t error: {}", e);
                        process::exit(1);
                    }
                },
                '5' => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    //Run function to delete data, and if an error is output (propagated from function), run code with error message
                    if let Err(e) = delete(input, &config, &column_labels, &mut data) {
                        eprintln!("Application\t error: {}", e);
                        process::exit(1);
                    }
                },
                '6' => {
                    //Run function to clear and refresh all data, and if an error is output (propagated from function), run code with error message
                    let both = refresh(&mut config).unwrap_or_else(|err| {
                        eprintln!("Problem parsing arguments: {}", err);
                        process::exit(1);
                    });
                    column_labels = both.0;
                    data = both.1;
                },
                _ => {println!("Please enter a valid selection (1-5, Q)");continue}
            }
        }
    }
    std::process::Command::new("cmd").args(&["/C","cls"]).status().expect("Failed to execute process (CLS)");
    Ok(())
}