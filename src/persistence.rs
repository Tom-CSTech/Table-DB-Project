//Final Project - Thomas Ivanov

use std::fs;
use std::error::Error;
use std::convert::TryInto;
use std::io::prelude::*;

use crate::datastore::*;
use crate::logic::Config;

/// Load the data from the csv file into a vector.
/// 
/// A Config struct is used to determine which file and which language setting to use for data IO. A Result
/// type is returned to pass possible errors up the stack to be safely handled by the calling function.
/// 
/// # Examples
/// 
/// ```
/// let data = load_data(&config).unwrap();
/// ```
pub fn load_data(config: &Config) -> Result<(Header, Vec<DataRow>), Box<dyn Error>> {

    //Match-extract the data from Ok(data) (which was output by the read fn), or else propagate Err(e) to main()
    let contents = fs::read_to_string(config.filename.clone())?;
    let column_labels: Header;
    //Call default filter on contents String for first row/line (column labels), then filter by language
    if config.filename == String::from("covid19-download.csv") {
        column_labels = match contents.lines().next() {
            Some(v) => Header::new(default_filter(v).map(|e| e.to_string()).collect()),
            None => return Err("File contents invalid: File must have at least one line of text".into()),
        };
    }
    else if config.filename == String::from("datastore.csv") {
        column_labels = match contents.lines().next() {
            Some(v) => Header::new(v.split(",").map(|e| e.to_string()).collect()),
            None => return Err("File contents invalid: File must have at least one line of text".into()),
        };
    }
    else {
        return Err("No valid filename".into())
    }

    //Reassign contents as iterator without column labels line
    let contents = contents.lines().enumerate()
                                .filter_map(|(i, e)| {if i > 0 { Some(e) } else { None }});

    let contents = reassemble(contents);

    //Format the collected lines for presentation, assemble them in structs (Vector used purely as intermediary), and print to screen
    let mut data: Vec<DataRow> = Vec::new();
    
    if config.filename == String::from("covid19-download.csv") {
        let mut line_count = 0;
        for line in contents.lines() {
            let line = default_filter(line);

            let line: Vec<&str> = line.collect();

            let line = DataRow::new(line);

            data.push(line);

            line_count += 1;
            if line_count >= 100 {break}
        }
    }
    else if config.filename == String::from("datastore.csv") {
        for line in contents.lines() {
            let line = line.split(",");

            let line: Vec<&str> = line.collect();

            let line = DataRow::new(line);

            data.push(line);
        }
    }
    else {
        return Err("No valid filename".into())
    }

    Ok((column_labels, data))
}

/// Save the current state of the data in memory to the working CSV file.
/// 
/// A string `s` is constructed by combining the header line with the output of all the
/// data rows (looped through and appended), with the data from each column treated as
/// strings.
pub fn save_data(column_labels: &Header, data: &Vec<DataRow>) -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    for label in column_labels.clone().labels.iter() {
        s.push_str(label);
        s.push_str(",");
    }
    s.pop();
    s.push_str("\n");
    let mut s2 = String::new();

    for col in data {
        for item in col.public_vec() {
            s2.push_str(&item);
            s2.push_str(",");
        }
        s2.pop();
        s2.push_str("\n");
    }

    s.push_str(&s2);
    let mut file = fs::File::create("datastore.csv")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

/// Replaces memory with the data from the original source file, in case
/// of needing to refresh the working data set.
pub fn refresh_data(config: &mut Config) -> Result<(Header, Vec<DataRow>), Box<dyn Error>> {
    config.change_file(String::from("covid19-download.csv"));
    let both = load_data(config)?;
    Ok(both)
}

/// Takes a string in CSV format and returns an iterator without commas and removes undesired columns.
fn default_filter(line: &str) -> impl Iterator<Item = &str> {
    line.split(",")
            .enumerate()
            .filter_map(|(i, e)| if (i < 4) || (i >= 5 && i < 9) || (i == 13) || (i == 15) { Some(e) } else { None })
}

/// Returns a dynamic number of spaces according to an input integer.
fn create_spacing(x: usize) -> String {
    let x: isize = x.try_into().unwrap();
    let mut i = 0;
    let mut s = String::new();
    while i < (x-2) {
        s.push(' ');
        i += 1;
    }
    s.push('\t');
    s
}

/// Reassembles an iterator of strings (lines) into a newline-delimited String.
fn reassemble<'a>(it: impl Iterator<Item = &'a str>) -> String {
    let mut line = String::new();
    for field in it {
        line.push_str(field);
        line.push('\n');
    }
    String::from(line.trim())
}