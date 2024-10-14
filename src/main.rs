use std::io;

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    println!("Введіть перше число:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Не вдалося прочитати рядок");
    let num1: u32 = input1.trim().parse().expect("Невірний формат числа");

    println!("Введіть друге число:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Не вдалося прочитати рядок");
    let num2: u32 = input2.trim().parse().expect("Невірний формат числа");

    let result = gcd(num1, num2);
    println!("GCD чисел {} і {} дорівнює: {}", num1, num2, result);
}
