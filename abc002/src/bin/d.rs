use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (n, m) = read!((usize, usize));
    let mut xys = vec![vec![false; n]; n];
    for _ in 0..m {
        let (x, y) = read!((usize, usize));
        xys[x - 1][y - 1] = true;
        xys[y - 1][x - 1] = true;
    }

    let ans = (0u64..(1 << n))
        .filter(|&x| {
            let xs = (0..n).filter(|&i| x & (1 << i) > 0);
            xs.clone().all(|i| xs.clone().all(|j| i == j || xys[i][j]))
        })
        .map(|x| x.count_ones())
        .max()
        .unwrap();
    println!("{}", ans);
}
