use std::collections::HashMap;
use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        (Bytes) => (read!(String).into_bytes());
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (_, n, k) = read!((usize, usize, u8));
    let mut ss = read!([Bytes; n]);

    let mut xycs = vec![];
    let mut s = b'1';
    loop {
        let t = ss[n / 2][n / 2];
        if s == t {
            s = (s - b'0' + 1) % k + b'0';
            continue;
        }
        xycs.push((n / 2, n / 2, s));
        let mut xys = vec![((n / 2) as i64, (n / 2) as i64)];
        let mut cs = HashMap::new();
        while let Some((x, y)) = xys.pop() {
            ss[x as usize][y as usize] = s;
            for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if 0 <= x + dx && x + dx < n as i64 && 0 <= y + dy && y + dy < n as i64 {
                    let u = ss[(x + dx) as usize][(y + dy) as usize];
                    if u == t {
                        xys.push((x + dx, y + dy));
                    } else if u != s {
                        *cs.entry(u).or_insert(0) += 1;
                    }
                }
            }
        }
        if (0..n).all(|i| (0..n).all(|j| ss[i][j] == s)) {
            break;
        }
        s = *cs.iter().max_by_key(|x| (x.1, x.0)).unwrap().0;
    }

    println!("{}", xycs.len());
    for (x, y, t) in xycs {
        println!("{} {} {}", x + 1, y + 1, t - b'0');
    }
}
