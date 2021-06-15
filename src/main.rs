mod generators;
mod games;
mod converters;

use std::io;

fn main() {
    loop {
        println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ Введите номер опции программы из перечня            ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ TUI - терминальные программы                        ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 1. Игра: «Угадай число»                             ┃");
        println!("┃ 2. Конвертеры                                       ┃");
        println!("┃ 3. Генераторы                                       ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 4. Завершить                                        ┃");
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Ошибка проверки строки");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => games::run(),
            2 => converters::run(),
            3 => generators::run(),
            4 => break,
            _ => (),
        }
    }
}
