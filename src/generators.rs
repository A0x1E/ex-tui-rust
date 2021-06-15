/// Простейший генератор чисел фибоначчи (32-битный)
pub fn run() {
    loop {
        println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ Выберите доступный генератор:                       ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 1. Числа Фибоначчи                                  ┃");
        println!("┠─────────────────────────────────────────────────────┨");
        println!("┃ 2. Назад                                            ┃");
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

        let mut choice = String::new();

        std::io::stdin().read_line(&mut choice).expect("Ошибка проверки строки");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => fibonacci(),
            3 => break,
            _ => (),
        }
    }
}

fn fibonacci() {
    eprint!("Введите исходное число (<=42): ");

    let mut num = String::new();

    std::io::stdin()
        .read_line(&mut num)
        .expect("Ошибка чтения строки");

    
    let num: usize = num
        .trim()
        .parse()
        .expect("Введите целое число");

    println!("Конечное число:                {}\n", fibonacci_algorithm(num))
}

fn fibonacci_algorithm(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci_algorithm(n - 1) + fibonacci_algorithm(n - 2),
    }
}