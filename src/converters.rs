use std::io;

pub fn run() {
    loop {
        println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ Выберите доступный конвертер:                       ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 1. Конвертер из °C в °F                             ┃");
        println!("┃ 2. Конвертер из °F в °C                             ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 3. Назад                                            ┃");
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Ошибка проверки строки");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => cels_to_fahr(),
            2 => fahr_to_cels(),
            3 => break,
            _ => (),
        }
    }
}

/// Температурная конвертация из единиц Цельсия в единицы Фаренгейт
fn cels_to_fahr() {
    eprint!("Введите температуру в C:\t");

    let mut c = String::new();

    std::io::stdin()
        .read_line(&mut c)
        .expect("Ошибка чтения строки");

    let c: f32 = c.trim().parse().expect("Введите число!");

    let f: f32 = (c * 1.8) + 32.0;

    return println!("Ваша температура в °F: \t\t{}\n", f.round());
}

/// Температурная конвертация из единиц Фаренгейт в единицы Цельсия
fn fahr_to_cels() {
    eprint!("Введите температуру в F:\t");

    let mut f = String::new();

    std::io::stdin()
        .read_line(&mut f)
        .expect("Ошибка чтения строки");

    let f: f32 = f.trim().parse().expect("Введите число!");

    let c: f32 = (f - 32.0) / 1.8;

    return println!("Ваша температура в °C: \t\t{}\n", c.round());
}