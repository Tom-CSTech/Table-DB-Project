//Final Project - Thomas Ivanov

use std::convert::TryInto;

use crate::logic::Config;
use crate::logic::Lang;

/// A data row object (struct) used to manipulate stored data from file.
/// 
/// A `DataRow` has an output function for each available presentation language, as well as a 
/// constructor function `new(line: Vec<&str>)`.
/// 
/// # Examples
/// 
/// ```
/// let data = DataRow::new(str_vector);
/// assert_eq!(data.prname, str_vector[1]);
/// ```
#[derive(Clone)]
#[derive(Debug)]
pub struct DataRow {
    pub pruid: isize,
    pub prname: String,
    pub prname_fr: String,
    pub date: String,
    pub numconf: isize,
    pub numprob: isize,
    pub numdeaths: isize,
    pub numtotal: isize,
    pub numtoday: isize,
    pub ratetotal: f64,
}

impl DataRow {
    /// A function for creating new `DataRow` structs from a vector.
    pub fn new(line: Vec<&str>) -> DataRow {
        DataRow {
            pruid: if line[0] == "" {0} else {line[0].parse().unwrap()},
            prname: String::from(line[1]),
            prname_fr: String::from(line[2]),
            date: String::from(line[3]),
            numconf: if line[4] == "" {0} else {line[4].parse().unwrap()},
            numprob: if line[5] == "" {0} else {line[5].parse().unwrap()},
            numdeaths: if line[6] == "" {0} else {line[6].parse().unwrap()},
            numtotal: if line[7] == "" {0} else {line[7].parse().unwrap()},
            numtoday: if line[8] == "" {0} else {line[8].parse().unwrap()},
            ratetotal: if line[9] == "" {0.00} else {line[9].parse().unwrap()},
        }
    }

    /// Produces a vector of strings representing the data row
    pub fn public_vec(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        v.push(self.pruid.clone().to_string());
        v.push(self.prname.clone());
        v.push(self.prname_fr.clone());
        v.push(self.date.clone());
        v.push(self.numconf.clone().to_string());
        v.push(self.numprob.clone().to_string());
        v.push(self.numdeaths.clone().to_string());
        v.push(self.numtotal.clone().to_string());
        v.push(self.numtoday.clone().to_string());
        v.push(self.ratetotal.clone().to_string());
        v
    }

    /// A function for outputting the formatted current-language version of the data.
    pub fn output_lang(&self, config: &Config) -> String {
        let mut line = String::new();

        line.push_str(&col_spacing(self.pruid.to_string(), 8));
        line.push_str(&col_spacing(match config.language {Lang::EN => self.prname.clone(), Lang::FR => self.prname_fr.clone()}, 30));
        line.push_str(&col_spacing(self.date.clone(), 15));
        line.push_str(&col_spacing(self.numconf.to_string(), 10));
        line.push_str(&col_spacing(self.numprob.to_string(), 10));
        line.push_str(&col_spacing(self.numdeaths.to_string(), 10));
        line.push_str(&col_spacing(self.numtotal.to_string(), 10));
        line.push_str(&col_spacing(self.numtoday.to_string(), 10));
        line.push_str(&col_spacing(self.ratetotal.to_string(), 10));

        line
    }
    /// A function for outputting data including both language-dependent columns
    pub fn output_all(&self) -> String {
        let mut line = String::new();

        line.push_str(&col_spacing(self.pruid.to_string(), 8));
        line.push_str(&col_spacing(self.prname.clone(), 30));
        line.push_str(&col_spacing(self.prname_fr.clone(), 30));
        line.push_str(&col_spacing(self.date.clone(), 15));
        line.push_str(&col_spacing(self.numconf.to_string(), 10));
        line.push_str(&col_spacing(self.numprob.to_string(), 10));
        line.push_str(&col_spacing(self.numdeaths.to_string(), 10));
        line.push_str(&col_spacing(self.numtotal.to_string(), 10));
        line.push_str(&col_spacing(self.numtoday.to_string(), 10));
        line.push_str(&col_spacing(self.ratetotal.to_string(), 10));

        line
    }
}

/// Basic implementation of PartialEq trait for equality comparison.
impl PartialEq for DataRow {
    fn eq(&self, other: &Self) -> bool {
        self.pruid == other.pruid &&
        self.prname == other.prname &&
        self.prname_fr == other.prname_fr &&
        self.date == other.date &&
        self.numconf == other.numconf &&
        self.numprob == other.numprob &&
        self.numdeaths == other.numdeaths &&
        self.numtotal == other.numtotal &&
        self.numtoday == other.numtoday &&
        self.ratetotal == other.ratetotal
    }

    fn ne(&self, other: &Self) -> bool {
        self.pruid != other.pruid ||
        self.prname != other.prname ||
        self.prname_fr != other.prname_fr ||
        self.date != other.date ||
        self.numconf != other.numconf ||
        self.numprob != other.numprob ||
        self.numdeaths != other.numdeaths ||
        self.numtotal != other.numtotal ||
        self.numtoday != other.numtoday ||
        self.ratetotal != other.ratetotal
    }
}

/// Returns String with a dynamic number of spaces according to an input integer.
fn col_spacing(mut s: String, x: usize) -> String {
    let x: isize = (x - s.len()).try_into().unwrap();
    let mut i = 0;
    while i < (x-2) {
        s.push(' ');
        i += 1;
    }
    s.push('\t');
    s
}

#[derive(Clone)]
/// Header struct holds the labels to be always displayed on the first line
pub struct Header {
    pub labels: Vec<String>,
}

impl Header {
    /// A function for creating new `DataRow` structs from a vector.
    pub fn new(labels: Vec<String>) -> Header {
        Header {
            labels
        }
    }
    
    /// A function for outputting the formatted English-language version of the data.
    pub fn output_lang(&self, config: &Config) -> String {
        let mut line = String::new();

        line.push_str(&col_spacing(self.labels[0].clone(), 8));
        line.push_str(&col_spacing(match config.language {Lang::EN => self.labels[1].clone(), Lang::FR => self.labels[2].clone()}, 30));
        line.push_str(&col_spacing(self.labels[3].clone(), 15));
        line.push_str(&col_spacing(self.labels[4].clone(), 10));
        line.push_str(&col_spacing(self.labels[5].clone(), 10));
        line.push_str(&col_spacing(self.labels[6].clone(), 10));
        line.push_str(&col_spacing(self.labels[7].clone(), 10));
        line.push_str(&col_spacing(self.labels[8].clone(), 10));
        line.push_str(&col_spacing(self.labels[9].clone(), 10));

        line
    }

    /// A function for outputting the formatted French-language version of the data.
    pub fn output_all(&self) -> String {
        let mut line = String::new();

        line.push_str(&col_spacing(self.labels[0].clone(), 8));
        line.push_str(&col_spacing(self.labels[1].clone(), 30));
        line.push_str(&col_spacing(self.labels[2].clone(), 30));
        line.push_str(&col_spacing(self.labels[3].clone(), 15));
        line.push_str(&col_spacing(self.labels[4].clone(), 10));
        line.push_str(&col_spacing(self.labels[5].clone(), 10));
        line.push_str(&col_spacing(self.labels[6].clone(), 10));
        line.push_str(&col_spacing(self.labels[7].clone(), 10));
        line.push_str(&col_spacing(self.labels[8].clone(), 10));
        line.push_str(&col_spacing(self.labels[9].clone(), 10));

        line
    }
}