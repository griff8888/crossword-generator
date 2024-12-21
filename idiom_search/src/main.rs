use std::{fs::File, io::{BufRead, BufReader, Write}};

pub fn reading_in() -> Vec<String>{
    let reader = BufReader::new(File::open("../Idioms.txt").unwrap());
    let mut entries: Vec<String> = vec![];
    let mut count = 0;
    for line in reader.lines() {
        //Turn the line from a result to a &str
        if count%2 == 1{
            count = count + 1;
            continue
        }
        let line: String = String::from(line.unwrap());
        // Assuming we're reading through the file a line at a time and `line`
        // holds the current line of text, we can split the line into parts.
        entries.push(line);
        count = count + 1
    }
    return entries;
}

pub fn search(search_string: &str, search_vec: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for entry in search_vec {
            if entry.to_lowercase().contains(&search_string.to_lowercase())            {
                results.push(entry);
            }
    }

    results
}

pub fn main() {
    let entries = reading_in();
    // for line in &entries{
    //     println!("{line}");
    // }
    loop {
        println!("Welcome to crossword search. We have a database of crossword clues and answers.");
        println!("To search through clues with an answer, enter 'c' ");
        println!("To search through answers with an clue, enter 'a' ");
        println!("Enter a search query or ctrl-d to exit");
        std::io::stdout().flush().unwrap();
        let mut enter = String::new();
        if std::io::stdin().read_line(&mut enter).expect("Failed to read line")== 0{
            break;
        }
        let search_string = &enter.trim().to_lowercase();
        let search_result = search(search_string, entries.clone());
        for entry in search_result{
        println!("{entry}");
        }
        println!("");
    }
}
