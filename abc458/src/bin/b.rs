use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut x: Vec<(i32, i32)> = Vec::new();

    for i in 1..=h {
        for j in 1..=w {
            let mut count: i32 = 0;

            // 上
            if i > 1 {
                count += 1;
            }
            // 下
            if j < w {
                count += 1;
            }
            // 左
            if j > 1 {
                count += 1;
            }
            // 右
            if i < h {
                count += 1;
            }

            print!("{}", count);

            if j < w {
                print!(" ");
            }
        }

        println!();
    }

}
