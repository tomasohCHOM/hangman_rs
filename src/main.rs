use std::io::Write;

struct GameData {
    answer: String,
    guesses: Vec<String>,
    hidden: String,
    lives: u8,
}

fn main() {
    display_init_message();
    let mut game_data: GameData = GameData {
        answer: String::from("hello"),
        guesses: vec![],
        hidden: String::new(),
        lives: 6,
    };

    game_data.guesses.push(String::from("a"));

    play_game(&game_data);
}

fn play_game(game_data: &GameData) {
    reveal_location(game_data, &'?');
    display_hangman(game_data);

    while game_data.lives > 0 && game_data.answer != game_data.hidden {
        print!("Enter your guess (must be a letter): ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        println!("The character is {}", input);
        println!("The length of the character is {}", input.len());

        if input.len() - 2 != 1 || !input.chars().nth(0).unwrap().is_alphabetic() {
            println!("Invalid, input must be a letter!");
            continue;
        }

        let letter = input
            .chars()
            .nth(0)
            .expect("No character read")
            .to_lowercase();
        println!("Read this character {}", letter);
    }

    if game_data.answer == game_data.hidden {
        println!("Congrats! You saved the Hangman and won the game.");
    }

    println!(
        "You lose! The Hangman was hanged. The answer was '{}'\nGG Go Next.",
        game_data.answer
    );
}

fn reveal_location(game_data: &GameData, letter_guess: &char) {
    for answer_char in game_data.answer.chars() {
        if answer_char == *letter_guess {
            game_data
                .hidden
                .replace(answer_char, &letter_guess.to_string());
        }
    }
}

fn display_incorrect_guesses(game_data: &GameData) {
    let mut output: String = String::from("[");
    for guess in game_data.guesses.clone() {
        output.push_str(&guess);
    }
    output.push_str("]");
    println!("{}", output);
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
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("I")
        }

        1 => {
            println!(".____.");
            println!("|    |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("I")
        }

        2 => {
            println!(".____.");
            println!("|    |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("I")
        }

        3 => {
            println!(".____.");
            println!("|    |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("I")
        }

        4 => {
            println!(".____.");
            println!("|    |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("I")
        }

        5 => {
            println!(".____.");
            println!("|    |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("I")
        }

        _ => {
            println!(".____.");
            println!("|    |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("I")
        }
    }
}
