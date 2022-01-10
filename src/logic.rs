//Final Project - Thomas Ivanov

#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;
use std::env;
use std::io::{self, Write};
use std::convert::TryInto;
use chrono::NaiveDate;
use std::cmp::Reverse;

use crate::datastore::*;
use crate::persistence;


/// A configuration object for carrying command-line arguments and Environment variables into program memory.
/// 
/// Function <a href="file:///E:/rust/projects/asgmt1/target/doc/asgmt1/struct.Config.html#method.new">new(args: Args)</a> 
/// creates a new Config object with the appropriate parameters for the user-provided arguments. Enum 
/// <a href="file:///E:/rust/projects/asgmt1/target/doc/asgmt1/enum.Lang.html">Lang</a> is used as the 
/// language parameter.
/// 
/// # Examples
/// 
/// ```
/// my_project> cargo run en
/// ```
/// 
/// ```
/// let config = Config::new(env::args());
/// assert_eq!(Lang::EN, config.language);
/// ```
pub struct Config {
    pub language: Lang,
    pub filename: String,
}

/// Language variant parameter for Config type.
#[derive(PartialEq)]
pub enum Lang {
    EN,
    FR,
}

impl Config {
    /// A function for outputting a new Config struct based on command line arguments.
    pub fn new(mut args: env::Args) -> Result<Config, Box<dyn Error>> {
        args.next();

        //If arg is found, set config language, else output error
        let language = match args.next() {
            Some(arg) => {match &arg[..] {"en" => Lang::EN, "fr" => Lang::FR, _ => return Err("Must supply a valid language (en/fr)".into())}},
            None => return Err("Must supply a language option (en/fr)".into()),
        };

        //Optional allowing filename input argument (Disabled)
        /*let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };*/

        //Return wrapped configuration struct (Default filename enabled)
        Ok(Config {
            language,
            filename: String::from("covid19-download.csv"),
        })
    }
    pub fn change_lang(&mut self, lang: Lang) {
        self.language = lang;
    }
    pub fn change_file(&mut self, file: String) {
        self.filename = file;
    }
}

