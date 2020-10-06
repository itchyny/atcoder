use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)));
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (n, g, e) = read!((usize, usize, usize));
    let mut gs = vec![vec![0; n + 1]; n + 1];
    for p in read!([usize; g]) {
        gs[p][n] = 1;
    }
    for (a, b) in read!([(usize, usize); e]) {
        gs[a][b] = 1;
        gs[b][a] = 1;
    }
    println!("{}", max_flow(0, n, &mut gs));
}

fn max_flow(s: usize, e: usize, gs: &mut Vec<Vec<u64>>) -> u64 {
    let mut flow = 0;
    loop {
        let mut visited = vec![false; gs.len()];
        let f = dfs(s, e, std::u64::MAX, gs, &mut visited);
        if f == 0 {
            return flow;
        }
        flow += f;
    }
}

fn dfs(s: usize, e: usize, f: u64, gs: &mut Vec<Vec<u64>>, visited: &mut Vec<bool>) -> u64 {
    if s == e {
        return f;
    }
    visited[s] = true;
    for i in 0..gs.len() {
        if gs[s][i] > 0 && !visited[i] {
            let d = dfs(i, e, f.min(gs[s][i]), gs, visited);
            if d > 0 {
                gs[s][i] -= d;
                gs[i][s] += d;
                return d;
            }
        }
    }
    0
}
