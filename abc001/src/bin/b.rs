use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(u64);
    if n < 100 {
        println!("00");
    } else if n <= 5000 {
        println!("{:>02}", n / 100);
    } else if n <= 30000 {
        println!("{}", n / 1000 + 50);
    } else if n <= 70000 {
        println!("{}", (n - 30000) / 5000 + 80);
    } else {
        println!("89");
    }
}
