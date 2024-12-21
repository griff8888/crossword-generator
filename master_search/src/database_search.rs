use std::{io::Write, process::exit};
mod clues;
mod dict;
mod idioms;
mod synonyms;


pub fn welcome() -> Vec<String>{
    println!("Please select the databases you would like to search by entering the corresponding letter(s)");
    //Prints database choices
    println!("A. New York Times Crossword Clues Database");
    println!("B. Idioms Database");
    println!("C. Synonyms Database");
    println!("D. Dictionary Database");
    println!("E. All Databases");
    println!();
    print!("Please enter your choice here: ");
    //Flushes standard out and reads in the choice
    std::io::stdout().flush().unwrap();
    let mut choice = String::new();
    let _ = std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line")
        == 0;
    //Trims choice and initializes crossword and dictionary choices
    let mut choices: Vec<String> = Vec::new();
    let mut databases = choice.trim().to_lowercase();
    let mut cross = "";
    let mut dict = "";
    
    //Check to make sure only has A, B, C, or D
    //If A is present, add nyt to databases vector
    //if b is present, add idioms to vec
    //if c dict
    //d syn
    //Checks to see if the database chosen is the NYT Database (a) or the dictionary database (d)
    if databases.eq("a") {
        //Welcomes user to crossword search and prompts them to choose whether to search for clue or answers using the other
        println!("Welcome to crossword search. We have a database of crossword clues and answers.");
        println!("To search through answers with an clue, enter 'a' ");
        println!("To search through clues with an answer, enter 'c' ");
        print!("Please enter your choice here: ");
        //Flushes standard out and reads in choice
        std::io::stdout().flush().unwrap();
        let mut choice = String::new();
        if std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line")
            == 0
        {
            exit(0);
        }
        //Trims choice and checks what the user chose and assigns cross to the appropriate choice
        choice = choice.trim().to_string();
        if choice.to_lowercase() == "a" {
            cross = "a";
        } else if choice.to_lowercase() == "c" {
            cross = "c";
        }
        //If a or c is not chosen, sets databases to f so the program exits in the next step
        else {
            databases = "f".to_string();
        }
    } else if databases.eq("d") {
        //Welcomes user to the dictionary search and prompts them to choose whether to search for the words or definitions using the other
        println!("Welcome to the dictionary search. We have a database of dictionary words and definitions.");
        println!("To search through words with a definition (or a part of one), enter 'd' ");
        println!("To search through definitions with a word, enter 'w' ");
        print!("Please enter your choice here: ");
        //Flushes standard out and reads in choice
        std::io::stdout().flush().unwrap();
        let mut choice = String::new();
        if std::io::stdin() .read_line(&mut choice) .expect("Failed to read line") == 0{
            exit(0);
        }
        //Trims choice and checks what the user chose and assigns dict to the appropriate choice
        choice = choice.trim().to_string();
        if choice.to_lowercase() == "d" {
            dict = "d";
        } else if choice.to_lowercase() == "w" {
            dict = "w";
        }
        //if d or w is not chosen, sets database to f so the program exits in the next step
        else {
            databases = "f".to_string();
        }
    }
    //Checks user input for valid database selection exiting if a, b, c, d, or e is not selected
    if !(databases.eq("a")||databases.eq("b")||databases.eq("c")||databases.eq("d")||databases.eq("e")){
        println!("This is not a valid choice. Please try again later. Goodbye");
        exit(0);
    }
    choices.push(databases);
    choices.push(cross.to_string());
    choices.push(dict.to_string());
    return choices

}
pub fn databases() {
    //reads in all the files
    let idiom_entries = idioms::reading_in();
    let clues_entries = clues::reading_in();
    let file_path = String::from("../dictionary.txt");
    let dict_entries = dict::index(file_path).unwrap();
    let syn_entries = synonyms::reading_in();
    //Welcomes user to search engine
    println!("Welcome to the crossword database!");
    let options: Vec<String> = welcome();
    //If a valid input is selected, a line is printed for aesthetic purposes and the search loop is entered
    println!();
    let mut databases = options[0].clone();
    let mut cross = options[1].clone();
    let mut dict = options[2].clone();
    loop {
        //The user is prompted to enter a serach query and standard out is flushed
        print!("Enter a search query or ctrl-c to exit: ");
        std::io::stdout().flush().unwrap();
        //The entered string is read in and trimmed
        let mut enter = String::new();
        if std::io::stdin()
            .read_line(&mut enter)
            .expect("Failed to read line")
            == 0
        {
            continue;
        }
        let search_string = &enter.trim().to_lowercase();
        //The results vectors are initialized
        let mut search_results: Vec<String> = Vec::new();
        //The database chosen is checked
        //If the NYT database is chosen, the cross variable is checked to check whether the answers or clues should be searched
        if databases.eq("a") {
            //If a, then the answers are searched and the results are printed
            if cross == "a" {
                search_results = clues::ans_search(search_string, clues_entries.clone());
                for entry in search_results.clone() {
                    println!("{entry}");
                }
                println!();
            }
            //If c, then the clues are searched and the results are printed
            else if cross == "c" {
                search_results = clues::clue_search(search_string, clues_entries.clone());
                for entry in search_results.clone() {
                    println!("{entry}");
                }
                println!();
            }
        }
        //If the idioms database was chosen, it is searched and the results are printed
        if databases.eq("b") {
            search_results = idioms::search(search_string, idiom_entries.clone());
            for entry in search_results.clone() {
                println!("{entry}");
            }
            println!();
        }
        //If the synonyms database was chosen, it is searched and the results are printed
        else if databases.eq("c") {
            search_results = synonyms::search(search_string, syn_entries.clone());
            for entry in search_results.clone() {
                println!("{entry}");
            }
            println!();
        }
        //If the dictionary database is chosen, the dict variable is checked to determine whether the words or definitions should be checked
        else if databases.eq("d") {
            //If d, then the definitions are searched and the results are printed
            if dict == "d" {
                search_results = dict::def_search(&dict_entries, search_string).unwrap();
                for entry in search_results.clone() {
                    println!("{entry}");
                }
                println!();
            }
            //If w, then the definitions are searched and the results are printed
            else if dict == "w" {
                search_results = dict::word_search(&dict_entries, search_string).unwrap();
                for entry in search_results.clone() {
                    println!("{entry}");
                }
                println!();
            }
        }
        //If all the databases are selected, then, for aesthetic purposes, headers are appended to the result vectors in between each database being searched
        //Then, results are printed
        else if databases.eq("e") {
            search_results.push("***".to_string());
            search_results.push("ANSWERS".to_string());
            search_results.push("***".to_string());
            search_results.append(&mut clues::ans_search(search_string, clues_entries.clone()));
            search_results.push("***".to_string());
            search_results.push("CLUES".to_string());
            search_results.push("***".to_string());
            search_results.append(&mut clues::clue_search(search_string, clues_entries.clone()));
            search_results.push("***".to_string());
            search_results.push("IDIOMS".to_string());
            search_results.push("***".to_string());
            search_results.append(&mut idioms::search(search_string, idiom_entries.clone()));
            search_results.push("***".to_string());
            search_results.push("SYNONYMS".to_string());
            search_results.push("***".to_string());
            search_results.append(&mut synonyms::search(search_string, syn_entries.clone()));
            search_results.push("***".to_string());
            search_results.push("DICTIONARY WORDS".to_string());
            search_results.push("***".to_string());
            search_results.append(&mut dict::def_search(&dict_entries, &enter).unwrap());
            search_results.push("***".to_string());
            search_results.push("DICTIONARY DEFINITIONS".to_string());
            search_results.push("***".to_string());
            search_results.append(&mut dict::word_search(&dict_entries, &enter).unwrap());

            for entry in search_results {
                println!("{entry}");
            }
            println!();
        }
        loop{
            println!("Would you like to continue to search the same database?");
        print!("Please type 'y' for yes, 'n' for no, and 'e' to exit: ");
        std::io::stdout().flush().unwrap();
            //The entered string is read in and trimmed
            let mut cont = String::new();
            if std::io::stdin().read_line(&mut cont).expect("Failed to read line") == 0 {
                continue;
            }
            let search_string = &cont.trim().to_lowercase();
            if search_string == "y"{
                break
            }
            else if search_string == "n"{
                let options: Vec<String> = welcome();
                //If a valid input is selected, a line is printed for aesthetic purposes and the search loop is entered
                println!();
                databases = options[0].clone();
                cross = options[1].clone();
                dict = options[2].clone();
                break
            }
            else if search_string == "e"{
                println!("Thank you for using our crossword database. Goodbye!");
                exit(0);
            }
            else{
                println!("This is not a valid option. Please try again.");
                continue
            }
        }
    }
}
