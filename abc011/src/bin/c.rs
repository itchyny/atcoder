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

    let mut n = read!(isize);
    let ngs = read!([isize; 3]);
    let mut cnt = 0;
    if !ngs.iter().all(|&ng| n != ng) {
        println!("NO");
        return;
    }
    while n > 0 {
        if let Some(m) = (0.max(n - 3)..n)
            .filter(|&n| ngs.iter().all(|&ng| n != ng))
            .next()
        {
            n = m;
            cnt += 1;
            if cnt > 100 {
                println!("NO");
                return;
            }
        } else {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