/// Run the data paginating system to present the user with an interactable command-line UI.
/// 
/// The `display` function creates a `counter` variable for the purposes of tracking page cycles. The 
/// displayed lines of text are a slice of the `data` vector which is over a different part of the 
/// vector depending on `counter`. Lines are printed using the appropriate language output method 
/// depending on the `Config` struct `language` state.
/// 
/// The user is prompted once the preample plus 50 lines have been printed, and they can leave the 
/// program by entering 'q' or 'Q'.
pub fn display(config: &Config, column_labels: &Header, data: &Vec<DataRow>) -> Result<(), Box<dyn Error>> {
    let mut active_data = data.clone();
    let line_count = active_data.iter().count();
    let mut counter = 0;
    let mut sorting = String::from("0");
    let mut rev = false;

    //Loop for paginating output
    while line_count > 25*counter {
        //Line below clears console window
        std::process::Command::new("cmd").args(&["/C","cls"]).status().expect("Failed to execute process (CLS)");

        //Print header lines, including column labels
        println!("Covid Data CLI App - Thomas Ivanov");
        print!("{}\n", column_labels.output_lang(config));

        //Use line count to determine how to split text for presentation
        let page: &[DataRow] = &active_data[25*counter..{if 25*(counter+1) < line_count {25*(counter+1)} else {line_count}}];


        //Format the collected lines for presentation, assemble them in structs (Vector used purely as intermediary), and print to screen
        for line in page.iter() {
            println!("{}", line.output_lang(config));
        }

        //Remove the 50 lines previously read from content collection, then increment counter
        counter += 1;

        //Process user input to determine whether to show next page, sort the output, or quit
        println!("\nPage: {}\n", counter);
        println!("\n[Press ENTER to read more, S to sort output by column, R to reverse output (asc-desc), or Q to go back]");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.to_lowercase().trim() {
            "q" => {break},
            "s" => {
                        active_data = data.clone();
                        //Option "s" is for sorting the page output by a column. The columns chosen are sorted for simultaneously, in order.
                        print!("\nChoose one or more columns (1-9, separated by commas) to sort for, in order of priority: ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let comma_sep: Vec<String> = input.split(",").map(|e| e.trim().to_string()).collect();

                        let mut i = comma_sep.len();

                        //loop for each column sorting
                        while i > 0 {
                            //"sorting" is a variable that tracks what column was most recently sorted by, for future reference
                            sorting = comma_sep[i-1].clone();

                            match sort(&mut active_data, &sorting, config) {
                                Ok(()) => {
                                    //"rev" is a variable that tracks whether the sorting order is reversed
                                    rev = false;
                                    //counter must be set to 0 to start displaying from page 1 again
                                    counter = 0;
                                },
                                Err(e) => {
                                    counter -= 1;
                                }
                            }
                            i -= 1;
                        }
                    },
            "r" => {
                        //Option "r" accounts for the most recent sort and reverses it, or un-reverses it if already reversed
                        reverse(&mut active_data, &sorting, &mut rev, config);
                        counter = 0;
                    },
            _ => {}
        }
    }

    Ok(())
}

/// Function for by mutating vector of data rows to sort it by some column
fn sort(data: &mut Vec<DataRow>, sorting: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    match sorting {
        "1" =>  {
                    //sort_by_key is a built-in function that sorts a vector of structs by field
                    //It mutates the vector, so that's why I clone the original vector at the top of this function
                    data.sort_by_key(|r| r.pruid);
                }
        "2" =>  {
                    data.sort_by_key(|r| if config.language == Lang::EN {r.prname.clone()} else {r.prname_fr.clone()});
                }
        "3" =>  {
                    data.sort_by_key(|r| r.date.clone());
                }
        "4" =>  {
                    data.sort_by_key(|r| r.numconf);
                }
        "5" =>  {
                    data.sort_by_key(|r| r.numprob);
                }
        "6" =>  {
                    data.sort_by_key(|r| r.numdeaths);
                }
        "7" =>  {
                    data.sort_by_key(|r| r.numtotal);
                }
        "8" =>  {
                    data.sort_by_key(|r| r.numtoday);
                }
        "9" =>  {
                    data.sort_by(|r, s| r.ratetotal.partial_cmp(&s.ratetotal).unwrap());
                }
        _   =>  {println!("Please select a valid number (1-9)");return Err("".into())}
    };
    Ok(())
}

/// Function for reversing the order of the current column sort by mutating vector of data rows
fn reverse(data: &mut Vec<DataRow>, sorting: &str, rev: &mut bool, config: &Config) {
    match sorting {
        "1" =>  {
                    //Firstly matches the most recent sorting method by checking the "sorting" variable
                    if *rev == false {
                        //If output is not reversed already for the current sorting option, reverses it 
                        data.sort_by_key(|r| Reverse(r.pruid));
                        *rev = true;
                    }
                    else {
                        //If output is reversed already for this sorting option, reverses it back to forward order
                        data.sort_by_key(|r| r.pruid);
                        *rev = false;
                    }
                }
        "2" =>  {
                    if *rev == false {
                        data.sort_by_key(|r| if config.language == Lang::EN {Reverse(r.prname.clone())} else {Reverse(r.prname_fr.clone())});
                        *rev = true;
                    }
                    else {
                        data.sort_by_key(|r| if config.language == Lang::EN {r.prname.clone()} else {r.prname_fr.clone()});
                        *rev = false;
                    }
                }
        "3" =>  {
                    if *rev == false {
                        data.sort_by_key(|r| Reverse(r.date.clone()));
                        *rev = true;
                    }
                    else {
                        data.sort_by_key(|r| r.date.clone());
                        *rev = false;
                    }
                }
        "4" =>  {
                    if *rev == false {
                        data.sort_by_key(|r| Reverse(r.numconf));
                        *rev = true;
                    }
                    else {
                        data.sort_by_key(|r| r.numconf);
                        *rev = false;
                    }
                }
        "5" =>  {
                    if *rev == false {
                        data.sort_by_key(|r| Reverse(r.numprob));
                        *rev = true;
                    }
                    else {
                        data.sort_by_key(|r| r.numprob);
                        *rev = false;
                    }
                }
        "6" =>  {
                    if *rev == false {
                        data.sort_by_key(|r| Reverse(r.numdeaths));
                        *rev = true;
                    }
                    else {
                        data.sort_by_key(|r| r.numdeaths);
                        *rev = false;
                    }
                }
        "7" =>  {
                    if *rev == false {
                        data.sort_by_key(|r| Reverse(r.numtotal));
                        *rev = true;
                    }
                    else {
                        data.sort_by_key(|r| r.numtotal);
                        *rev = false;
                    }
                }
        "8" =>  {
                    if *rev == false {
                        data.sort_by_key(|r| Reverse(r.numtoday));
                        *rev = true;
                    }
                    else {
                        data.sort_by_key(|r| r.numtoday);
                        *rev = false;
                    }
                }
        "9" =>  { 
                    if *rev == false {
                        data.sort_by(|r, s| Reverse(r.ratetotal).partial_cmp(&Reverse(s.ratetotal)).unwrap());
                        *rev = true;
                    }
                    else {
                        data.sort_by(|r, s| r.ratetotal.partial_cmp(&s.ratetotal).unwrap());
                        *rev = false;
                    }
                }
        _   =>  { 
                    if *rev == false {
                        data.sort_by_key(|r| Reverse(r.date.clone()));
                        *rev = true;
                    }
                    else {
                        data.sort_by_key(|r| r.date.clone());
                        *rev = false;
                    }
                }
    };
}

/// Search for a specific set of data rows by index on the set provided
/// 
/// The 'search' function takes a search string along with the configuration data, header and data to 
/// filter, and parses the search string using both commas for enumerated indices, and dashes to
/// denote a range of indices. The set of matching indices is returned as a vector to use when needed.
pub fn search(search_index: String, config: &Config, data: &Vec<DataRow>) -> Result<Vec<usize>, Box<dyn Error>> {
    //final_data is the filtered list of data row indices we want to display from
    let mut final_data: Vec<usize> = Vec::new();
    
    //comma_sep is our user-input entry indices separated by the commas and placed into a vector
    let comma_sep: Vec<String> = search_index.split(",").map(|e| e.to_string()).collect();
    let mut dash_sep: Vec<Vec<String>> = Vec::new();
    for s in comma_sep {
        //dash_sep is our user-input entry indices separated now also by dashes and placed into new sub-vectors
        dash_sep.push(s.split("-").map(|e| e.to_string()).collect());
    }
    for index in dash_sep {
        //If sub-vector is length 1 it didn't have a dash so doesn't denote a range - grab one data row only
        if index.len() == 1 {
            let index = index.iter().next().unwrap().trim();
            final_data.push(index.parse::<usize>()?);
        }
        //If sub-vector is length 1 it had dash to denote a range - grab a range of data rows
        else if index.len() == 2 {
            let mut index_vec = index.iter();
            let index = index_vec.next().unwrap().trim();
            let first = index.parse::<usize>()?;
            let index = index_vec.next().unwrap().trim();
            let last = index.parse::<usize>()?;
            for i in first..last {
                final_data.push(i);
            }
        }
        //Any length of sub-vector other than 1 or 2 is invalid
        else {
            return Err("Search string parameter invalid. Please refer to the following example for formatting (without quotes):
                            \"4, 7, 9-14\"\n".into())
        }
    }

    Ok(final_data)
}

pub fn edit(search_index: String, config: &Config, column_labels: &Header, data: &mut Vec<DataRow>) -> Result<(), Box<dyn Error>> {
    //The index of the data row we want to edit
    let search_index = search_index.trim().parse::<usize>()?;
    if search_index < data.len() && data.len() != 0 {
        loop {
            let mut input = String::new();
            println!("Choose a column (1-10) to edit from the following (row {}). Enter Q to quit.\n{}\n{}", 
                                                        search_index.to_string(), column_labels.output_all(), data[search_index].output_all());
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            //The number of the column whose data we want to edit
            match input {
                "1" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 3 digits)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim().parse::<usize>()?;
                            //Newly input column data is validated, then overwritten if valid (resets the loop if invalid, skipping the overwriting)
                            if input > 999 {println!("Invalid number (must be 0-999), please try again.");continue}
                            data[search_index].pruid = input.try_into().unwrap();
                        }
                "2" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 30 characters)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            if input.len() > 30 {println!("Invalid name (must be less than 30 characters long), please try again.");continue}
                            data[search_index].prname = String::from(input.trim());
                        }
                "3" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 30 characters)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            if input.len() > 30 {println!("Invalid name (must be less than 30 characters long), please try again.");continue}
                            data[search_index].prname = String::from(input.trim());
                        }
                "4" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 15 characters)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim();
                            if input.len() > 30 {println!("Invalid date (must be less than 15 characters long), please try again.");continue}
                            match NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
                                Ok(date) => {data[search_index].date = String::from(input);},
                                Err(e) => {println!("Invalid date format, please try again.");continue},
                            }
                        }
                "5" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 8 digits)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim().parse::<usize>()?;
                            if input > 99999999 {println!("Invalid number (must be 0-99999999), please try again.");continue}
                            data[search_index].numconf = input.try_into().unwrap();
                        }
                "6" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 8 digits)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim().parse::<usize>()?;
                            if input > 99999999 {println!("Invalid number (must be 0-99999999), please try again.");continue}
                            data[search_index].numprob = input.try_into().unwrap();
                        }
                "7" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 8 digits)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim().parse::<usize>()?;
                            if input > 99999999 {println!("Invalid number (must be 0-99999999), please try again.");continue}
                            data[search_index].numdeaths = input.try_into().unwrap();
                        }
                "8" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 8 digits)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim().parse::<usize>()?;
                            if input > 99999999 {println!("Invalid number (must be 0-99999999), please try again.");continue}
                            data[search_index].numtotal = input.try_into().unwrap();
                        }
                "9" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 8 digits)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim().parse::<usize>()?;
                            if input > 99999999 {println!("Invalid number (must be 0-99999999), please try again.");continue}
                            data[search_index].numtoday = input.try_into().unwrap();
                        }
                "10" =>  {
                            let mut input = String::new();
                            println!("Choose a new value for this line (max 99999.99, no more than two digits after decimal point)");
                            io::stdin().read_line(&mut input).expect("Failed to read line");
                            let input = input.trim();
                            let input_flt = input.parse::<f64>()?;
                            if input_flt > 99999.99 {println!("Invalid number (must be 0.00-99999.99), please try again.");continue}
                            let flt = input.match_indices('.').nth(0).map(|(index, _)| input.split_at(index)).unwrap();
                            if flt.1.len()-1 > 2 {println!("Invalid number (must have no more than two digits after the decimal), please try again.");continue}
                            data[search_index].ratetotal = input_flt;
                        }
                "q" | "Q" => break,
                _   =>  {println!("Please select a valid number (1-9)");continue}
            }
            let mut input = String::new();
            println!("Do you want to keep editing? Enter Q to quit (any key to continue)");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            //Quit the user interaction editing loop
            match input.trim() {
                "q" | "Q" => break,
                _ => continue
            }
        }

    }
    Ok(())
}

