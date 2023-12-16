use std::io::Write;
mod dictionary;
struct GameData {
    answer: String,
    guesses: Vec<char>,
    hidden: String,
    lives: u8,
}

fn main() {
    display_init_message();

    loop {
        let word: &String = &dictionary::get_random_word();

        let mut game_data: GameData = GameData {
            answer: word.to_string(),
            guesses: vec![],
            hidden: String::from(std::iter::repeat("_").take(word.len()).collect::<String>()),
            lives: 6,
        };
        play_game(&mut game_data);

        print!("Wanna play again? [y/n]: ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        if input.chars().nth(0).unwrap() == 'n' {
            break;
        }
    }

    println!("Goodbye :)");
}

fn play_game(game_data: &mut GameData) {
    display_hangman(game_data);
    println!("The word: {}\n", game_data.hidden);

    while game_data.lives > 0 && game_data.answer != game_data.hidden {
        print!("Enter your guess (must be a letter): ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        if input.len() - 2 != 1 || !input.chars().nth(0).unwrap().is_alphabetic() {
            println!("Invalid, input must be a letter!");
            continue;
        }

        let letter = input
            .chars()
            .nth(0)
            .expect("No character read")
            .to_lowercase()
            .nth(0)
            .unwrap();

        // Check whether the letter was already guessed
        if game_data.hidden.find(letter).is_some() || game_data.guesses.contains(&letter) {
            println!("The letter has already been guessed. Try again.");
        }
        // See if the letter is part of the answer or not
        else if game_data.answer.find(letter).is_some() {
            reveal_letter(game_data, &letter);
        } else {
            game_data.lives -= 1;
            game_data.guesses.push(letter);
            println!("That's incorrect! A part has been added to the Hangman's body.");
        }

        display_hangman(game_data);
        display_incorrect_guesses(game_data);
        println!("The word: {}\n", game_data.hidden);
    }

    if game_data.answer == game_data.hidden {
        println!("Congrats! You saved the Hangman and won the game.");
        return;
    }

    println!(
        "You lose! The Hangman was hanged. The answer was '{}'\nGG Go Next.",
        game_data.answer
    );
}

fn reveal_letter(game_data: &mut GameData, letter_guess: &char) {
    for (index, answer_char) in game_data.answer.chars().enumerate() {
        if answer_char == *letter_guess {
            game_data
                .hidden
                .replace_range(index..index + 1, &letter_guess.to_string());
        }
    }
}

fn display_incorrect_guesses(game_data: &GameData) {
    let mut output: String = String::from("[");

    for (index, itr) in game_data.guesses.iter().enumerate() {
        output.push(*itr);
        if index == game_data.guesses.len() - 1 {
            continue;
        }
        output.push_str(", ");
    }
    output.push_str("]");
    println!("Incorrect guesses: {}", output);
}

fn display_init_message() {
    println!("\nWelcome to Hangman! Guess the secret word before the man is hanged.");
    println!("You do so by typing letters, revealing the letters of the secret word one by one.");
    println!("For every incorrect guess, one part will be added to the man.");
    println!("If his body is completed before you guess the word, you lose! Good Luck!\n");
}

fn display_hangman(game_data: &GameData) {
    match game_data.lives {
        0 => {
            println!(".____.");
            println!("|    |");
            println!("|    O");
            println!("|   /|\\");
            println!("|   / \\");
            println!("|");
            println!("I\n")
        }

        1 => {
            println!(".____.");
            println!("|    |");
            println!("|    O");
            println!("|   /|\\");
            println!("|   /");
            println!("|");
            println!("I\n")
        }

        2 => {
            println!(".____.");
            println!("|    |");
            println!("|    O");
            println!("|   /|\\");
            println!("|");
            println!("|");
            println!("I\n")
        }

        3 => {
            println!(".____.");
            println!("|    |");
            println!("|    O");
            println!("|   /|");
            println!("|");
            println!("|");
            println!("I\n")
        }

        4 => {
            println!(".____.");
            println!("|    |");
            println!("|    O");
            println!("|    |");
            println!("|");
            println!("|");
            println!("I\n")
        }

        5 => {
            println!(".____.");
            println!("|    |");
            println!("|    O");
            println!("|");
            println!("|");
            println!("|");
            println!("I\n")
        }

        _ => {
            println!(".____.");
            println!("|    |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("I\n")
        }
    }
}
