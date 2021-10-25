use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

use flate2::read::MultiGzDecoder;

#[derive(Debug)]
pub struct Whitelist {
    list: HashSet<String>,
    mapping: Option<HashMap<String, String>>,
    size: usize,
    counts: HashMap<String, usize>
}
impl Whitelist {
    /// Creates a whitelist from a provided text file
    /// where each line represents a new value
    pub fn from_file(filename: &str) -> Self {
        let file = File::open(filename).expect("Error: Unable to open provided filename");
        let mut bufr = BufReader::new(file);
        let mut line = String::new();
        let mut list = HashSet::new();

        let mut size = 0;
        loop {
            line.clear();
            if bufr.read_line(&mut line).expect("Error Reading") == 0 {break;}
            let line = line.trim();
            if size == 0 {
                size = line.len();
            }
            else {
                if line.len() == size {
                    if list.contains(line) {
                        panic!("Duplicate sequence found");
                    }
                    else {
                        list.insert(line.to_string());
                    }
                }
                else {
                    panic!("Irregular sizes in whitelist");
                }
            }
            list.insert(line.to_string());
        }

        Self {
            list,
            mapping: None,
            size,
            counts: HashMap::new()
        }
    }

    /// Creates a whitelist from a provided gzip text file
    /// where each line represents a new value
    pub fn from_gzip_file(filename: &str) -> Self {
        let file = File::open(filename).expect("Error: Unable to open provided filename");
        let gfile = MultiGzDecoder::new(file);
        let mut bufr = BufReader::new(gfile);
        let mut line = String::new();
        let mut list = HashSet::new();
        let mut size = 0;

        loop {
            line.clear();
            if bufr.read_line(&mut line).expect("Error Reading") == 0 {break;}
            let line = line.trim();
            if size == 0 {
                size = line.len();
            }
            else {
                if line.len() == size {
                    if list.contains(line) {
                        panic!("Duplicate sequence found");
                    }
                    else {
                        list.insert(line.to_string());
                    }
                }
                else {
                    panic!("Irregular sizes in whitelist");
                }
            }
            list.insert(line.to_string());
        }

        Self {
            list,
            mapping: None,
            size,
            counts: HashMap::new()
        }
    }

    /// Creates a whitelist from a provided text file
    /// where each line contains a value and an alias
    pub fn from_table(filename: &str, sep: char) -> Self {
        let file = File::open(filename).expect("Error: Unable to open provided filename");
        let mut bufr = BufReader::new(file);
        let mut line = String::new();
        let mut list = HashSet::new();
        let mut mapping = HashMap::new();
        let mut size = 0;

        loop {
            line.clear();
            if bufr.read_line(&mut line).expect("Error Reading") == 0 {break;}
            let mut split_line = line.trim().split(sep);
            let seq = split_line.next().expect("Malformed Table");
            let name = split_line.next().expect("Malformed Table");
            if size == 0 {
                size = seq.len();
            }
            else {
                if size == seq.len() {
                    if list.contains(seq) {
                        panic!("Duplicate sequence found");
                    }
                    else{
                        list.insert(seq.to_string());
                        mapping.insert(seq.to_string(), name.to_string());
                    }
                }
                else {
                    panic!("Irregular sizes in whitelist");
                }
            }

        }

        Self {
            list,
            mapping: Some(mapping),
            size,
            counts: HashMap::new()
        }
    }

    /// Checks if provided sequence is within the whitelist
    pub fn contains(&self, seq: &str) -> bool {
        self.list.contains(&seq[0..self.size])
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn increment_counts(&mut self, s: &str) {
        if self.list.contains(&s[0..self.size]) {
            let counter = self.counts.entry(s[0..self.size].to_string()).or_insert(0);
            *counter += 1;
        }
    }

    pub fn pretty_print(&self) {
        for (key, value) in &self.counts {
            match self.mapping.is_some() {
                true => {
                    let alias = self.mapping
                        .as_ref()
                        .unwrap()
                        .get(key)
                        .expect("Error: Missing Key in alias map");
                    println!("{}\t{}", alias, value);
                },
                false => {
                    println!("{}\t{}", key, value);
                }
            }
        }
    }
}
