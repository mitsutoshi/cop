use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Error};

#[derive(Debug, PartialEq)]
pub struct Record {
    pub index: usize,
    pub text: String,
}

#[derive(Debug)]
pub struct DB {
    path: String,
    records: Vec<Record>,
}

fn load(path: &str) -> Vec<Record> {

    let mut records = Vec::new();

    // load data file
    let mut file: Result<File, Error> = OpenOptions::new()
        .create(false)
        .read(true)
        .write(false)
        .open(&path);

    match &mut file {
        Ok(f) => {

            // read file data
            let mut data: String = String::new();
            f.read_to_string(&mut data).unwrap();

            // convert file data to Vec<Record>
            let lines: Vec<&str> = data.split('\n').collect();
            for line in lines {
                if line != "" {
                    let cols: Vec<&str> = line.split(',').collect();
                    let index = cols[0].parse().unwrap();
                    let r = Record {
                        index: index,
                        text: String::from(cols[1]),
                    };
                    records.push(r);
                }
            }
        }
        Err(_) => (())
    };
    records
}

impl DB {
    pub fn from(path: &str) -> DB {
        DB {
            path: String::from(path),
            //records: records,
            records: load(path),
        }
    }

    pub fn add(&mut self, text: &str) -> usize {
        let r = Record {
            index: self.records.len(),
            text: String::from(text),
        };
        self.records.push(r);
        self.store();
        self.records.len() - 1
    }

    pub fn del(&mut self, index: usize) {
        if index < self.records.len() {
            self.records.remove(index);
            self.store();
        }
    }

    pub fn list(&self) -> &Vec<Record> {
        &self.records
    }

    pub fn get(&self, index: usize) -> Option<&Record> {
        if index < self.records.len() {
            Some(&self.records[index])
        } else {
            None
        }
    }

    fn store(&self) {
        // open file
        let mut f: File = OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&self.path)
            .unwrap();

        // write records to file
        let mut data: String = String::new();

        for (i, r) in self.records.iter().enumerate() {
            data += &format!("{},{}\n", i, r.text);
        }
        f.write_all(data.as_bytes()).unwrap();
    }
}
