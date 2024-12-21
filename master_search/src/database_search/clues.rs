use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};


pub fn reading_in() -> Vec<Vec<String>>{
    let reader = BufReader::new(File::open("../nytc.txt").unwrap());
    let mut entries: Vec<Vec<String>> = vec![];
    let mut count = 0;
    for line in reader.lines() {
        count += 1;
        if count == 1 {
            continue
        }
        let lines: String = line.unwrap();
        let mut parts: Vec<String> = lines.split(',').map(str::to_string).collect();
        parts.remove(0);
        entries.push(parts);
    }
    entries
}

pub fn clue_search(search_string: &str, search_vec: Vec<Vec<String>>) -> Vec<String> {
    let mut results = HashSet::new();
    for entry in search_vec {
        if entry[0].to_lowercase() == search_string.to_lowercase() {
            let result = entry[1..].join(",");
            let cleaned_result = result.replace("\"", ""); // Remove quotation marks
            results.insert(cleaned_result.to_lowercase());
        }
    }
    results.into_iter().collect()
}

pub fn ans_search(search_string: &str, search_vec: Vec<Vec<String>>) -> Vec<String> {
    let mut results = HashSet::new();
    for entry in search_vec {
        let joined = entry[1..].join(",");
        if joined.to_lowercase().contains(&search_string.to_lowercase()) {
            let result = &entry[0];
            let cleaned_result = result.replace("\"", ""); // Remove quotation marks
            results.insert(cleaned_result.to_lowercase());
        }
    }
    results.into_iter().collect()
}

// pub fn main() {
//     let entries = reading_in();
//     println!("Welcome to crossword search. We have a database of crossword clues and answers.");
//     println!("To search through clues with an answer, enter 'c' ");
//     println!("To search through answers with an clue, enter 'a' ");
//     std::io::stdout().flush().unwrap();
//     let mut choice = String::new();
//     if std::io::stdin().read_line(&mut choice).expect("Failed to read line")== 0 {
//         return
//     }
//     choice = choice.trim().to_string();
//     if choice.to_lowercase() == "c".to_string() {
//         loop {
//             print!("Enter a search query or ctrl-d to exit: ");
//             std::io::stdout().flush().unwrap();
//             let mut enter = String::new();
//             if std::io::stdin().read_line(&mut enter).expect("Failed to read line")== 0 {
//                 break;
//             }
//             let search_string = &enter.trim().to_lowercase();
//             let search_result = clue_search(search_string, entries.clone());
//             for entry in search_result {
//                 println!("{entry}");
//             }
//             println!("");
//         }
//     } else if choice.to_lowercase() == "a".to_string() {
//         loop {
//             println!();
//             print!("Enter a search query or ctrl-d to exit: ");
//             std::io::stdout().flush().unwrap();
//             let mut enter = String::new();
//             if std::io::stdin().read_line(&mut enter).expect("Failed to read line")== 0 {
//                 break;
//             }
//             let search_string = &enter.trim().to_lowercase();
//             let search_result = ans_search(search_string, entries.clone());
//             for entry in search_result {
//                 println!("{entry}");
//             }
//             println!("");
//         }
//     } else {
//         println!("Not a valid selection");
//     }
// }
