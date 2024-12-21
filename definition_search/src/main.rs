use std::{fs::File, io::{BufRead, BufReader}};
use rand::Rng;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Entry {
    word: String,
    def: String,
}

#[derive(Debug)]
pub struct Index {
    entries: Vec<Entry>,
}

fn index(file_path: String) -> Result<Index> {
    let reader = BufReader::new(File::open(file_path)?);
    let mut ct = 0;
    let mut lines = Vec::<String>::new();
    
    let mut index = Index{
        entries: Vec::<Entry>::new(),
    };
    
    for line in reader.lines() {
        ct += 1;
        if ct == 1 {
            continue
        }
        lines.push(line?);
    }

    for line in &lines {
        let parts: Vec<&str> = line.split('"').collect();
        if parts[1].to_string().len() > 2{
            let new_entry = Entry {
                word: parts[1].to_string(),
                def: parts[5].to_string(),
            };
            index.entries.push(new_entry);
        }
    }

    Ok(index)
}

fn get_def<'a>(word: &'a str, index: &'a Index) -> Option<&'a str> {
    for entry in &index.entries {
        if word == entry.word {
            return Some(&entry.def)
        }
    }
    None
}

impl Index {
    fn def_search(&self, search: &str) -> Option<&str> {
        let mut matches = Vec::<&str>::new();
        for word in &self.entries {
            if word.def.to_lowercase().contains(&search.to_lowercase())            {
                matches.push(&word.word);
            }
        }
        let result: &str = matches[rand::thread_rng().gen_range(0..matches.len())];
        Some(result)
    }

    fn word_search(&self, search: &str) -> Option<&str> {
        if search.len() < 3 {
            return Some("Word must be at least 3 characters");
        }
        let mut matches = Vec::<&str>::new();
        for word in &self.entries {
            if word.word.to_lowercase() == (search.to_lowercase()) {
                matches.push(&word.def);
            }
        }
        // let result: &str = matches[rand::thread_rng().gen_range(0..(matches.len() - 1))];
        // Some(result)
        if matches.is_empty() {
            return Some("Word not in dictionary");
        }

        let result = matches[rand::thread_rng().gen_range(0..matches.len())];
        Some(result)

    }
}

fn main() {
    let file_path = String::from("../../dictionary.txt");
    let entries = index(file_path).unwrap();
    //println!("{} ---> {}", entries.entries[100].word, get_def(&entries.entries[100].word, &entries).unwrap());
    // for i in 10000..10500 {
    //     println!("{} ---> {}", entries.entries[i].word, get_def(&entries.entries[i].word, &entries).unwrap());      
    // }
    //println!("{}", entries.def_search("used to").unwrap());
    println!("{}", entries.word_search("jdh").unwrap());
    println!("{}", entries.word_search("gfh").unwrap());
}
