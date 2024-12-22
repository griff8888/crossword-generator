use std::{fs::File, io::{BufRead, BufReader}};
// use rand::Rng;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Entry {
    word: String,
    def: String,
}

pub fn index(file_path: String) -> Result<Vec<Entry>> {
    let reader = BufReader::new(File::open(file_path)?);
    let mut ct = 0;
    let mut lines = Vec::<String>::new();
    
    let mut index = Vec::<Entry>::new();
   
    
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
            index.push(new_entry);
        }
    }

    Ok(index)
}

// fn get_def<'a>(word: &'a str, index: &'a Index) -> Option<&'a str> {
//     for entry in &index.entries {
//         if word == entry.word {
//             return Some(&entry.def)
//         }
//     }
//     None
// }


    pub fn def_search(entries: &Vec<Entry>, search: &str) -> Option<Vec<String>> {
        let mut matches = Vec::<String>::new();
        for word in entries {
            if word.def.to_lowercase().contains(&search.to_lowercase()){
                // println!("{}", &word.word);
                matches.push(word.word.clone());
            }
        }
        //let result: &str = matches[rand::thread_rng().gen_range(0..(matches.len() - 1))];
        Some(matches)
    }

    pub fn word_search(entries: &Vec<Entry>, search: &str) -> Option<Vec<String>> {
        let mut matches = Vec::<String>::new();
        for word in entries {
            if word.word.to_lowercase().contains(&search.to_lowercase()){
                // println!("{}", &word.def);
                matches.push(word.def.clone());
            }
        }
        // let result: &str = matches[rand::thread_rng().gen_range(0..(matches.len() - 1))];
        if matches.is_empty() {
            matches.push(String::from("Word not in dictionary"));
            return Some(matches);
        }
        Some(matches)
    }


// fn main() {
//     let file_path = String::from("../../dictionary.txt");
//     let entries = index(file_path).unwrap();
//     println!("{} ---> {}", entries.entries[100].word, get_def(&entries.entries[100].word, &entries).unwrap());
//     println!("{}", entries.def_search("used to").unwrap());
//     println!("{}", entries.word_search("ange").unwrap());
// }
 