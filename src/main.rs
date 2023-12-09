struct GameData {
    answer: String,
    guesses: Vec<String>,
    lives: u8,
}

fn main() {
    let mut game_data: GameData = GameData {
        answer: String::from("Hello World!"),
        guesses: vec![],
        lives: 6,
    };

    println!("{}", game_data.answer);
    game_data
        .guesses
        .push(String::from("This is Hangman in Rust!"));

    for guess in game_data.guesses.clone() {
        println!("{}", guess);
    }

    println!("{}", game_data.lives);
    print_hangman(&game_data);
}

fn print_hangman(game_data: &GameData) {
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
