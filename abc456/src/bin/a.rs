use proconio::input;

fn main() {
    input! {
        x: i32,
    }

    if 3 <= x && x <= 18 {
        println!("Yes");
    } else {
        println!("No");
    }
}