pub fn delete(search_index: String, config: &Config, column_labels: &Header, data: &mut Vec<DataRow>) -> Result<(), Box<dyn Error>> {
    //The data row index to delete
    let search_index = search_index.trim().parse::<usize>()?;
    if search_index < data.len() && data.len() != 0 {
        let mut input = String::new();
        println!("Do you want to delete the following (row {})? y/N\n{}", 
                                        search_index.to_string(), data[search_index].output_lang(config));
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        //Removing the data from the vector of all the data in memory
        if (input == "Y") | (input == "y") {
            data.remove(search_index);
        }
    }
    Ok(())
}

//--IO function calls routed to persistence--//

pub fn load(config: &Config) -> Result<(Header, Vec<DataRow>), Box<dyn Error>> {
    persistence::load_data(config)
}

pub fn save(column_labels: &Header, data: &Vec<DataRow>) -> Result<(), Box<dyn Error>> {
    persistence::save_data(&column_labels, &data)
}

pub fn refresh(config: &mut Config) -> Result<(Header, Vec<DataRow>), Box<dyn Error>> {
    persistence::refresh_data(config)
}

#[cfg(test)]
mod test {
    use crate::logic;
    use crate::persistence;
    use crate::datastore;

    #[test]
    fn test_refresh() {
        let mut config = logic::Config {language: logic::Lang::EN, filename: String::from("covid19-download.csv")};
        let data_original: Vec<datastore::DataRow>;

        let both = persistence::load_data(&config).unwrap();
        data_original = both.1;
        let mut data = data_original.clone();

        data.remove(1);
        assert_ne!(data, data_original);

        let both = persistence::refresh_data(&mut config).unwrap();
        data = both.1;
        assert_eq!(data, data_original);
    }

