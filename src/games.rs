use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    loop {
        println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ Выберите доступную игру:                            ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 1. «Угадай число»                                   ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 2. Назад                                            ┃");
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Ошибка проверки строки");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => guessing_game(),
            2 => break,
            _ => (),
        }
    }
}

fn guessing_game() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Пожалуйста, введите свою догадку.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не получилось прочитать строку");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Вы загадали: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число!"),
            Ordering::Greater => println!("Слишком большое число!"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            }
        }
    }
}