use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut count: i32 = 0;


    for i in 1..=s.len() {
        for j in 0..=s.len() - i {
            let str: String = s.chars().skip(j).take(i).collect();

            let mid: usize = str.len() / 2;

            if str.len() %2 !=0 && str.chars().collect::<Vec<_>>()[mid] == 'C' {
                count += 1;
            }
        }

    }

    println!("{}", count);
}


// ABCCAから部分文字の抜き出しと真ん中にCのあるもの
// AのみBのみCのみCのみAのみ

// ループとして、iのみ取得とskipループが必要

/**
 *
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    let mut count: i64 = 0;

    for i in 0..n {
        if chars[i] == 'C' {
            let left = i;
            let right = n - 1 - i;

            count += std::cmp::min(left, right) as i64 + 1;
        }
    }

    println!("{}", count);
}
 */
