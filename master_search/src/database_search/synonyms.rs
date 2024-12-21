use std::{fs::File, io::{BufRead, BufReader}};

pub fn reading_in() -> Vec<Vec<String>>{
    let reader = BufReader::new(File::open("../synonyms.txt").unwrap());
    let mut synonyms: Vec<Vec<String>> = vec![];
    for line in reader.lines() {
        // Assuming we're reading through the file a line at a time and `line`
        // holds the current line of text, we can split the line into parts.
        let line: &str = &line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        let mut line_list = Vec::new();
        for word in parts {
            if word.chars().count() >= 3{
                line_list.push(word.trim().to_string());
            }
            
        }
        synonyms.push(line_list.clone());
    }
    synonyms
}

pub fn search(search_string: &str, search_vec: Vec<Vec<String>>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for line in search_vec {
        let mut line_copy = line.clone();
        let mut match_found = false;
        for (index, word) in line.clone().into_iter().enumerate() {
            if word.to_lowercase().eq(&search_string.to_lowercase()){
                line_copy.remove(index);
                match_found = true;
            }
        }
        if match_found{
            for word in line_copy {
                results.push(word)
            }
        }
    }
    results
}

// pub fn main() {
//     let entries = reading_in();
//     // for line in &entries{
//     //     println!("{line}");
//     // }
//     loop {
//         println!("Welcome to crossword search. We have a database of crossword clues and answers.");
//         println!("To search through clues with an answer, enter 'c' ");
//         println!("To search through answers with an clue, enter 'a' ");
//         println!("Enter a search query or ctrl-d to exit");
//         std::io::stdout().flush().unwrap();
//         let mut enter = String::new();
//         if std::io::stdin().read_line(&mut enter).expect("Failed to read line")== 0{
//             break;
//         }
//         let search_string = &enter.trim().to_lowercase();
//         let search_result = search(search_string, entries.clone());
//         for find in search_result{
//         println!("{find}");
//         }
//         println!("");
//     }
// }

