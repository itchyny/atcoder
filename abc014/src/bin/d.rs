use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)));
        (($($tt:tt),+)) => (($(read!($tt)),*));
        (Usize1) => (read!(usize) - 1);
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(usize);
    let mut gs: Vec<Vec<usize>> = vec![vec![]; n];
    for (x, y) in read!([(Usize1, Usize1); n - 1]) {
        gs[x].push(y);
        gs[y].push(x);
    }

    let nlog2 = (n as f64).log2() as usize + 1;
    let mut ds = vec![0; n];
    let mut ps = vec![vec![0; n]; nlog2];
    let mut vs = vec![(0, 0, 0)];
    while let Some((v, p, d)) = vs.pop() {
        ps[0][v] = p;
        ds[v] = d;
        for &w in &gs[v] {
            if w != p {
                vs.push((w, v, d + 1));
            }
        }
    }
    for i in 1..nlog2 {
        for v in 0..n {
            ps[i][v] = ps[i - 1][ps[i - 1][v]];
        }
    }

    for (a, b) in read!([(Usize1, Usize1)]) {
        let (mut a, mut b) = if ds[a] < ds[b] { (a, b) } else { (b, a) };
        let mut d = 0;
        if ds[a] < ds[b] {
            let k = ds[b] - ds[a];
            for l in 0..nlog2 {
                if (k >> l) & 1 != 0 {
                    b = ps[l][b];
                    d += 1 << l;
                }
            }
        }
        if a != b {
            for l in (0..nlog2).rev() {
                if ps[l][a] != ps[l][b] {
                    a = ps[l][a];
                    b = ps[l][b];
                    d += 2 * (1 << l);
                }
            }
            while a != b {
                a = ps[0][a];
                b = ps[0][b];
                d += 2;
            }
        }
        println!("{}", d + 1);
    }
}
