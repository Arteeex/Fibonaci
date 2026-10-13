use std::io;


fn main() {
    
    let mut n = String::new();
    let mut a = 0;
    let mut b = 1;


    println!("Введите порядковый номер числа Фибоначи");

    io::stdin()
        .read_line(&mut n)
        .expect("error");

    let n: i32 = n
        .trim()
        .parse()
        .expect("error");

    for _ in 2..=n{
        let c = a + b;
        a = b;
        b = c;
    }

println!("Число фибоначчи для {} равно {}", n, b);

    

}
