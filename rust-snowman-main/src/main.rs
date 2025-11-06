/*
 * Snowman - A Terminal-Based Word Guessing Game
 * 
 * Author: Michael Lopez & Christian Rotondo
 * 
 * Description:
 * Snowman is a twist on the classic Hangman game. Instead of hanging a stick figure,
 * players build a snowman piece by piece with each incorrect guess. The game features
 * a customizable dictionary system where players can add their own words, making it
 * endlessly replayable and educational. Perfect for learning Rust or just having fun
 * in the terminal!
 * 
 * Features:
 * - ASCII art snowman that builds progressively with wrong guesses
 * - Dynamic dictionary system with persistent storage
 * - Input validation and error handling
 * - Case-insensitive gameplay
 * - Support for letters, hyphens, and apostrophes in words
 * - Interactive dictionary management
 */

 use rand::prelude::*;
 use std::fs::File;
 use std::io::{prelude::*, stdin};
 use std::path::Path;
 use std::fs::OpenOptions;
 use std::process::exit;
 
 /// Prints the snowman's hat (appears on 4th wrong guess)
 fn print_hat() {
     println!("    ,===.     ");
     println!("   _|___|_    ");
 }
 
 /// Prints the snowman's head (appears on 3rd wrong guess)
 fn print_head() {
     println!("    /. .\\     ");
     println!("    \\___/     ");
 }
 
 /// Prints the snowman's torso with buttons (appears on 2nd wrong guess)
 fn print_torso() {
     println!("   .'=*=`.    ");
     println!("  Y   *   Y   ");
     println!("   \\  *  /    ");
     println!("    `---'     ");
 }
 
 /// Prints the snowman's bottom section (appears on 1st wrong guess)
 fn print_bottom() {
     println!("  .`   *   '. ");
     println!("  |    *    | ");
     println!("  \\    *    / ");
     println!("__'-.___.'-.__");
 }
 
 /// Prints the complete snowman with right arm (appears on 5th wrong guess)
 fn print_right_arm() {
     println!("       ,===.     ");
     println!("      _|___|_        /__");
     println!("       /. .\\     ,'  ");
     println!("       \\___/    /     ");
     println!("      .'=*=`.  /        ");
     println!("     Y   *   Y          ");
     println!("      \\  *  /          ");
     println!("      .`---'.         ");
     println!("    .`   *   '.");
     println!("    |    *    |");
     println!("    \\    *    /");
     println!("  __.-`._____.'-.__");
 }
 
 /// Prints the complete snowman with both arms (appears on 6th wrong guess - game over)
 fn print_left_arm() {
     println!("           ,===.     ");
     println!("          _|___|_");
     println!("  __/      /. .\\      /__");
     println!("   /`.     \\___/    ,'   ");
     println!("      `.  .'=*=`. .'    ");
     println!("         Y   *   Y         ");
     println!("          \\  *  /          ");
     println!("          .`---'.         ");
     println!("        .`   *   '.");
     println!("        |    *    |");
     println!("        \\    *    /");
     println!("     __.-`._____.'-.__");
 }
 
 /// Reads the dictionary file and returns a vector of words.
 /// Creates the dictionary file if it doesn't exist.
 /// 
 /// # Returns
 /// A `Vec<String>` containing all words from the dictionary, one per line.
 /// 
 /// # Panics
 /// Panics if the file cannot be created, opened, or read.
 fn read_dictionary() -> Vec<String> {
     let path: &Path = Path::new("src/dictionary.txt");
     let mut contents: String = String::new();
 
     // Create the dictionary file if it doesn't exist
     if !path.exists() {
         File::create(path).expect("Couldn't create dictionary file");
     }
 
     // Open and read the dictionary file
     File::open(&path)
         .expect("Couldn't open dictionary")
         .read_to_string(&mut contents)
         .expect("Couldn't read dictionary");
 
     // Split the contents by newlines and collect into a vector
     contents.lines().map(|s: &str| s.to_string()).collect()
 }
 
 /// Appends a new word to the dictionary file.
 /// 
 /// # Arguments
 /// * `word` - The word to add to the dictionary
 /// 
 /// # Errors
 /// Prints an error message to stderr if the write operation fails.
 fn write_dictionary(word: &str) {
     let path: &str = "src/dictionary.txt";
 
     // Open the file in append mode
     let mut file: File = OpenOptions::new()
         .append(true)
         .open(path)
         .expect("Couldn't open dictionary file");
 
     // Write the word followed by a newline
     if let Err(e) = writeln!(file, "{}", word) {
         eprintln!("Error writing to dictionary: {}", e);
     }
 }
 
 /// Prompts the user for input and returns their response.
 /// 
 /// # Arguments
 /// * `prompt` - The message to display to the user
 /// 
 /// # Returns
 /// The user's input as a lowercase, trimmed String.
 /// 
 /// # Panics
 /// Panics if stdout cannot be flushed or input cannot be read.
 fn get_user_input(prompt: &str) -> String {
     print!("{}", prompt);
     std::io::stdout().flush().expect("Failed to flush stdout");
     let mut input: String = String::new();
     stdin().read_line(&mut input).expect("Failed to read input");
     input.trim().to_lowercase()
 }
 
 /// Interactive mode for adding new words to the dictionary.
 /// Allows users to add multiple words separated by spaces.
 /// Type '1' to exit this mode.
 /// 
 /// # Arguments
 /// * `dictionary` - Reference to the current dictionary for validation
 fn add_new_words_to_dictionary(dictionary: &Vec<String>) {
     println!("Enter new valid words to be added to the dictionary, separated by spaces, or press 1 to exit");
 
     let mut exit: bool = false;
 
     while !exit {
         let input: String = get_user_input("Enter new words: ");
 
         // Check if user wants to exit
         if input.trim() == "1" {
             exit = true;
             println!("Exiting...");
         } else {
             // Split input into individual words
             let words: Vec<&str> = input.split(" ").collect();
 
             // Validate and add each word
             for word in words {
                 match validate_new_word(word, dictionary) {
                     Ok(_) => {
                         write_dictionary(word);
                         println!("Added {} to the dictionary!", word);
                     }
                     Err(_) => {
                         println!("{} already in the dictionary!", word);
                     }
                 }
             }
         }
         println!("");
         println!("Enter new valid words to be added to the dictionary, separated by spaces, or press 1 to exit");
     }
 }
 
 /// Validates a user's letter guess during gameplay.
 /// 
 /// # Arguments
 /// * `guess` - The user's input to validate
 /// * `guessed_letters` - Vector of previously guessed letters
 /// 
 /// # Returns
 /// * `Ok(())` if the guess is valid
 /// * `Err(&str)` with an error message if invalid
 /// 
 /// # Validation Rules
 /// - Must be exactly one character
 /// - Must be alphabetic or a hyphen/apostrophe
 /// - Cannot be a previously guessed letter
 fn validate_guess<'a>(guess: &'a str, guessed_letters: &'a Vec<String>) -> Result<(), &'a str> {
     if guess.chars().count() != 1 {
         return Err("Please enter a single letter!");
     }
     if !guess.chars().all(|c: char| c.is_alphabetic() || c == '-' || c == '\'') {
         return Err("Please enter a valid letter!");
     }
     if guessed_letters.contains(&guess.to_string()) {
         return Err("You already guessed that letter!");
     }
     Ok(())
 }
 
 /// Validates a new word before adding it to the dictionary.
 /// 
 /// # Arguments
 /// * `word` - The word to validate
 /// * `dictionary` - Reference to the current dictionary
 /// 
 /// # Returns
 /// * `Ok(())` if the word is valid
 /// * `Err(&str)` with an error message if invalid
 /// 
 /// # Validation Rules
 /// - Must be at least 2 characters long
 /// - Can only contain letters, hyphens, or apostrophes
 /// - Cannot already exist in the dictionary
 fn validate_new_word<'a>(word: &'a str, dictionary: &'a Vec<String>) -> Result<(), &'a str> {
     if word.chars().count() < 2 {
         return Err("Please enter a word with at least 2 characters!");
     }
     if word.chars().any(|c: char| !c.is_alphabetic() && c != '-' && c != '\'') {
         return Err("Please enter a valid word!");
     }
     if dictionary.contains(&word.to_string()) {
         return Err("The word is already in the dictionary!");
     }
     Ok(())
 }
 
 /// Main game loop - handles the Snowman game logic.
 /// 
 /// # Game Flow
 /// 1. Loads dictionary from file
 /// 2. Selects a random word
 /// 3. Player guesses letters one at a time
 /// 4. Snowman builds up with each wrong guess (6 tries total)
 /// 5. Player wins by guessing all letters or loses after 6 wrong guesses
 /// 6. Offers option to add new words to dictionary
 fn main() {
     // Read the dictionary file
     let dictionary: Vec<String> = read_dictionary();
 
     // Check if the dictionary is empty - can't play without words!
     if dictionary.is_empty() {
         println!("The dictionary is empty. Please add words to play the game.");
         add_new_words_to_dictionary(&dictionary);
         return;
     }
 
     // Choose a random word from the dictionary for this game
     let word: &String = dictionary.choose(&mut rand::rng()).unwrap();
 
     println!("Welcome to Snowman!");
     
     // Display initial underscores for each letter in the word
     word.chars().for_each(|_| {
         print!("_ ");
     });
 
     println!("");
 
     // Initialize game state variables
     let mut guessed: bool = false;
     let mut guessed_letters: Vec<String> = Vec::new();
     let mut tries: i32 = 0;
 
     // Main game loop - continues until player wins or loses
     while !guessed {
         println!("");
         let guess: String = get_user_input("Guess a letter: ");
         
         // Validate the guess before processing
         match validate_guess(&guess, &guessed_letters) {
             Ok(_) => (),
             Err(e) => {
                 println!("{}", e);
                 continue;
             }
         }
 
         // Add valid guess to the list
         guessed_letters.push(guess.clone());
 
         let mut display_word: String = String::new();
 
         // Build display string showing guessed letters and blanks
         word.chars().for_each(|c: char| {
             if guessed_letters.contains(&c.to_string()) {
                 display_word.push_str(&format!("{} ", c));
             } else {
                 display_word.push_str("_ ");
             }
         });
 
         // Check if the entire word has been guessed (no underscores left)
         if display_word.chars().all(|c: char| c != '_') {
             guessed = true;
         }
         
         println!("");
         println!("{}", display_word);
         println!("");
 
         // Handle wrong guesses - build the snowman progressively
         if !word.contains(&guess) {
             tries += 1;
             if tries == 1 {
                 print_bottom();
             } else if tries == 2 {
                 print_torso();
                 print_bottom();
             } else if tries == 3 {
                 print_head();
                 print_torso();
                 print_bottom();
             } else if tries == 4 {
                 print_hat();
                 print_head();
                 print_torso();
                 print_bottom();
             } else if tries == 5 {
                 print_right_arm();
                 
             } else if tries == 6 {
                 print_left_arm();
             }
             println!("");
             println!("Wrong guess! You have {} tries left", 6 - tries);
         }
 
         // Sort guessed letters alphabetically for better display
         guessed_letters.sort();
 
         // Display all guessed letters so far
         println!("");
         println!("guessed Letters: {}", guessed_letters.join(" "));
 
         // Check for game over conditions
         if tries == 6 {
             println!("You lost. The word was {}", word);
             break;
         }
         if guessed {
             println!("You won! The word was {}", word);
             break;
         }
     }
 
     println!("");
 
     // Ask the user if they would like to add new words to the dictionary
     loop {
         let input: String = get_user_input("Would you like to add new words? (y/n): ");
 
         if input == "y" {
             break;
         } else if input == "n" {
             exit(0);
         } else {
             println!("Please enter y or n!");
         }
     }
 
     // Enter dictionary management mode
     add_new_words_to_dictionary(&dictionary);
     println!("");
     println!("Thank you for playing!");
 }