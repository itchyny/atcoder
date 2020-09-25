use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (xs, ys) = read!((String, String));
    let cs = "atcoder@".chars().collect::<Vec<_>>();
    let ans = xs
        .chars()
        .zip(ys.chars())
        .all(|(x, y)| x == y || x == '@' && cs.contains(&y) || y == '@' && cs.contains(&x));
    if ans {
        println!("You can win");
    } else {
        println!("You will lose");
    }
}
