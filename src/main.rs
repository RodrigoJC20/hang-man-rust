use std::io;
use std::process::Command;
use rand::Rng;

const EASY_WORDS: [&'static str; 5] = [
    "hello", 
    "world", 
    "house", 
    "green", 
    "queen"
];
const MEDIUM_WORDS: [&'static str; 5] = [
    "aberration",
    "renovation",
    "generation",
    "remarkable",
    "reflection"
];
const HARD_WORDS: [&'static str; 5] = [
    "differentiating",
    "acknowledgeable",
    "bidirectionally",
    "confidentiality",
    "demographically"
];

const TRIES: i32 = 5;

fn main() {
    clear_screen();
    let result = menu();

    let word = match result {
        1 => choose_word(EASY_WORDS),
        2 => choose_word(MEDIUM_WORDS),
        3 => choose_word(HARD_WORDS),
        _ => panic!("Invalid difficulty level"),
    };     

    let winner = start_game(&word);

    println!("Congratulations you are {} {}!!", match winner {
        true => "Winner\n\nYou guessed the word", 
        false => "Loser\n\nTry again! Word ->"
    }, word);
}

fn start_game(word: &String) -> bool {
    let mut winner: bool = false;
    let mut lives = TRIES;
   
    let mut current_word: String = std::iter::repeat("-").take(word.len()).collect();

    clear_screen();

    loop {
        let mut guess = String::new();
       
        loop {
            print_lives(lives);
            print_word(&current_word); 

            guess.clear();

            println!("Make your gues:");

            io::stdin().read_line(&mut guess).expect("Enter a valid character");

            guess = guess.trim().to_string();

            if guess.chars().count() > 1 {
                clear_screen();
                println!("Please enter only one character, {}", guess.len());
                continue;
            }

            break;
        }
        

        let guess_result = query(guess, &word, &mut current_word);

        clear_screen();

        match guess_result {
            true => {
                let is_complete = is_word_complete(&current_word); 

                if is_complete {
                    winner = true;
                    break;
                }

                println!("Good gues!! :D");
                continue;
            },
            false => {
                lives -= 1;

                if lives == 0 {
                    break;
                }

                println!("Wrong, keep trying :(");
            }
        }
    }

    winner
}

fn is_word_complete(word: &String) -> bool {
    let count = word.chars().filter(|&c| c == '-').count();

    count == 0
}

fn query(guess: String, word: &String, current_word: &mut String) -> bool {
    let mut result = false;
    
    for (i, c) in word.chars().enumerate() {
        if c == guess.chars().next().unwrap() {
            current_word.replace_range(i..=i, &guess);
            result = true;
        } 
    }

    result
}

fn print_word(word: &String) {
    for c in word.chars() {
        print!("{c} ");
    }
    println!("\n");
}

fn print_lives(lives: i32) {
    print!("Lives: ");
    
    for _ in 0..TRIES - lives {
        print!("x ");
    }

    for _ in 0..lives {
        print!("o "); 
    }

    println!("\n");
}

fn choose_word(words: [&'static str; 5]) -> String {
    let index = rand::thread_rng().gen_range(0..=4); 

    String::from(words[index])
}

fn menu() -> u32 {
    let mut difficulty = String::new();
    
    loop {
        println!("---------------------");
        println!("Welcome to Hang-Man!");
        println!("---------------------");
        println!("Select the difficulty level:");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard");

        difficulty.clear();

        io::stdin().read_line(&mut difficulty)
            .expect("Failed to read line");

        let difficulty: u32 = match difficulty.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                clear_screen();
                println!("Please enter a valid number");
                continue;
            }
        };

        match difficulty {
            1..=3 => break difficulty,
            _ => {
                clear_screen();
                println!("Please enter a valid difficulty (1-3)");
                continue;
            }
        }
    }    
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        Command::new("sh").arg("-c").arg("clear").status().unwrap();
    }   
}
