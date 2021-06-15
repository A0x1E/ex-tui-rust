mod temperature;

use std::io;

fn main() {
    loop {
        println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ Введите номер опции программы из перечня            ┃");
        println!("┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫");
        println!("┃ 1. Конвертер из °F в °C                             ┃");
        println!("┃ 2. Конвертер из °C в °F                             ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 3. Выйти из программы                               ┃");
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
            1 => temperature::cels_to_fahr(),
            2 => temperature::fahr_to_cels(),
            3 => break,
            _ => (),
        }
    }
}
