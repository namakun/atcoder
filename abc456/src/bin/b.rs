use proconio::input;

fn main() {
    input! {
        a: [[i32; 6]; 3],
    }

    let mut count = 0;

    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                let mut v = vec![a[0][i], a[1][j], a[2][k]];
                v.sort();

                if v == vec![4, 5, 6] {
                    count += 1;
                }
            }
        }
    }

    let ans = count as f64 / 216.0;
    println!("{:.10}", ans);
}
