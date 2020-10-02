use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        (Chars) => (read!(String).chars().collect::<Vec<_>>());
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (n, k, xs) = read!((usize, usize, Chars));
    let mut ys = xs.clone();
    for i in 0..n {
        let mut l = i;
        for j in i + 1..n {
            if ys[j] < ys[l]
                && xs
                    .iter()
                    .enumerate()
                    .filter(|&(k, &x)| {
                        x != ys[if j == k {
                            i
                        } else if i == k {
                            j
                        } else {
                            k
                        }]
                    })
                    .count()
                    <= k
            {
                l = j
            }
        }
        if l > i {
            ys.swap(i, l)
        }
    }
    println!("{}", ys.iter().collect::<String>());
}
