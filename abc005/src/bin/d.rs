use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(usize);
    let ds = read!([[u64; n]; n]);

    let mut sums = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            sums[i][j] = sums[i - 1][j] + sums[i][j - 1] - sums[i - 1][j - 1] + ds[i - 1][j - 1];
        }
    }

    let mut xs = vec![0; n * n + 1];
    for l in 1..=n {
        for m in 1..=n {
            xs[l * m] = xs[l * m].max(
                (0..n - l + 1)
                    .flat_map(|i| {
                        let sums = &sums;
                        (0..n - m + 1).map(move |j| {
                            sums[i + l][j + m] + sums[i][j] - sums[i][j + m] - sums[i + l][j]
                        })
                    })
                    .max()
                    .unwrap(),
            );
        }
    }
    for i in 1..=n * n {
        xs[i] = xs[i].max(xs[i - 1]);
    }

    for _ in 0..read!(usize) {
        println!("{}", xs[read!(usize)]);
    }
}