    #[test]
    fn test_sort() {
        let config = logic::Config {language: logic::Lang::EN, filename: String::from("covid19-download.csv")};
        let data_original: Vec<datastore::DataRow>;

        let both = persistence::load_data(&config).unwrap();
        data_original = both.1;

        let search_result = logic::search(String::from("0-3"), &config, &data_original).unwrap();
        let mut out: Vec<datastore::DataRow> = Vec::new();
        
        for entry in search_result {
            if entry < data_original.len() {
                out.push(data_original[entry].clone());
            }
        }

        let data_original = out;

        let mut data_sorted = data_original.clone();

        let mut rev = false;
        let sorting = "8";

        logic::sort(&mut data_sorted, sorting, &config).unwrap();
        assert_ne!(data_original, data_sorted);
        assert_eq!(data_original[0], data_sorted[1]);
        assert_eq!(data_original[1], data_sorted[0]);
        assert_eq!(data_original[2], data_sorted[2]);
        let mut data_reversed = data_sorted.clone();
        logic::reverse(&mut data_reversed, sorting, &mut rev, &config);
        assert_ne!(data_sorted, data_reversed);
        assert_eq!(data_sorted[0], data_reversed[2]);
        assert_eq!(data_sorted[1], data_reversed[1]);
        assert_eq!(data_sorted[2], data_reversed[0]);
    }
}