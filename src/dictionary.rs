const FILENAME: &str = "/words/words.txt";

pub fn get_random_word() -> String {
    let f = std::fs::File::open(FILENAME);
    return "holamama".to_string();
}
