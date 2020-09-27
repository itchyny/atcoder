use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let t = read!(u64);
    let n = read!(usize);
    let xs = read!([u64; n]);
    let m = read!(usize);
    let ys = read!([u64; m]);
    let mut ix = 0;
    for &y in ys.iter() {
        while ix < n && xs[ix] + t < y {
            ix += 1;
        }
        if ix == n || y < xs[ix] {
            println!("no");
            return;
        }
        ix += 1;
    }
    println!("yes");
}
