/// Температурная конвертация из единиц Цельсия в единицы Фаренгейт
pub fn cels_to_fahr() {
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
pub fn fahr_to_cels() {
    eprint!("Введите температуру в F:\t");

    let mut f = String::new();

    std::io::stdin()
        .read_line(&mut f)
        .expect("Ошибка чтения строки");

    let f: f32 = f.trim().parse().expect("Введите число!");

    let c: f32 = (f - 32.0) / 1.8;

    return println!("Ваша температура в °C: \t\t{}\n", c.round());
}
