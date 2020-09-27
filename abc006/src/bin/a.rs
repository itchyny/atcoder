use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(usize);
    if n % 3 == 0 || n.to_string().chars().any(|x| x == '3') {
        println!("YES");
    } else {
        println!("NO");
    }
}
