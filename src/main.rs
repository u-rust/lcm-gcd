use std::io;
fn main() {
    loop {
        println!("Введите а число: ");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Ошибка чтения строки");

        let num1: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Введите b число: ");
        let mut num2 = String::new();
        io::stdin()
           .read_line(&mut num2)
            .expect("Ошибка чтения строки");
        let num2: i32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result1 = lcm(num1, num2);
        let result2 = gcd(num1, num2);
        println!("Наименьшее общее кратное (НОК) - {}, \nНаибольший общий делитель (НОД) - {}.", result1, result2 );
    }
}
fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}